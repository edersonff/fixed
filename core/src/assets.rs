use crate::appid_cache::resolve_appid;

use crate::asset_cache::load_asset_cache;

use crate::asset_cache::save_asset_cache;

use crate::asset_cache::ResolvedAssets;

use crate::store_items::cover_candidates;

use crate::store_items::fetch_asset_manifest;

use crate::store_items::hero_candidates;

use crate::store_items::AssetManifest;

pub(crate) const STEAM_API_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36";

pub fn steam_hero_url(appid: u32) -> String {

    format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/library_hero.jpg", appid)

}

pub fn steam_logo_url(appid: u32) -> String {

    format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/logo_2x.png", appid)

}

pub fn steam_cover_url(appid: u32) -> String {

    format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/library_600x900_2x.jpg", appid)

}

fn url_exists(url: &str) -> bool {

    ureq::head(url)

        .set("User-Agent", STEAM_API_AGENT)

        .timeout(std::time::Duration::from_secs(8))

        .call()

        .map(|response| response.status() == 200)

        .unwrap_or(false)

}

fn first_existing(candidates: &[String]) -> String {

    candidates.iter().find(|url| url_exists(url)).cloned().unwrap_or_default()

}

// GetItems carries no logo key for any appid (modern or legacy, checked both); the hero's hash
// folder never contains a logo file either. A missing logo on a modern appid is correct behavior,
// not a gap: the UI falls back to the title text.
fn resolve_logo(appid: u32) -> String {

    let legacy = steam_logo_url(appid);

    if url_exists(&legacy) {

        legacy

    } else {

        String::new()

    }

}

fn resolve_hero(manifest: Option<&AssetManifest>) -> String {

    match manifest {

        Some(manifest) => first_existing(&hero_candidates(manifest)),

        None => String::new(),

    }

}

fn resolve_cover(manifest: Option<&AssetManifest>) -> String {

    match manifest {

        Some(manifest) => first_existing(&cover_candidates(manifest)),

        None => String::new(),

    }

}

#[derive(serde::Serialize)]

#[serde(rename_all = "camelCase")]

pub struct GameAssets {

    pub appid: u32,

    pub hero_url: String,

    pub logo_url: String,

    pub cover_url: String,

}

pub(crate) fn resolved_assets(appid: u32) -> ResolvedAssets {

    let mut cache = load_asset_cache();

    if let Some((_, resolved)) = cache.iter().find(|(id, _)| *id == appid) {

        return resolved.clone();

    }

    let manifest = fetch_asset_manifest(appid);

    let resolved = ResolvedAssets {

        hero_url: resolve_hero(manifest.as_ref()),

        logo_url: resolve_logo(appid),

        cover_url: resolve_cover(manifest.as_ref()),

    };

    cache.push((appid, resolved.clone()));

    save_asset_cache(&cache);

    resolved

}

pub fn game_assets(title: &str) -> Option<GameAssets> {

    let appid = resolve_appid(title)?;

    Some(to_game_assets(appid, &resolved_assets(appid)))

}

fn to_game_assets(appid: u32, resolved: &ResolvedAssets) -> GameAssets {

    GameAssets { appid, hero_url: resolved.hero_url.clone(), logo_url: resolved.logo_url.clone(), cover_url: resolved.cover_url.clone() }

}

#[cfg(test)]
#[path = "assets_tests.rs"]
mod assets_tests;
