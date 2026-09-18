use crate::GameEntry;
use crate::GameDetail;
use crate::DownloadLane;
use crate::*;

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

    } else if url.contains("pixeldrain.com") {

        Some("mirror")

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

    for marker in ["youtube.com/embed/", "youtube-nocookie.com/embed/", "watch?v=", "ytimg.com/vi/"] {

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

