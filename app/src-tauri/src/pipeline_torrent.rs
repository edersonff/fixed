use bytes::Bytes;
use librqbit::AddTorrent;
use librqbit::AddTorrentOptions;

use crate::DownloadEngine;
use crate::DownloadProgress;
use crate::add_game_to_steam;
use crate::delete_installers;
use crate::extract_first_rar;
use crate::log_fail;
use tauri::Emitter;

#[tauri::command]
pub async fn start_torrent_download(app: tauri::AppHandle, engine: tauri::State<'_, DownloadEngine>, title: String, lane_url: String) -> Result<String, String> {

    let torrent_url = fix_core::torrent_file_url(&lane_url)

        .ok_or_else(|| log_fail(&title, "lane resolve", String::from("no .torrent file found in lane")))?;

    let torrent_bytes = fix_core::fetch_bytes(&torrent_url)

        .map_err(|error| log_fail(&title, "torrent fetch", error))?;

    let safe_title = title.replace('/', "_");

    let folder = crate::game_folder(&safe_title)

        .ok_or_else(|| log_fail(&title, "home dir", String::from("HOME and USERPROFILE unset")))?;

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

