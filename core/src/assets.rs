use std::fs;

use std::io::Read;

pub struct AssetSpec {

    pub kind: &'static str,

    pub file: &'static str,

}

pub const ASSETS: [AssetSpec; 4] = [

    AssetSpec { kind: "cover", file: "library_600x900_2x.jpg" },

    AssetSpec { kind: "logo", file: "logo_2x.png" },

    AssetSpec { kind: "hero", file: "library_hero.jpg" },

    AssetSpec { kind: "capsule", file: "capsule_616x353.jpg" },

];

fn extension_for(content_type: &str) -> &'static str {

    if content_type.contains("png") {

        "png"

    } else {

        "jpg"

    }

}

pub fn save_app_assets(appid: u32, cache_root: &str) -> Result<Vec<String>, String> {

    let dir = format!("{}/{}", cache_root, appid);

    fs::create_dir_all(&dir).map_err(|error| format!("create {}: {}", dir, error))?;

    let mut saved = Vec::new();

    for spec in ASSETS.iter() {

        let url = format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/{}", appid, spec.file);

        let response = ureq::get(&url).call().map_err(|error| format!("{} {}: {}", spec.kind, url, error))?;

        let content_type = response.content_type().to_string();

        let mut bytes = Vec::new();

        response.into_reader().read_to_end(&mut bytes).map_err(|error| format!("{} read: {}", spec.kind, error))?;

        let path = format!("{}/{}.{}", dir, spec.kind, extension_for(&content_type));

        fs::write(&path, &bytes).map_err(|error| format!("write {}: {}", path, error))?;

        saved.push(format!("{} {} bytes", path, bytes.len()));

    }

    Ok(saved)

}


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

fn search_appid(title: &str) -> Option<u32> {

    let query = title.replace(' ', "+");

    let url = format!("https://store.steampowered.com/api/storesearch/?term={}&cc=us&l=en", query);

    let response = ureq::get(&url)

        .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")

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

pub fn steam_hero_url(appid: u32) -> String {

    format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/library_hero.jpg", appid)

}

pub fn steam_logo_url(appid: u32) -> String {

    format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/logo_2x.png", appid)

}

#[derive(serde::Serialize)]

#[serde(rename_all = "camelCase")]

pub struct GameAssets {

    pub appid: u32,

    pub hero_url: String,

    pub logo_url: String,

}

pub fn game_assets(title: &str) -> Option<GameAssets> {

    let appid = resolve_appid(title)?;

    Some(GameAssets {

        appid,

        hero_url: steam_hero_url(appid),

        logo_url: steam_logo_url(appid),

    })

}
