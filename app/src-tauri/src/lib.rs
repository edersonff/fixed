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
mod quiet_command;
mod steam_client;
pub mod steam_ipc;
mod steam_root;
mod steam_user;
mod user_error;
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

// Windows GUI builds have no console: eprintln alone vanishes, so every diagnostic line also
// lands in ~/.cache/fixed/logs/app.log (USERPROFILE-aware via home_dir). This file is the
// evidence source for crashes and game-level errors reported on the owner's machine.
pub(crate) fn flog(message: &str) {

    eprintln!("{}", message);

    let Some(home) = helpers::home_dir() else {

        return;

    };

    let dir = std::path::PathBuf::from(home).join(".cache/fixed/logs");

    if std::fs::create_dir_all(&dir).is_err() {

        return;

    }

    let secs = std::time::SystemTime::now()

        .duration_since(std::time::UNIX_EPOCH)

        .map(|elapsed| elapsed.as_secs())

        .unwrap_or(0);

    let _ = std::fs::OpenOptions::new()

        .create(true)

        .append(true)

        .open(dir.join("app.log"))

        .and_then(|mut file| std::io::Write::write_all(&mut file, format!("[{}] {}\n", secs, message).as_bytes()));

}

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

    let default_panic = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {

        flog(&format!("[PANIC] {}", info));

        default_panic(info);

    }));

    flog("[BOOT] run entry");

    disable_broken_dmabuf_renderer();

    std::thread::spawn(|| {

        if steam_client::is_steam_running() {

            flog("[STEAM] prewarm: already running");

            return;

        }

        flog("[STEAM] prewarm: starting silent steam in background (app open, not on Play)");

        if let Err(error) = steam_client::start_silent() {

            flog(&format!("[STEAM] prewarm: {}", error));

            return;

        }

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(90);

        while std::time::Instant::now() < deadline {

            if steam_client::is_steam_ready() {

                flog("[STEAM] prewarm: steam ready");

                return;

            }

            std::thread::sleep(std::time::Duration::from_millis(500));

        }

        flog("[STEAM] prewarm: steam not ready within 90s (launch flow will keep waiting)");

    });

    let games_dir = helpers::games_root().unwrap_or_else(|| std::env::temp_dir().join("games"));

    if let Err(error) = std::fs::create_dir_all(&games_dir) {

        flog(&format!("[BOOT] games dir {} not writable (downloads will fail honestly): {}", games_dir.display(), error));

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

                                flog(&format!("[DL] torrent engine ready"));

                            }

                        }

                    }

                    Err(error) => {

                        flog(&format!("[DL] torrent engine failed to start (torrent lane disabled): {}", error));

                    }

                }

            });

            Ok(())

        })

        .invoke_handler(tauri::generate_handler![list_games, find_games, game_detail, game_assets, lane_parts, open_download_window, start_torrent_download, start_http_download, cancel_all_downloads, cancel_download, launch_game, install_plugin, installed_games, open_game_folder, uninstall_game])

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

}
