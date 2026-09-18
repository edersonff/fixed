use bytes::Bytes;

use librqbit::AddTorrent;

use librqbit::AddTorrentOptions;

use librqbit::Session;

use librqbit::SessionOptions;

use serde::Serialize;

use std::sync::Arc;

use tauri::Emitter;

use tauri::Manager;

use fix_core::discover_parts;

use fix_core::fetch_detail;

use fix_core::home_games;

use fix_core::search_games;

const LANE_AUTOClick: &str = r#"

(function () {

  const timer = setInterval(() => {

    const candidates = Array.from(document.querySelectorAll('a, button, div, span')).filter((el) => {

      return el.textContent && el.textContent.trim() === 'Download' && el.children.length === 0;

    });

    if (candidates.length > 0) {

      clearInterval(timer);

      candidates[0].click();

    }

  }, 1000);

})();

"#;

#[derive(Serialize, Clone)]

#[serde(rename_all = "camelCase")]

struct DownloadProgress {

    title: String,

    downloaded_bytes: u64,

    total_bytes: u64,

    state: String,

}

struct DownloadEngine {

    session: Arc<Session>,

}

#[tauri::command]
fn open_download_window(app: tauri::AppHandle, url: String) -> Result<(), String> {

    use tauri::WebviewUrl;

    use tauri::WebviewWindowBuilder;

    let parsed = tauri::Url::parse(&url).map_err(|error| error.to_string())?;

    if let Some(existing) = app.get_webview_window("lane-test") {

        let _ = existing.close();

    }

    WebviewWindowBuilder::new(&app, "lane-test", WebviewUrl::External(parsed))

        .title("FIXED lane")

        .inner_size(520.0, 640.0)

        .initialization_script(LANE_AUTOClick)

        .on_download(|_webview, event| match event {

            tauri::webview::DownloadEvent::Requested { url, destination } => {

                eprintln!("[LANE-TEST] REQUESTED: {} -> {:?}", url, destination);

                false

            }

            tauri::webview::DownloadEvent::Finished { url, path, success } => {

                eprintln!("[LANE-TEST] FINISHED: {} {:?} ok={}", url, path, success);

                true

            }

            _ => true,

        })

        .build()

        .map_err(|error| error.to_string())?;

    Ok(())

}

#[tauri::command]
fn list_games(page: u32) -> fix_core::GamesPage {

    home_games(page)

}

#[tauri::command]
fn find_games(query: String) -> fix_core::GamesPage {

    search_games(&query)

}

#[tauri::command]
fn game_detail(url: String) -> fix_core::GameDetail {

    fetch_detail(&url)

}

#[tauri::command]
fn lane_parts(url: String) -> Vec<String> {

    discover_parts(&url)

}

fn extract_first_rar(folder: &str) -> Result<u32, String> {

    let mut rars: Vec<std::path::PathBuf> = std::fs::read_dir(folder)

        .map_err(|error| format!("read {}: {}", folder, error))?

        .flatten()

        .map(|entry| entry.path())

        .filter(|path| path.extension().map(|ext| ext == "rar").unwrap_or(false))

        .collect();

    rars.sort();

    let first = rars.first().ok_or_else(|| String::from("no .rar found in download folder"))?;

    fix_core::extract_archive(&first.to_string_lossy(), folder)

}

fn find_shortcuts_vdf() -> Option<String> {

    let home = std::env::var("HOME").ok()?;

    let userdata = format!("{}/.steam/steam/userdata", home);

    let entries = std::fs::read_dir(&userdata).ok()?;

    for entry in entries.flatten() {

        let candidate = entry.path().join("config/shortcuts.vdf");

        if candidate.exists() {

            return candidate.to_str().map(String::from);

        }

    }

    None

}

fn add_game_to_steam(title: &str, folder: &str) {

    let Some(vdf) = find_shortcuts_vdf() else {

        eprintln!("[DL] {}: steam shortcuts.vdf not found", title);

        return;

    };

    let Some(exe) = fix_core::find_game_exe(folder) else {

        eprintln!("[DL] {}: no game exe found in {}", title, folder);

        return;

    };

    let start_dir = std::path::Path::new(&exe)

        .parent()

        .map(|parent| format!("{}/", parent.to_string_lossy()))

        .unwrap_or_default();

    match fix_core::add_steam_shortcut(&vdf, title, &exe, &start_dir, fix_core::ONLINE_FIX_LAUNCH_OPTIONS) {

        Ok(index) => eprintln!("[DL] {}: added to Steam (index {})", title, index),

        Err(error) => eprintln!("[DL] {}: steam shortcut FAILED: {}", title, error),

    }

}

fn delete_installers(folder: &str) {

    let Ok(entries) = std::fs::read_dir(folder) else {

        return;

    };

    for entry in entries.flatten() {

        let path = entry.path();

        let is_rar = path.extension().map(|ext| ext == "rar").unwrap_or(false);

        if !is_rar {

            continue;

        }

        match std::fs::remove_file(&path) {

            Ok(()) => eprintln!("[DL] installer removed: {}", path.display()),

            Err(error) => eprintln!("[DL] installer remove FAILED: {}: {}", path.display(), error),

        }

    }

}

#[tauri::command]

async fn start_http_download(app: tauri::AppHandle, title: String, lane_url: String) -> Result<String, String> {

    let mirror_url = fix_core::mirror_download_url(&lane_url)

        .ok_or_else(|| log_fail(&title, "mirror resolve", String::from("no direct mirror in hosters lane")))?;

    let home = std::env::var("HOME")

        .map_err(|error| log_fail(&title, "HOME env", error.to_string()))?;

    let safe_title = title.replace('/', "_");

    let folder = format!("{}/games/{}", home, safe_title);

    std::fs::create_dir_all(&folder)

        .map_err(|error| log_fail(&title, "create game folder", error.to_string()))?;

    let dest = format!("{}/game-download.rar", folder);

    let total = fix_core::http_size(&mirror_url);

    eprintln!("[DL] {}: http mirror {} ({} bytes)", title, mirror_url, total);

    let done = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));

    let total_seen = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(total));

    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));

    let emitter_app = app.clone();

    let emitter_title = title.clone();

    let emitter_done = done.clone();

    let emitter_total = total_seen.clone();

    let emitter_running = running.clone();

    tauri::async_runtime::spawn(async move {

        while emitter_running.load(std::sync::atomic::Ordering::Relaxed) {

            tokio::time::sleep(std::time::Duration::from_millis(700)).await;

            let bytes = emitter_done.load(std::sync::atomic::Ordering::Relaxed);

            let bytes_total = emitter_total.load(std::sync::atomic::Ordering::Relaxed);

            eprintln!("[DL] {}: {}/{} bytes (http)", emitter_title, bytes, bytes_total);

            let _ = emitter_app.emit("download-progress", DownloadProgress {

                title: emitter_title.clone(),

                downloaded_bytes: bytes,

                total_bytes: bytes_total,

                state: String::from("downloading"),

            });

        }

    });

    let dl_url = mirror_url.clone();

    let dl_dest = dest.clone();

    let dl_done = done.clone();

    let dl_total = total_seen.clone();

    let dl_running = running.clone();

    let pipeline_app = app.clone();

    let pipeline_title = title.clone();

    let pipeline_folder = folder.clone();

    tauri::async_runtime::spawn(async move {

        let url = dl_url.clone();

        let dest_path = dl_dest.clone();

        let done_ref = dl_done.clone();

        let total_ref = dl_total.clone();

        let flag_ref = dl_running.clone();

        let result = tokio::task::spawn_blocking(move || {

            let outcome = fix_core::http_download(&url, &dest_path, &|bytes_done, bytes_total| {

                done_ref.store(bytes_done, std::sync::atomic::Ordering::Relaxed);

                if bytes_total > 0 {

                    total_ref.store(bytes_total, std::sync::atomic::Ordering::Relaxed);

                }

            });

            flag_ref.store(false, std::sync::atomic::Ordering::Relaxed);

            outcome

        })

        .await

        .unwrap_or_else(|error| Err(format!("join: {}", error)));

        match result {

            Ok(()) => {

                eprintln!("[DL] {}: http download complete, extracting", pipeline_title);

                let _ = pipeline_app.emit("download-progress", DownloadProgress {

                    title: pipeline_title.clone(),

                    downloaded_bytes: 1,

                    total_bytes: 1,

                    state: String::from("extracting"),

                });

                let extract_folder = pipeline_folder.clone();

                let extract_result = tokio::task::spawn_blocking(move || extract_first_rar(&extract_folder))

                    .await

                    .unwrap_or_else(|error| Err(format!("join: {}", error)));

                let final_state = match extract_result {

                    Ok(count) => {

                        eprintln!("[DL] {}: extracted {} files", pipeline_title, count);

                        add_game_to_steam(&pipeline_title, &pipeline_folder);

                        delete_installers(&pipeline_folder);

                        String::from("ready")

                    }

                    Err(error) => {

                        eprintln!("[DL] {}: extract FAILED: {}", pipeline_title, error);

                        String::from("error")

                    }

                };

                let _ = pipeline_app.emit("download-progress", DownloadProgress {

                    title: pipeline_title.clone(),

                    downloaded_bytes: 1,

                    total_bytes: 1,

                    state: final_state,

                });

            }

            Err(error) => {

                eprintln!("[DL] {}: http download FAILED: {}", pipeline_title, error);

                let _ = pipeline_app.emit("download-progress", DownloadProgress {

                    title: pipeline_title.clone(),

                    downloaded_bytes: 0,

                    total_bytes: 0,

                    state: String::from("error"),

                });

            }

        }

    });

    Ok(mirror_url)

}

#[tauri::command]

fn install_plugin(title: String, archive_path: String) -> Result<u32, String> {

    let home = std::env::var("HOME").map_err(|error| format!("HOME: {}", error))?;

    let folder = format!("{}/games/{}", home, title);

    let Some(exe) = fix_core::find_game_exe(&folder) else {

        let message = format!("no game exe found in {}", folder);

        eprintln!("[PLUGIN] {}: {}", title, message);

        return Err(message);

    };

    let game_dir = std::path::Path::new(&exe)

        .parent()

        .map(|parent| parent.to_string_lossy().to_string())

        .unwrap_or(folder.clone());

    let bep_core = format!("{}/BepInEx/core", game_dir);

    if !std::path::Path::new(&bep_core).exists() {

        eprintln!("[PLUGIN] {}: BepInEx loader not found, plugin placed anyway", title);

    }

    match fix_core::install_plugin(&archive_path, &game_dir) {

        Ok(count) => {

            eprintln!("[PLUGIN] {}: installed {} file(s) into {}", title, count, game_dir);

            match fix_core::apply_fix_repair(&folder, &game_dir) {

                Ok(repaired) if repaired > 0 => {

                    eprintln!("[PLUGIN] {}: fix repair reapplied ({} archive) over plugin", title, repaired);

                }

                Ok(_) => eprintln!("[PLUGIN] {}: no fix repair archive present", title),

                Err(error) => eprintln!("[PLUGIN] {}: fix repair FAILED: {}", title, error),

            }

            Ok(count)

        }

        Err(error) => {

            eprintln!("[PLUGIN] {}: FAILED: {}", title, error);

            Err(error)

        }

    }

}

#[tauri::command]

fn launch_game(title: String) -> Result<String, String> {

    let home = std::env::var("HOME").map_err(|error| format!("HOME: {}", error))?;

    let folder = format!("{}/games/{}", home, title);

    let Some(exe) = fix_core::find_game_exe(&folder) else {

        let message = format!("no game exe found in {}", folder);

        eprintln!("[LAUNCH] {}: {}", title, message);

        return Err(message);

    };

    if find_shortcuts_vdf().is_some() {

        add_game_to_steam(&title, &folder);

    } else {

        eprintln!("[LAUNCH] {}: steam shortcuts.vdf not found", title);

    }

    let appid = fix_core::shortcut_appid(&exe, &title);

    let url = format!("steam://rungameid/{}", appid);

    std::process::Command::new("xdg-open")

        .arg(&url)

        .stdin(std::process::Stdio::null())

        .stdout(std::process::Stdio::null())

        .stderr(std::process::Stdio::null())

        .spawn()

        .map(|child| {

            eprintln!("[LAUNCH] {}: {} (pid {})", title, url, child.id());

            format!("launched:{}", url)

        })

        .map_err(|error| {

            let message = format!("xdg-open: {}", error);

            eprintln!("[LAUNCH] {}: {}", title, message);

            message

        })

}

fn log_fail(title: &str, step: &str, error: String) -> String {

    eprintln!("[DL] {}: {} FAILED: {}", title, step, error);

    error

}

#[tauri::command]
async fn start_torrent_download(app: tauri::AppHandle, engine: tauri::State<'_, DownloadEngine>, title: String, lane_url: String) -> Result<String, String> {

    let torrent_url = fix_core::torrent_file_url(&lane_url)

        .ok_or_else(|| log_fail(&title, "lane resolve", String::from("no .torrent file found in lane")))?;

    let torrent_bytes = fix_core::fetch_bytes(&torrent_url)

        .map_err(|error| log_fail(&title, "torrent fetch", error))?;

    let home = std::env::var("HOME")

        .map_err(|error| log_fail(&title, "HOME env", error.to_string()))?;

    let safe_title = title.replace('/', "_");

    let folder = format!("{}/games/{}", home, safe_title);

    std::fs::create_dir_all(&folder)

        .map_err(|error| log_fail(&title, "create game folder", error.to_string()))?;

    let game_folder = folder.clone();

    let mut opts = AddTorrentOptions::default();

    opts.output_folder = Some(folder);

    opts.overwrite = true;

    let session = engine.session.clone();

    let response = session

        .add_torrent(AddTorrent::TorrentFileBytes(Bytes::from(torrent_bytes)), Some(opts))

        .await

        .map_err(|error| log_fail(&title, "add torrent", error.to_string()))?;

    let handle = response.into_handle()

        .ok_or_else(|| log_fail(&title, "into handle", String::from("no handle returned")))?;

    let emit_title = title.clone();

    let cancel_token = session.cancellation_token().clone();

    tauri::async_runtime::spawn(async move {

        loop {

            tokio::select! {

                _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {}

                _ = cancel_token.cancelled() => {

                    eprintln!("[DL] {}: session cancelled, emitter stopped", emit_title);

                    break;

                }

            }

            let stats = handle.stats();

            eprintln!("[DL] {}: {}/{} bytes", emit_title, stats.progress_bytes, stats.total_bytes);

            let _ = app.emit("download-progress", DownloadProgress {

                title: emit_title.clone(),

                downloaded_bytes: stats.progress_bytes,

                total_bytes: stats.total_bytes,

                state: String::from("downloading"),

            });

            if stats.total_bytes > 0 && stats.progress_bytes >= stats.total_bytes {

                eprintln!("[DL] {}: download complete, extracting", emit_title);

                let _ = app.emit("download-progress", DownloadProgress {

                    title: emit_title.clone(),

                    downloaded_bytes: stats.progress_bytes,

                    total_bytes: stats.total_bytes,

                    state: String::from("extracting"),

                });

                let extract_title = emit_title.clone();

                let extract_folder = game_folder.clone();

                let extract_app = app.clone();

                tauri::async_runtime::spawn(async move {

                    let blocking_folder = extract_folder.clone();

                    let result = tokio::task::spawn_blocking(move || extract_first_rar(&blocking_folder))

                        .await

                        .unwrap_or_else(|error| Err(format!("join: {}", error)));

                    let final_state = match result {

                        Ok(count) => {

                            eprintln!("[DL] {}: extracted {} files", extract_title, count);

                            add_game_to_steam(&extract_title, &extract_folder);

                            delete_installers(&extract_folder);

                            String::from("ready")

                        }

                        Err(error) => {

                            eprintln!("[DL] {}: extract FAILED: {}", extract_title, error);

                            String::from("error")

                        }

                    };

                    let _ = extract_app.emit("download-progress", DownloadProgress {

                        title: extract_title,

                        downloaded_bytes: 0,

                        total_bytes: 0,

                        state: final_state,

                    });

                });

                break;

            }

        }

    });

    Ok(String::from("downloading"))

}

#[tauri::command]
async fn cancel_all_downloads(engine: tauri::State<'_, DownloadEngine>) -> Result<(), String> {

    engine.session.cancellation_token().cancel();

    Ok(())

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));

    let games_dir = format!("{}/games", home);

    std::fs::create_dir_all(&games_dir).expect("create ~/games");

    let mut session_opts = SessionOptions::default();

    session_opts.disable_dht_persistence = true;

    let session = tauri::async_runtime::block_on(Session::new_with_opts(games_dir.into(), session_opts))

        .expect("start torrent session");

    tauri::Builder::default()

        .plugin(tauri_plugin_opener::init())

        .plugin(tauri_plugin_dialog::init())

        .manage(DownloadEngine { session })

        .invoke_handler(tauri::generate_handler![list_games, find_games, game_detail, lane_parts, open_download_window, start_torrent_download, start_http_download, cancel_all_downloads, launch_game, install_plugin])

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

}
