use std::fs;

use std::io::Read;

use crate::assets::STEAM_API_AGENT;

fn cache_path() -> Option<String> {

    let home = std::env::var("HOME").ok()?;

    let dir = format!("{}/.cache/fixed", home);

    fs::create_dir_all(&dir).ok()?;

    Some(format!("{}/appids.json", dir))

}

fn load_cache() -> Vec<(String, u32)> {

    let Some(path) = cache_path() else {

        return Vec::new();

    };

    let Ok(text) = fs::read_to_string(path) else {

        return Vec::new();

    };

    text.lines()

        .filter_map(|line| {

            let (title, id) = line.split_once('=')?;

            let id: u32 = id.parse().ok()?;

            Some((title.to_string(), id))

        })

        .collect()

}

fn save_cache(entries: &[(String, u32)]) {

    let Some(path) = cache_path() else {

        return;

    };

    let body: String = entries.iter().map(|(t, id)| format!("{}={}\n", t, id)).collect();

    let _ = fs::write(path, body);

}

// online-fix.me titles carry the release alias in brackets ("Dayz (dayzavr)", "Game [v1.2]").
// Steam's storesearch takes them literally and returns total=0, so the game gets no art at all.
// Measured 2026-09-19: "Dayz (dayzavr)" -> total 0; "Dayz" -> DayZ, appid 221100.
pub(crate) fn search_terms(title: &str) -> Vec<String> {

    let mut terms = vec![title.trim().to_string()];

    let stripped = strip_bracketed(title);

    if !stripped.is_empty() && stripped != terms[0] {

        terms.push(stripped);

    }

    terms

}

fn strip_bracketed(title: &str) -> String {

    let mut out = String::new();

    let mut depth = 0u32;

    for character in title.chars() {

        match character {

            '(' | '[' => depth += 1,

            ')' | ']' => depth = depth.saturating_sub(1),

            _ if depth == 0 => out.push(character),

            _ => {}

        }

    }

    out.split_whitespace().collect::<Vec<_>>().join(" ")

}

fn search_appid(title: &str) -> Option<u32> {

    search_terms(title).into_iter().find_map(|term| search_appid_exact(&term))

}

fn search_appid_exact(title: &str) -> Option<u32> {

    let query = title.replace(' ', "+");

    let url = format!("https://store.steampowered.com/api/storesearch/?term={}&cc=us&l=en", query);

    let response = ureq::get(&url)

        .set("User-Agent", STEAM_API_AGENT)

        .timeout(std::time::Duration::from_secs(12))

        .call()

        .ok()?;

    let mut body = String::new();

    response.into_reader().take(200_000).read_to_string(&mut body).ok()?;

    let marker = "\"id\":";

    let start = body.find(marker)? + marker.len();

    let digits: String = body[start..].chars().take_while(|c| c.is_ascii_digit()).collect();

    digits.parse().ok()

}

pub fn resolve_appid(title: &str) -> Option<u32> {

    let key = title.to_lowercase();

    let mut cache = load_cache();

    if let Some((_, id)) = cache.iter().find(|(t, _)| *t == key) {

        return Some(*id);

    }

    let id = search_appid(title)?;

    cache.push((key, id));

    save_cache(&cache);

    Some(id)

}

#[cfg(test)]
#[path = "appid_cache_tests.rs"]
mod appid_cache_tests;
