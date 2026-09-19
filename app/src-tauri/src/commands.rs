use fix_core::discover_parts;
use fix_core::fetch_detail;
use fix_core::search_games;
use fix_core::home_games;
use crate::DownloadEngine;

#[tauri::command]
pub fn list_games(page: u32) -> fix_core::GamesPage {

    home_games(page)

}

#[tauri::command]
pub fn find_games(query: String) -> fix_core::GamesPage {

    search_games(&query)

}

#[tauri::command]
pub fn game_detail(url: String) -> fix_core::GameDetail {

    fetch_detail(&url)

}

#[tauri::command]
pub fn lane_parts(url: String) -> Vec<String> {

    discover_parts(&url)

}

#[tauri::command]

pub async fn cancel_all_downloads(engine: tauri::State<'_, DownloadEngine>) -> Result<(), String> {

    engine.session.cancellation_token().cancel();

    Ok(())

}

#[tauri::command]

pub fn game_assets(title: String) -> Option<fix_core::GameAssets> {

    let assets = fix_core::game_assets(&title)?;

    eprintln!("[ASSETS] {}: appid {} hero {}", title, assets.appid, assets.hero_url);

    Some(assets)

}

#[tauri::command]

pub fn cancel_download(engine: tauri::State<'_, DownloadEngine>, title: String) -> Result<bool, String> {

    let safe_title = title.replace('/', "_");

    let cancels = engine.cancels.lock().map_err(|error| error.to_string())?;

    let found = match cancels.get(&safe_title) {

        Some(flag) => {

            flag.store(true, std::sync::atomic::Ordering::Relaxed);

            eprintln!("[DL] {}: cancel requested", title);

            true

        }

        None => false,

    };

    Ok(found)

}
