use librqbit::Session;

use librqbit::SessionOptions;

mod commands;
mod helpers;
mod lane;
mod launch;
mod launch_progress;
mod library;
mod pipeline_http;
mod steam_client;
mod pipeline_torrent;
mod state;

#[cfg(test)]
mod test_support;

pub use commands::*;
pub use helpers::*;
pub use lane::*;
pub use launch::*;
pub use library::*;
pub use pipeline_http::*;
pub use pipeline_torrent::*;
pub use state::*;

// WebKitGTK's dmabuf path fails on hybrid/NVIDIA setups ("Failed to create GBM buffer") and the
// window renders fully black. Measured on the 0.2.1 release bundle: black without this, correct with it.
#[cfg(target_os = "linux")]
fn disable_broken_dmabuf_renderer() {

    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_some() {

        return;

    }

    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

}

#[cfg(not(target_os = "linux"))]
fn disable_broken_dmabuf_renderer() {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    disable_broken_dmabuf_renderer();

    let games_dir = helpers::games_root().unwrap_or_else(|| std::env::temp_dir().join("games"));

    std::fs::create_dir_all(&games_dir).expect("create games directory");

    let mut session_opts = SessionOptions::default();

    session_opts.disable_dht_persistence = true;

    let session = tauri::async_runtime::block_on(Session::new_with_opts(games_dir, session_opts))

        .expect("start torrent session");

    tauri::Builder::default()

        .plugin(tauri_plugin_opener::init())

        .plugin(tauri_plugin_dialog::init())

        .manage(DownloadEngine { session, cancels: std::sync::Mutex::new(std::collections::HashMap::new()), torrents: std::sync::Mutex::new(std::collections::HashMap::new()) })

        .invoke_handler(tauri::generate_handler![list_games, find_games, game_detail, game_assets, lane_parts, open_download_window, start_torrent_download, start_http_download, cancel_all_downloads, cancel_download, launch_game, install_plugin, installed_games, open_game_folder, uninstall_game])

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

}
