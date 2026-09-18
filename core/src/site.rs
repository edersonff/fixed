use crate::USER_AGENT;
use crate::HOME_FIXTURE;
use crate::GameEntry;
use crate::GameDetail;
use crate::*;

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

