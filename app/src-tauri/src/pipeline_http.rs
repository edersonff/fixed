use crate::DownloadEngine;
use crate::DownloadProgress;
use crate::add_game_to_steam;
use crate::delete_installers;
use crate::extract_first_rar;
use crate::log_fail;
use tauri::Emitter;
use tauri::Manager;

#[tauri::command]

pub async fn start_http_download(app: tauri::AppHandle, title: String, lane_url: String) -> Result<String, String> {

    let mirror_url = fix_core::mirror_download_url(&lane_url)

        .map_err(|error| log_fail(&title, "mirror resolve", error))?

        .ok_or_else(|| log_fail(&title, "mirror resolve", format!("no direct mirror in hosters lane {}", lane_url)))?;

    let safe_title = title.replace('/', "_");

    let folder = crate::game_folder(&safe_title)

        .ok_or_else(|| log_fail(&title, "home dir", String::from("HOME and USERPROFILE unset")))?;

    std::fs::create_dir_all(&folder)

        .map_err(|error| log_fail(&title, "create game folder", error.to_string()))?;

    if let Ok(entries) = std::fs::read_dir(&folder) {

        for entry in entries.flatten() {

            let path = entry.path();

            let is_rar = path.extension().map(|ext| ext == "rar").unwrap_or(false);

            if !is_rar {

                continue;

            }

            match std::fs::remove_file(&path) {

                Ok(()) => eprintln!("[DL] pre-clean removed stale archive: {}", path.display()),

                Err(error) => eprintln!("[DL] pre-clean kept {}: {}", path.display(), error),

            }

        }

    }

    let dest = std::path::Path::new(&folder).join("game-download.rar").to_string_lossy().to_string();

    let total = fix_core::http_size(&mirror_url);

    eprintln!("[DL] {}: http mirror {} ({} bytes)", title, mirror_url, total);

    let cancel_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

    let engine = app.state::<DownloadEngine>();

    {

        let mut cancels = engine

            .cancels

            .lock()

            .map_err(|error| log_fail(&title, "cancel registry", error.to_string()))?;

        if cancels.contains_key(&safe_title) {

            return Err(log_fail(&title, "duplicate", String::from("already downloading")));

        }

        if let Ok(active) = engine.torrents.lock() {

            if active.contains_key(&safe_title) {

                return Err(log_fail(&title, "duplicate", String::from("already downloading via torrent lane")));

            }

        }

        cancels.insert(safe_title.clone(), cancel_flag.clone());

    }

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

    let dl_cancel = cancel_flag.clone();

    let pipeline_app = app.clone();

    let pipeline_title = title.clone();

    let pipeline_folder = folder.clone();

    let pipeline_safe_title = safe_title.clone();

    tauri::async_runtime::spawn(async move {

        let url = dl_url.clone();

        let dest_path = dl_dest.clone();

        let done_ref = dl_done.clone();

        let total_ref = dl_total.clone();

        let flag_ref = dl_running.clone();

        let cancel_ref = dl_cancel.clone();

        let result = tokio::task::spawn_blocking(move || {

            let outcome = fix_core::http_download(&url, &dest_path, &cancel_ref, &|bytes_done, bytes_total| {

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

        let engine = pipeline_app.state::<DownloadEngine>();

        if let Ok(mut cancels) = engine.cancels.lock() {

            cancels.remove(&pipeline_safe_title);

        }

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

                if error == "cancelled" {

                    eprintln!("[DL] {}: http download stopped by user", pipeline_title);

                    let _ = std::fs::remove_file(&dl_dest);

                    let _ = pipeline_app.emit("download-progress", DownloadProgress {

                        title: pipeline_title.clone(),

                        downloaded_bytes: 0,

                        total_bytes: 0,

                        state: String::from("stopped"),

                    });

                } else {

                    eprintln!("[DL] {}: http download FAILED: {}", pipeline_title, error);

                    let _ = pipeline_app.emit("download-progress", DownloadProgress {

                        title: pipeline_title.clone(),

                        downloaded_bytes: 0,

                        total_bytes: 0,

                        state: String::from("error"),

                    });

                }

            }

        }

    });

    Ok(mirror_url)

}

