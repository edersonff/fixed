use encoding_rs::WINDOWS_1251;

use scraper::Html;

use scraper::Selector;

use serde::Serialize;

use std::io::Read;

use std::io::Write;

use std::path::Path;

const HOME_FIXTURE: &[u8] = include_bytes!("../tests/fixtures/home.html");

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0 Safari/537.36 FIXED/0.1";

#[derive(Serialize)]

#[serde(rename_all = "camelCase")]

pub struct GameEntry {

    pub title: String,

    pub page_url: String,

    pub poster_url: String,

    pub category: String,

    pub published_at: String,

    pub views: u64,

    pub comments: u64,

}

#[derive(Serialize)]

#[serde(rename_all = "camelCase")]

pub struct DownloadLane {

    pub kind: String,

    pub url: String,

}

#[derive(Serialize)]

#[serde(rename_all = "camelCase")]

pub struct GameDetail {

    pub title: String,

    pub build: String,

    pub steam_ext_url: String,

    pub lanes: Vec<DownloadLane>,

    pub mentions_fix_repair: bool,

    pub video_id: String,

}

pub fn decode_cp1251(bytes: &[u8]) -> String {

    let (text, _, had_errors) = WINDOWS_1251.decode(bytes);

    if had_errors {

        return String::from_utf8_lossy(bytes).into_owned();

    }

    text.into_owned()

}

fn category_from_url(url: &str) -> String {

    let mut parts = url.split('/').skip_while(|part| *part != "games");

    parts.next();

    let category = parts.next().unwrap_or("unknown");

    category.trim_end_matches(".html").to_string()

}

fn last_number(text: &str) -> u64 {

    text.split_whitespace()

        .filter(|token| !token.is_empty() && token.chars().all(|c| c.is_ascii_digit()))

        .next_back()

        .and_then(|token| token.parse().ok())

        .unwrap_or(0)

}

fn first_build(html: &str) -> String {

    let after = match html.find("Build ") {

        Some(index) => &html[index + 6..],

        None => return String::new(),

    };

    let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();

    digits

}

fn lane_kind(url: &str) -> Option<&'static str> {

    if url.contains("hosters.online-fix.me") {

        Some("hosters")

    } else if url.contains("drive.online-fix.me") {

        Some("drive")

    } else if url.contains(":2053/uploads/") {

        Some("direct")

    } else if url.contains(":2053/torrents/") {

        Some("torrent")

    } else {

        None

    }

}

fn ext_lane_kind(text: &str) -> Option<&'static str> {

    if text.contains("торрент") {

        Some("torrent")

    } else if text.contains("mega") {

        Some("mega")

    } else if text.contains("yandex") {

        Some("yandex")

    } else if text.contains("google") {

        Some("google-drive")

    } else if text.contains("скачать") {

        Some("mirror")

    } else {

        None

    }

}

pub fn parse_home(html: &str) -> Vec<GameEntry> {

    let document = Html::parse_document(html);

    let article = Selector::parse("article.news").expect("article selector");

    let big_link = Selector::parse("a.big-link").expect("big-link selector");

    let poster = Selector::parse("div.image img").expect("poster selector");

    let published = Selector::parse("time").expect("time selector");

    let info = Selector::parse("span.info-date").expect("info selector");

    let comment_link = Selector::parse("a[href*='#comment']").expect("comment selector");

    let title = Selector::parse("h2.title").expect("title selector");

    document.select(&article)

        .map(|node| {

            let page_url = node.select(&big_link).next()

                .and_then(|el| el.value().attr("href"))

                .unwrap_or("")

                .to_string();

            let poster_url = node.select(&poster).next()

                .and_then(|el| el.value().attr("data-src").or_else(|| el.value().attr("src")))

                .unwrap_or("")

                .to_string();

            let published_at = node.select(&published).next()

                .and_then(|el| el.value().attr("datetime"))

                .unwrap_or("")

                .to_string();

            let comments = node.select(&comment_link).next()

                .map(|el| el.text().collect::<String>().trim().parse().unwrap_or(0))

                .unwrap_or(0);

            let info_text = node.select(&info).next()

                .map(|el| el.text().collect::<String>())

                .unwrap_or_default();

            let without_comments = info_text

                .split_whitespace()

                .filter(|token| !token.chars().all(|c| c.is_ascii_digit()) || *token != comments.to_string())

                .collect::<Vec<_>>()

                .join(" ");

            GameEntry {

                title: node.select(&title).next().map(|el| el.text().collect::<String>().trim().replace(" по сети", "").trim().to_string()).unwrap_or_default(),

                category: category_from_url(&page_url),

                views: last_number(&without_comments),

                comments,

                page_url,

                poster_url,

                published_at,

            }

        })

        .collect()

}

pub fn parse_detail(html: &str) -> GameDetail {

    let document = Html::parse_document(html);

    let heading = Selector::parse("h1").expect("h1 selector");

    let steam_anchor = Selector::parse("a[title='store.steampowered.com']").expect("steam selector");

    let link = Selector::parse("a[href]").expect("link selector");

    let mut lanes: Vec<DownloadLane> = Vec::new();

    for node in document.select(&link) {

        let url = node.value().attr("href").unwrap_or("");

        let kind = lane_kind(url).map(String::from).or_else(|| {

            if url.starts_with("https://online-fix.me/ext/") {

                let text = node.text().collect::<String>().to_lowercase();

                ext_lane_kind(&text).map(|kind| kind.to_string())

            } else {

                None

            }

        });

        if let Some(kind) = kind {

            if !lanes.iter().any(|lane| lane.kind == kind) {

                lanes.push(DownloadLane { kind, url: url.to_string() });

            }

        }

    }

    GameDetail {

        title: document.select(&heading).next().map(|el| el.text().collect::<String>().trim().to_string()).unwrap_or_default(),

        build: first_build(html),

        steam_ext_url: document.select(&steam_anchor).next()

            .and_then(|el| el.value().attr("href"))

            .unwrap_or("")

            .to_string(),

        lanes,

        mentions_fix_repair: html.contains("Fix Repair"),

        video_id: find_video_id(html),

    }

}

fn find_video_id(html: &str) -> String {

    for marker in ["watch?v=", "ytimg.com/vi/", "youtube.com/embed/"] {

        if let Some(start) = html.find(marker) {

            let from = start + marker.len();

            let candidate: String = html[from..]

                .chars()

                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')

                .collect();

            if candidate.len() == 11 {

                return candidate;

            }

        }

    }

    String::new()

}

#[derive(Serialize)]

#[serde(rename_all = "camelCase")]

pub struct GamesPage {

    pub source: String,

    pub games: Vec<GameEntry>,

}

pub fn home_games(page: u32) -> GamesPage {

    let url = if page <= 1 {

        String::from("https://online-fix.me/")

    } else {

        format!("https://online-fix.me/page/{}/", page)

    };

    match fetch_cp1251(&url) {

        Ok(bytes) => GamesPage { source: String::from("live"), games: parse_home(&decode_cp1251(&bytes)) },

        Err(_) => GamesPage { source: String::from("fixture"), games: parse_home(&decode_cp1251(HOME_FIXTURE)) },

    }

}

pub fn search_games(query: &str) -> GamesPage {

    let url = format!("https://online-fix.me/index.php?do=search&subaction=search&story={}", query.replace(' ', "+"));

    match fetch_cp1251(&url) {

        Ok(bytes) => GamesPage { source: String::from("live"), games: parse_search(&decode_cp1251(&bytes)) },

        Err(_) => GamesPage { source: String::from("error"), games: Vec::new() },

    }

}

fn parse_ru_date(text: &str) -> String {

    const MONTHS: [(&str, &str); 12] = [

        ("января", "01"),

        ("февраля", "02"),

        ("марта", "03"),

        ("апреля", "04"),

        ("мая", "05"),

        ("июня", "06"),

        ("июля", "07"),

        ("августа", "08"),

        ("сентября", "09"),

        ("октября", "10"),

        ("ноября", "11"),

        ("декабря", "12"),

    ];

    let parts: Vec<&str> = text.trim().split_whitespace().collect();

    if parts.len() >= 3 {

        let day = parts[0].trim_end_matches(',');

        let year = parts[2].trim_end_matches(',');

        if let Some((_, month)) = MONTHS.iter().find(|(name, _)| *name == parts[1]) {

            return format!("{}-{}-{:0>2}", year, month, day);

        }

    }

    String::new()

}

pub fn parse_search(html: &str) -> Vec<GameEntry> {

    let document = Html::parse_document(html);

    let result_link = Selector::parse("a[title][href*='/games/']").expect("search result selector");

    let poster = Selector::parse("img[src]").expect("search poster selector");

    let date_line = Selector::parse("p.text-gray").expect("search date selector");

    document.select(&result_link)

        .map(|node| {

            let href = node.value().attr("href").unwrap_or("");

            let page_url = if href.starts_with("http") {

                href.to_string()

            } else {

                format!("https://online-fix.me{}", href)

            };

            let poster_url = node.select(&poster).next()

                .and_then(|el| el.value().attr("src"))

                .unwrap_or("")

                .to_string();

            let title = node.value().attr("title").unwrap_or("").trim().replace(" по сети", "").trim().to_string();

            let ru_date = node.select(&date_line).next()

                .map(|el| el.text().collect::<String>())

                .unwrap_or_default();

            GameEntry {

                title,

                category: category_from_url(&page_url),

                views: 0,

                comments: 0,

                published_at: parse_ru_date(&ru_date),

                page_url,

                poster_url,

            }

        })

        .collect()

}

fn fetch_cp1251(url: &str) -> Result<Vec<u8>, String> {

    let response = ureq::get(url)

        .set("User-Agent", USER_AGENT)

        .timeout(std::time::Duration::from_secs(12))

        .call()

        .map_err(|error| error.to_string())?;

    let mut bytes = Vec::new();

    response.into_reader().take(4_000_000).read_to_end(&mut bytes).map_err(|error| error.to_string())?;

    Ok(bytes)

}

pub fn fetch_detail(page_url: &str) -> GameDetail {

    match fetch_cp1251(page_url) {

        Ok(bytes) => parse_detail(&decode_cp1251(&bytes)),

        Err(error) => {

            eprintln!("fetch_detail failed for {}: {}", page_url, error);

            GameDetail { title: String::new(), build: String::new(), steam_ext_url: String::new(), lanes: Vec::new(), mentions_fix_repair: false, video_id: String::new() }

        },

    }

}

fn absolutize(base: &str, url: &str) -> String {

    if url.starts_with("http://") || url.starts_with("https://") {

        url.to_string()

    } else if let Some(stripped) = url.strip_prefix("./") {

        format!("{}/{}", base.trim_end_matches('/'), stripped)

    } else if url.starts_with('/') {

        let origin = base.split('/').take(3).collect::<Vec<_>>().join("/");

        format!("{}{}", origin, url)

    } else {

        format!("{}/{}", base.trim_end_matches('/'), url)

    }

}

pub fn discover_parts(lane_url: &str) -> Vec<String> {

    let bytes = match fetch_cp1251(lane_url) {

        Ok(bytes) => bytes,

        Err(_) => return Vec::new(),

    };

    let html = decode_cp1251(&bytes);

    let document = Html::parse_document(&html);

    let link = Selector::parse("a[href]").expect("link selector");

    document.select(&link)

        .filter_map(|node| {

            let url = node.value().attr("href").unwrap_or("");

            let lowered = url.to_lowercase();

            if lowered.ends_with(".rar") || lowered.ends_with(".zip") || lowered.ends_with(".7z") {

                Some(absolutize(lane_url, url))

            } else {

                None

            }

        })

        .collect()

}

#[derive(Serialize)]

#[serde(rename_all = "camelCase")]

pub struct LaneProbe {

    pub parts: Vec<String>,

    pub first_part_bytes: u64,

}

pub fn probe_lane(lane_url: &str) -> LaneProbe {

    let parts = discover_parts(lane_url);

    let first_part_bytes = parts.first()

        .map(|url| remote_size(url))

        .unwrap_or(0);

    LaneProbe { parts, first_part_bytes }

}

fn remote_size(url: &str) -> u64 {

    let head = ureq::request("HEAD", url)

        .set("User-Agent", USER_AGENT)

        .timeout(std::time::Duration::from_secs(12))

        .call()

        .ok()

        .and_then(|response| response.header("Content-Length").and_then(|value| value.parse().ok()));

    if let Some(size) = head {

        return size;

    }

    ureq::get(url)

        .set("User-Agent", USER_AGENT)

        .set("Range", "bytes=0-0")

        .timeout(std::time::Duration::from_secs(12))

        .call()

        .ok()

        .and_then(|response| response.header("Content-Range").and_then(|range| {

            range.split('/').next_back().and_then(|total| total.trim().parse().ok())

        }))

        .unwrap_or(0)

}

pub fn download_partial(url: &str, dest: &str, limit: u64) -> Result<u64, String> {

    let response = ureq::get(url)

        .set("User-Agent", USER_AGENT)

        .timeout(std::time::Duration::from_secs(30))

        .call()

        .map_err(|error| format!("request {}: {}", url, error))?;

    let mut reader = response.into_reader().take(limit);

    let mut file = std::fs::File::create(dest).map_err(|error| format!("create {}: {}", dest, error))?;

    let mut buffer = [0u8; 65536];

    let mut written: u64 = 0;

    loop {

        let read = reader.read(&mut buffer).map_err(|error| format!("read: {}", error))?;

        if read == 0 {

            break;

        }

        file.write_all(&buffer[..read]).map_err(|error| format!("write: {}", error))?;

        written += read as u64;

    }

    Ok(written)

}

pub fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {

    let response = ureq::get(url)

        .set("User-Agent", USER_AGENT)

        .timeout(std::time::Duration::from_secs(30))

        .call()

        .map_err(|error| format!("request {}: {}", url, error))?;

    let mut bytes = Vec::new();

    response.into_reader().take(8_000_000).read_to_end(&mut bytes).map_err(|error| error.to_string())?;

    Ok(bytes)

}

pub fn torrent_file_url(lane_url: &str) -> Option<String> {

    let bytes = fetch_cp1251(lane_url).ok()?;

    let html = decode_cp1251(&bytes);

    let document = Html::parse_document(&html);

    let link = Selector::parse("a[href]").expect("link selector");

    document.select(&link)

        .filter_map(|node| {

            let url = node.value().attr("href").unwrap_or("");

            if url.to_lowercase().ends_with(".torrent") {

                Some(absolutize(lane_url, url))

            } else {

                None

            }

        })

        .next()

}

pub const RAR_PASSWORD: &str = "online-fix.me";

pub const ONLINE_FIX_LAUNCH_OPTIONS: &str = "WINEDLLOVERRIDES=\"winhttp=n,b;WINMM=n,b;SteamOverlay64=n,b;steam_api64=n,b\" %command%";

pub fn shortcut_appid(exe_path: &str, app_name: &str) -> u32 {

    let mut hash: u32 = 2166136261;

    for byte in exe_path.bytes().chain(app_name.bytes()) {

        hash ^= byte as u32;

        hash = hash.wrapping_mul(16777619);

    }

    hash

}

fn vdf_string(key: &str, value: &str) -> Vec<u8> {

    let mut out = vec![1u8];

    out.extend_from_slice(key.as_bytes());

    out.push(0);

    out.extend_from_slice(value.as_bytes());

    out.push(0);

    out

}

fn vdf_int(key: &str, value: u32) -> Vec<u8> {

    let mut out = vec![2u8];

    out.extend_from_slice(key.as_bytes());

    out.push(0);

    out.extend_from_slice(&value.to_le_bytes());

    out

}

fn entry_index_at(data: &[u8], marker_start: usize) -> Option<u32> {

    let digits_end = marker_start.checked_sub(1)?;

    if data.get(digits_end)? != &0u8 {

        return None;

    }

    let mut start = digits_end;

    while start > 0 && data[start - 1].is_ascii_digit() {

        start -= 1;

    }

    if start == digits_end || start == 0 || data[start - 1] != 0u8 {

        return None;

    }

    std::str::from_utf8(&data[start..digits_end]).ok()?.parse().ok()

}

fn max_entry_index(data: &[u8]) -> u32 {

    let marker: &[u8] = b"\x02appid\x00";

    let mut max: u32 = 0;

    let mut pos = 0;

    while let Some(found) = find_subslice(data, marker, pos) {

        if let Some(index) = entry_index_at(data, found) {

            if index > max {

                max = index;

            }

        }

        pos = found + marker.len();

    }

    max

}

fn find_subslice(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {

    if needle.is_empty() || from >= haystack.len() {

        return None;

    }

    haystack[from..]

        .windows(needle.len())

        .position(|window| window == needle)

        .map(|offset| offset + from)

}

const PLUGIN_META_FILES: [&str; 4] = ["manifest.json", "icon.png", "readme.md", "changelog.md"];

pub fn install_plugin(archive_path: &str, game_dir: &str) -> Result<u32, String> {

    let data = std::fs::read(archive_path).map_err(|error| format!("read {}: {}", archive_path, error))?;

    if data.starts_with(b"MZ") {

        let name = archive_path.rsplit('/').next().unwrap_or("plugin.dll");

        let plugins_dir = format!("{}/BepInEx/plugins", game_dir);

        std::fs::create_dir_all(&plugins_dir).map_err(|error| format!("create {}: {}", plugins_dir, error))?;

        std::fs::write(format!("{}/{}", plugins_dir, name), &data).map_err(|error| format!("write dll: {}", error))?;

        return Ok(1);

    }

    if data.starts_with(b"Rar!") {

        let temp_dir = format!("{}/.fixed-plugin-tmp", game_dir);

        extract_archive(archive_path, &temp_dir)?;

        let count = place_extracted(&temp_dir, game_dir)?;

        std::fs::remove_dir_all(&temp_dir).map_err(|error| format!("cleanup: {}", error))?;

        return Ok(count);

    }

    let reader = std::io::Cursor::new(data);

    let mut archive = zip::ZipArchive::new(reader).map_err(|error| format!("open zip: {}", error))?;

    let mut count: u32 = 0;

    for index in 0..archive.len() {

        let mut file = archive.by_index(index).map_err(|error| format!("zip entry: {}", error))?;

        if file.is_dir() {

            continue;

        }

        let name = file.name().to_string();

        let base = name.rsplit('/').next().unwrap_or("").to_lowercase();

        if PLUGIN_META_FILES.contains(&base.as_str()) {

            continue;

        }

        let mut bytes: Vec<u8> = Vec::new();

        std::io::Read::read_to_end(&mut file, &mut bytes).map_err(|error| format!("read {}: {}", name, error))?;

        let root_dll = !name.contains('/') && name.to_lowercase().ends_with(".dll");

        let dest = if root_dll {

            format!("{}/BepInEx/plugins/{}", game_dir, name)

        } else {

            format!("{}/{}", game_dir, name)

        };

        if let Some(parent) = std::path::Path::new(&dest).parent() {

            std::fs::create_dir_all(parent).map_err(|error| format!("create dirs: {}", error))?;

        }

        std::fs::write(&dest, &bytes).map_err(|error| format!("write {}: {}", dest, error))?;

        count += 1;

    }

    Ok(count)

}

pub fn apply_fix_repair(title_folder: &str, game_dir: &str) -> Result<u32, String> {

    let repair_dir = format!("{}/Fix Repair", title_folder);

    let entries = std::fs::read_dir(&repair_dir).map_err(|error| format!("read {}: {}", repair_dir, error))?;

    let mut applied: u32 = 0;

    for entry in entries.flatten() {

        let path = entry.path();

        let is_rar = path

            .extension()

            .map(|ext| ext.to_string_lossy().to_lowercase() == "rar")

            .unwrap_or(false);

        if !is_rar {

            continue;

        }

        let rar_path = path.to_string_lossy().to_string();

        extract_archive(&rar_path, game_dir)?;

        applied += 1;

    }

    Ok(applied)

}

fn place_extracted(source: &str, game_dir: &str) -> Result<u32, String> {

    let mut count: u32 = 0;

    let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(source)

        .map_err(|error| format!("read {}: {}", source, error))?

        .flatten()

        .map(|entry| entry.path())

        .collect();

    while let Some(path) = paths.pop() {

        if path.is_dir() {

            let children = std::fs::read_dir(&path).map_err(|error| format!("read dir: {}", error))?;

            for child in children.flatten() {

                paths.push(child.path());

            }

            continue;

        }

        let relative = path

            .strip_prefix(source)

            .map_err(|error| format!("relative: {}", error))?

            .to_string_lossy()

            .to_string();

        let base = relative.rsplit('/').next().unwrap_or("").to_lowercase();

        if PLUGIN_META_FILES.contains(&base.as_str()) {

            continue;

        }

        let root_dll = !relative.contains('/') && relative.to_lowercase().ends_with(".dll");

        let dest = if root_dll {

            format!("{}/BepInEx/plugins/{}", game_dir, relative)

        } else {

            format!("{}/{}", game_dir, relative)

        };

        if let Some(parent) = std::path::Path::new(&dest).parent() {

            std::fs::create_dir_all(parent).map_err(|error| format!("create dirs: {}", error))?;

        }

        std::fs::copy(&path, &dest).map_err(|error| format!("copy {}: {}", dest, error))?;

        count += 1;

    }

    Ok(count)

}

pub fn find_game_exe(folder: &str) -> Option<String> {

    const EXCLUSIONS: [&str; 9] = [

        "unitycrashhandler",

        "vc_redist",

        "vcredist",

        "dxsetup",

        "dotnet",

        "unins000",

        "launchersetting",

        "redist",

        "settings",

    ];

    fn walk(dir: &Path, depth: u8, candidates: &mut Vec<(bool, String)>) {

        if depth == 0 {

            return;

        }

        if let Ok(entries) = std::fs::read_dir(dir) {

            for entry in entries.flatten() {

                let path = entry.path();

                if path.is_dir() {

                    walk(&path, depth - 1, candidates);

                } else {

                    let name = path.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();

                    if !name.ends_with(".exe") {

                        continue;

                    }

                    if EXCLUSIONS.iter().any(|exclusion| name.contains(exclusion)) {

                        continue;

                    }

                    let stem = name.trim_end_matches(".exe");

                    let parent = path.parent()

                        .and_then(|p| p.file_name())

                        .map(|n| n.to_string_lossy().to_lowercase())

                        .unwrap_or_default();

                    candidates.push((stem == parent, path.to_string_lossy().to_string()));

                }

            }

        }

    }

    let mut candidates: Vec<(bool, String)> = Vec::new();

    walk(Path::new(folder), 4, &mut candidates);

    candidates.sort_by(|a, b| b.0.cmp(&a.0));

    candidates.first().map(|(_, path)| path.clone())

}

pub fn add_steam_shortcut(vdf_path: &str, app_name: &str, exe_path: &str, start_dir: &str, launch_options: &str) -> Result<u32, String> {

    let data = std::fs::read(vdf_path).map_err(|error| format!("read {}: {}", vdf_path, error))?;

    let name_marker = vdf_string("AppName", app_name);

    if let Some(found) = find_subslice(&data, &name_marker, 0) {

        if let Some(index) = find_subslice(&data, b"\x02appid\x00", found.saturating_sub(64))

            .and_then(|marker| entry_index_at(&data, marker))

        {

            return Ok(index);

        }

    }

    let index = max_entry_index(&data) + 1;

    let mut entry: Vec<u8> = Vec::new();

    entry.push(0);

    entry.extend_from_slice(index.to_string().as_bytes());

    entry.push(0);

    entry.extend_from_slice(&vdf_int("appid", shortcut_appid(exe_path, app_name)));

    entry.extend_from_slice(&vdf_string("AppName", app_name));

    entry.extend_from_slice(&vdf_string("Exe", &format!("\"{}\"", exe_path)));

    entry.extend_from_slice(&vdf_string("StartDir", start_dir));

    entry.extend_from_slice(&vdf_string("icon", ""));

    entry.extend_from_slice(&vdf_string("ShortcutPath", ""));

    entry.extend_from_slice(&vdf_string("LaunchOptions", launch_options));

    entry.extend_from_slice(&vdf_int("IsHidden", 0));

    entry.extend_from_slice(&vdf_int("AllowDesktopConfig", 1));

    entry.extend_from_slice(&vdf_int("AllowOverlay", 1));

    entry.extend_from_slice(&vdf_int("OpenVR", 0));

    entry.extend_from_slice(&vdf_int("Devkit", 0));

    entry.extend_from_slice(&vdf_string("DevkitGameID", ""));

    entry.extend_from_slice(&vdf_int("DevkitOverrideAppID", 0));

    entry.extend_from_slice(&vdf_int("LastPlayTime", 0));

    entry.extend_from_slice(&vdf_string("FlatpakAppID", ""));

    entry.extend_from_slice(&vdf_string("sortas", ""));

    entry.push(0);

    entry.extend_from_slice(b"tags");

    entry.push(0);

    entry.push(8);

    entry.push(8);

    if data.last() != Some(&8u8) {

        return Err(String::from("vdf does not end with map terminator"));

    }

    let mut out = data[..data.len() - 1].to_vec();

    out.extend_from_slice(&entry);

    out.push(8);

    std::fs::write(vdf_path, &out).map_err(|error| format!("write {}: {}", vdf_path, error))?;

    let verify = std::fs::read(vdf_path).map_err(|error| format!("re-read {}: {}", vdf_path, error))?;

    if !find_subslice(&verify, &name_marker, 0).is_some() {

        return Err(String::from("verification failed: entry not found after write"));

    }

    Ok(index)

}

pub fn extract_archive(archive_path: &str, dest_dir: &str) -> Result<u32, String> {

    let trimmed_dest = dest_dir.trim_end_matches('/');

    std::fs::create_dir_all(trimmed_dest).map_err(|error| format!("create {}: {}", trimmed_dest, error))?;

    let destination = format!("{}/", trimmed_dest);

    let password_flag = format!("-p{}", RAR_PASSWORD);

    let output = std::process::Command::new("unrar")

        .arg("x")

        .arg(password_flag)

        .arg("-o+")

        .arg("-idp")

        .arg(archive_path)

        .arg(destination)

        .output()

        .map_err(|error| format!("spawn unrar: {}", error))?;

    if !output.status.success() {

        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        let lines: Vec<String> = String::from_utf8_lossy(&output.stdout)

            .lines()

            .map(String::from)

            .collect();

        let tail_start = lines.len().saturating_sub(5);

        let stdout_tail: String = lines[tail_start..].join(" | ");

        return Err(format!(

            "unrar exit {:?}: stderr=[{}] stdout_tail=[{}]",

            output.status.code(),

            stderr,

            stdout_tail

        ));

    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let count = stdout.lines().filter(|line| line.starts_with("Extracting")).count();

    Ok(count as u32)

}
