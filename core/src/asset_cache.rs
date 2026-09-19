use std::fs;

const ASSET_CACHE_VERSION: &str = "v1";

#[derive(Clone, Default, PartialEq, Debug)]
pub(crate) struct ResolvedAssets {

    pub hero_url: String,

    pub logo_url: String,

    pub cover_url: String,

}

fn asset_cache_path() -> Option<String> {

    let home = crate::fixed_home()?;

    let dir = format!("{}/.cache/fixed", home);

    fs::create_dir_all(&dir).ok()?;

    Some(format!("{}/asset_urls.cache", dir))

}

fn parse_asset_cache_line(line: &str) -> Option<(u32, ResolvedAssets)> {

    let mut parts = line.split('\t');

    let appid: u32 = parts.next()?.parse().ok()?;

    let hero_url = parts.next()?.to_string();

    let logo_url = parts.next()?.to_string();

    let cover_url = parts.next()?.to_string();

    Some((appid, ResolvedAssets { hero_url, logo_url, cover_url }))

}

fn format_asset_cache_line(appid: u32, resolved: &ResolvedAssets) -> String {

    format!("{}\t{}\t{}\t{}\n", appid, resolved.hero_url, resolved.logo_url, resolved.cover_url)

}

pub(crate) fn load_asset_cache() -> Vec<(u32, ResolvedAssets)> {

    let Some(path) = asset_cache_path() else {

        return Vec::new();

    };

    let Ok(text) = fs::read_to_string(path) else {

        return Vec::new();

    };

    let mut lines = text.lines();

    if lines.next() != Some(ASSET_CACHE_VERSION) {

        return Vec::new();

    }

    lines.filter_map(parse_asset_cache_line).collect()

}

pub(crate) fn save_asset_cache(entries: &[(u32, ResolvedAssets)]) {

    let Some(path) = asset_cache_path() else {

        return;

    };

    let mut body = format!("{}\n", ASSET_CACHE_VERSION);

    for (appid, resolved) in entries {

        body.push_str(&format_asset_cache_line(*appid, resolved));

    }

    let _ = fs::write(path, body);

}

#[cfg(test)]
#[path = "asset_cache_tests.rs"]
mod asset_cache_tests;
