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

mod commands;
mod helpers;
mod pipeline_http;
mod pipeline_torrent;

pub use helpers::home_dir;
mod state;

pub use commands::*;
pub use helpers::*;
pub use pipeline_http::*;
pub use pipeline_torrent::*;
pub use state::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let home = helpers::home_dir().unwrap_or_else(|| String::from("/tmp"));

    let games_dir = format!("{}/games", home);

    std::fs::create_dir_all(&games_dir).expect("create ~/games");

    let mut session_opts = SessionOptions::default();

    session_opts.disable_dht_persistence = true;

    let session = tauri::async_runtime::block_on(Session::new_with_opts(games_dir.into(), session_opts))

        .expect("start torrent session");

    tauri::Builder::default()

        .plugin(tauri_plugin_opener::init())

        .plugin(tauri_plugin_dialog::init())

        .manage(DownloadEngine { session, cancels: std::sync::Mutex::new(std::collections::HashMap::new()) })

        .invoke_handler(tauri::generate_handler![list_games, find_games, game_detail, lane_parts, open_download_window, start_torrent_download, start_http_download, cancel_all_downloads, cancel_download, launch_game, install_plugin])

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

}
