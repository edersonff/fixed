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
