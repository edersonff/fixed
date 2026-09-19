use crate::assets::STEAM_API_AGENT;

use std::io::Read;

const STORE_ITEM_ASSETS_BASE: &str = "https://shared.akamai.steamstatic.com/store_item_assets/";

const STOREPAGEBACKGROUND_BASE: &str = "https://store.akamai.steamstatic.com/images/storepagebackground/";

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct AssetManifest {

    pub asset_url_format: Option<String>,

    pub library_hero_2x: Option<String>,

    pub library_hero: Option<String>,

    pub hero_capsule: Option<String>,

    pub header: Option<String>,

    pub page_background_path: Option<String>,

    pub library_capsule_2x: Option<String>,

    pub library_capsule: Option<String>,

    pub main_capsule: Option<String>,

}

fn json_string_field(body: &str, key: &str) -> Option<String> {

    let marker = format!("\"{}\":\"", key);

    let start = body.find(&marker)? + marker.len();

    let raw: String = body[start..].chars().take_while(|c| *c != '"').collect();

    if raw.is_empty() {

        None

    } else {

        Some(raw.replace("\\/", "/"))

    }

}

// GetItems is one keyless call per appid that carries every per-file hash Steam assigned; the
// hashes are otherwise unguessable and change per file, which is why the legacy CDN path 404s.
pub(crate) fn parse_get_items(body: &str) -> AssetManifest {

    AssetManifest {

        asset_url_format: json_string_field(body, "asset_url_format"),

        library_hero_2x: json_string_field(body, "library_hero_2x"),

        library_hero: json_string_field(body, "library_hero"),

        hero_capsule: json_string_field(body, "hero_capsule"),

        header: json_string_field(body, "header"),

        page_background_path: json_string_field(body, "page_background_path"),

        library_capsule_2x: json_string_field(body, "library_capsule_2x"),

        library_capsule: json_string_field(body, "library_capsule"),

        main_capsule: json_string_field(body, "main_capsule"),

    }

}

fn templated_url(manifest: &AssetManifest, filename: Option<&str>) -> Option<String> {

    let format = manifest.asset_url_format.as_deref()?;

    let filename = filename?;

    Some(format!("{}{}", STORE_ITEM_ASSETS_BASE, format.replace("${FILENAME}", filename)))

}

pub(crate) fn hero_candidates(manifest: &AssetManifest) -> Vec<String> {

    let mut candidates = Vec::new();

    for filename in [manifest.library_hero_2x.as_deref(), manifest.library_hero.as_deref(), manifest.hero_capsule.as_deref()] {

        if let Some(url) = templated_url(manifest, filename) {

            candidates.push(url);

        }

    }

    if let Some(path) = manifest.page_background_path.as_deref() {

        candidates.push(format!("{}{}", STOREPAGEBACKGROUND_BASE, path));

    }

    if let Some(url) = templated_url(manifest, manifest.header.as_deref()) {

        candidates.push(url);

    }

    candidates

}

pub(crate) fn cover_candidates(manifest: &AssetManifest) -> Vec<String> {

    [manifest.library_capsule_2x.as_deref(), manifest.library_capsule.as_deref(), manifest.main_capsule.as_deref()]

        .into_iter()

        .filter_map(|filename| templated_url(manifest, filename))

        .collect()

}

pub(crate) fn fetch_asset_manifest(appid: u32) -> Option<AssetManifest> {

    let input = format!(

        "{{\"ids\":[{{\"appid\":{}}}],\"context\":{{\"language\":\"english\",\"country_code\":\"US\"}},\"data_request\":{{\"include_assets\":true}}}}",

        appid

    );

    let response = ureq::get("https://api.steampowered.com/IStoreBrowseService/GetItems/v1/")

        .query("input_json", &input)

        .set("User-Agent", STEAM_API_AGENT)

        .timeout(std::time::Duration::from_secs(12))

        .call()

        .ok()?;

    let mut body = String::new();

    response.into_reader().take(400_000).read_to_string(&mut body).ok()?;

    Some(parse_get_items(&body))

}

#[cfg(test)]
#[path = "store_items_tests.rs"]
mod store_items_tests;
