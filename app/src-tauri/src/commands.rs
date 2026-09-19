use crate::flog;
use fix_core::discover_parts;
use crate::DownloadEngine;
use crate::DownloadProgress;
use tauri::Emitter;

pub(crate) fn prof_span<T>(label: &str, run: impl FnOnce() -> T) -> T {

    let start = std::time::Instant::now();

    let result = run();

    flog(&format!("[PROF] {} {}ms", label, start.elapsed().as_millis()));

    result

}

#[tauri::command]
pub async fn list_games(page: u32) -> Result<fix_core::GamesPage, String> {

    tauri::async_runtime::spawn_blocking(move || prof_span("list_games", || fix_core::home_games(page)))

        .await

        .map_err(|error| error.to_string())

}

#[tauri::command]
pub async fn find_games(query: String) -> Result<fix_core::GamesPage, String> {

    tauri::async_runtime::spawn_blocking(move || fix_core::search_games(&query))

        .await

        .map_err(|error| error.to_string())

}

#[tauri::command]
pub async fn game_detail(url: String) -> Result<fix_core::GameDetail, String> {

    tauri::async_runtime::spawn_blocking(move || prof_span("game_detail", || fix_core::fetch_detail(&url)))

        .await

        .map_err(|error| error.to_string())

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

        let session = engine.session.lock().map_err(|error| error.to_string())?.clone();

        if let Some(session) = session {

            if let Err(error) = session.pause(&handle).await {

                flog(&format!("[DL] pause all failed: {}", error));

            }

        }

    }

    Ok(())

}

// The catalog calls this once per visible game, so an unconditional line here floods startup
// (measured 2026-09-19: ~20 lines before anything else happens). Off by default.
fn assets_debug_enabled() -> bool {

    std::env::var_os("FIX_DEBUG_ASSETS").is_some()

}

#[tauri::command]

pub async fn game_assets(title: String) -> Result<Option<fix_core::GameAssets>, String> {

    let log_title = title.clone();

    let debug = assets_debug_enabled();

    let assets = tauri::async_runtime::spawn_blocking(move || {

        let found = prof_span("game_assets", || fix_core::game_assets(&title));

        if debug {

            if let Some(assets) = &found {

                flog(&format!("[ASSETS] {}: appid {} hero {}", log_title, assets.appid, assets.hero_url));

            }

        }

        found

    })

    .await

    .map_err(|error| error.to_string())?;

    Ok(assets)

}

#[tauri::command]

pub async fn cancel_download(app: tauri::AppHandle, engine: tauri::State<'_, DownloadEngine>, title: String) -> Result<bool, String> {

    let safe_title = title.replace('/', "_");

    let http_found = {

        let cancels = engine.cancels.lock().map_err(|error| error.to_string())?;

        match cancels.get(&safe_title) {

            Some(flag) => {

                flag.store(true, std::sync::atomic::Ordering::Relaxed);

                flog(&format!("[DL] {}: cancel requested", title));

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

        let session = engine.session.lock().map_err(|error| error.to_string())?.clone();

        let Some(session) = session else {

            return Err(String::from("torrent engine not ready"));

        };

        session

            .pause(&handle)

            .await

            .map_err(|error| format!("pause: {}", error))?;

        flog(&format!("[DL] {}: torrent paused by user", title));

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
