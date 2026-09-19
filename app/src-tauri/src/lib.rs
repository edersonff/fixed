use librqbit::Session;

use librqbit::SessionOptions;

use tauri::Manager;

mod commands;
mod game_process;
mod helpers;
mod lane;
mod launch;
mod launch_monitor;
mod launch_progress;
mod library;
mod pipeline_http;
mod plugin;
mod steam_client;
mod steam_ipc;
mod steam_root;
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
pub use plugin::*;
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

    eprintln!("[BOOT] run entry");

    disable_broken_dmabuf_renderer();

    let games_dir = helpers::games_root().unwrap_or_else(|| std::env::temp_dir().join("games"));

    if let Err(error) = std::fs::create_dir_all(&games_dir) {

        eprintln!("[BOOT] games dir {} not writable (downloads will fail honestly): {}", games_dir.display(), error);

    }

    let mut session_opts = SessionOptions::default();

    session_opts.disable_dht_persistence = true;

    tauri::Builder::default()

        .plugin(tauri_plugin_opener::init())

        .plugin(tauri_plugin_dialog::init())

        .manage(DownloadEngine { session: std::sync::Mutex::new(None), cancels: std::sync::Mutex::new(std::collections::HashMap::new()), torrents: std::sync::Mutex::new(std::collections::HashMap::new()) })

        .setup(|app| {

            let open = tauri::menu::MenuItem::with_id(app, "open", "Open FIXED", true, None::<&str>)?;

            let quit = tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = tauri::menu::Menu::with_items(app, &[&open, &quit])?;

            tauri::tray::TrayIconBuilder::with_id("main")

                .icon(app.default_window_icon().cloned().ok_or_else(|| String::from("no app icon"))?)

                .menu(&menu)

                .show_menu_on_left_click(false)

                .on_menu_event(|app, event| {

                    if event.id() == "open" {

                        if let Some(window) = app.get_webview_window("main") {

                            let _ = window.show();

                            let _ = window.unminimize();

                            let _ = window.set_focus();

                        }

                    }

                    if event.id() == "quit" {

                        app.exit(0);

                    }

                })

                .on_tray_icon_event(|tray, event| {

                    if let tauri::tray::TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {

                        let app = tray.app_handle().clone();

                        if let Some(window) = app.get_webview_window("main") {

                            let _ = window.show();

                            let _ = window.unminimize();

                            let _ = window.set_focus();

                        }

                    }

                })

                .build(app)?;

            let handle = app.handle().clone();

            std::thread::spawn(move || {

                let games_dir = helpers::games_root().unwrap_or_else(|| std::env::temp_dir().join("games"));

                let mut session_opts = SessionOptions::default();

                session_opts.disable_dht_persistence = true;

                match tauri::async_runtime::block_on(Session::new_with_opts(games_dir, session_opts)) {

                    Ok(session) => {

                        if let Some(engine) = handle.try_state::<DownloadEngine>() {

                            if let Ok(mut slot) = engine.session.lock() {

                                *slot = Some(session);

                                eprintln!("[DL] torrent engine ready");

                            }

                        }

                    }

                    Err(error) => {

                        eprintln!("[DL] torrent engine failed to start (torrent lane disabled): {}", error);

                    }

                }

            });

            Ok(())

        })

        .invoke_handler(tauri::generate_handler![list_games, find_games, game_detail, game_assets, lane_parts, open_download_window, start_torrent_download, start_http_download, cancel_all_downloads, cancel_download, launch_game, install_plugin, installed_games, open_game_folder, uninstall_game])

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

}
