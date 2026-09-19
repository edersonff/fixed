use fix_core::discover_parts;
use fix_core::fetch_detail;
use fix_core::search_games;
use fix_core::home_games;
use crate::DownloadEngine;
use crate::DownloadProgress;
use tauri::Emitter;

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

    if let Ok(cancels) = engine.cancels.lock() {

        for flag in cancels.values() {

            flag.store(true, std::sync::atomic::Ordering::Relaxed);

        }

    }

    let handles: Vec<std::sync::Arc<librqbit::ManagedTorrent>> = engine

        .torrents

        .lock()

        .map(|active| active.values().cloned().collect())

        .unwrap_or_default();

    for handle in handles {

        if let Err(error) = engine.session.pause(&handle).await {

            eprintln!("[DL] pause all failed: {}", error);

        }

    }

    Ok(())

}

#[tauri::command]

pub fn game_assets(title: String) -> Option<fix_core::GameAssets> {

    let assets = fix_core::game_assets(&title)?;

    eprintln!("[ASSETS] {}: appid {} hero {}", title, assets.appid, assets.hero_url);

    Some(assets)

}

#[tauri::command]

pub async fn cancel_download(app: tauri::AppHandle, engine: tauri::State<'_, DownloadEngine>, title: String) -> Result<bool, String> {

    let safe_title = title.replace('/', "_");

    let http_found = {

        let cancels = engine.cancels.lock().map_err(|error| error.to_string())?;

        match cancels.get(&safe_title) {

            Some(flag) => {

                flag.store(true, std::sync::atomic::Ordering::Relaxed);

                eprintln!("[DL] {}: cancel requested", title);

                true

            }

            None => false,

        }

    };

    if http_found {

        return Ok(true);

    }

    let torrent_handle = engine

        .torrents

        .lock()

        .map_err(|error| error.to_string())?

        .get(&safe_title)

        .cloned();

    if let Some(handle) = torrent_handle {

        engine

            .session

            .pause(&handle)

            .await

            .map_err(|error| format!("pause: {}", error))?;

        eprintln!("[DL] {}: torrent paused by user", title);

        let _ = app.emit("download-progress", DownloadProgress {

            title: title.clone(),

            downloaded_bytes: 0,

            total_bytes: 0,

            state: String::from("stopped"),

        });

        return Ok(true);

    }

    Ok(false)

}
