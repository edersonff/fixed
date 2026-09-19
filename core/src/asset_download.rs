use std::fs;

use std::io::Read;

use crate::resolved_assets;

fn extension_for(content_type: &str) -> &'static str {

    if content_type.contains("png") {

        "png"

    } else {

        "jpg"

    }

}

fn save_one(kind: &str, url: &str, dir: &str) -> Result<String, String> {

    let response = ureq::get(url).call().map_err(|error| format!("{} {}: {}", kind, url, error))?;

    let content_type = response.content_type().to_string();

    let mut bytes = Vec::new();

    response.into_reader().read_to_end(&mut bytes).map_err(|error| format!("{} read: {}", kind, error))?;

    let path = format!("{}/{}.{}", dir, kind, extension_for(&content_type));

    fs::write(&path, &bytes).map_err(|error| format!("write {}: {}", path, error))?;

    Ok(format!("{} {} bytes", path, bytes.len()))

}

pub fn save_app_assets(appid: u32, cache_root: &str) -> Result<Vec<String>, String> {

    let dir = format!("{}/{}", cache_root, appid);

    fs::create_dir_all(&dir).map_err(|error| format!("create {}: {}", dir, error))?;

    let resolved = resolved_assets(appid);

    let mut saved = Vec::new();

    for (kind, url) in [

        ("hero", resolved.hero_url.as_str()),

        ("cover", resolved.cover_url.as_str()),

        ("logo", resolved.logo_url.as_str()),

    ] {

        if url.is_empty() {

            saved.push(format!("{} none", kind));

            continue;

        }

        saved.push(save_one(kind, url, &dir)?);

    }

    Ok(saved)

}

#[cfg(test)]
#[path = "asset_download_tests.rs"]
mod asset_download_tests;
