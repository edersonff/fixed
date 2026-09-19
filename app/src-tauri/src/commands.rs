use fix_core::discover_parts;
use fix_core::fetch_detail;
use fix_core::search_games;
use fix_core::home_games;
use crate::DownloadEngine;
use crate::DownloadProgress;
use crate::add_game_to_steam;
use crate::delete_installers;
use crate::extract_first_rar;
use crate::find_shortcuts_vdf;
use crate::log_fail;
use serde::Serialize;
use tauri::Emitter;
use tauri::Manager;

pub(crate) const LANE_AUTOClick: &str = r#"

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

#[tauri::command]
pub fn open_download_window(app: tauri::AppHandle, url: String) -> Result<(), String> {

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

pub fn install_plugin(title: String, archive_path: String) -> Result<u32, String> {

    let home = crate::home_dir().ok_or_else(|| String::from("home dir not found"))?;

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

pub fn launch_game(title: String) -> Result<String, String> {

    let home = crate::home_dir().ok_or_else(|| String::from("home dir not found"))?;

    let folder = format!("{}/games/{}", home, title);

    let Some(exe) = fix_core::find_game_exe(&folder) else {

        let message = format!("no game exe found in {}", folder);

        eprintln!("[LAUNCH] {}: {}", title, message);

        return Err(message);

    };

    let mut added_now = false;

    if find_shortcuts_vdf().is_some() {

        added_now = add_game_to_steam(&title, &folder);

    } else {

        eprintln!("[LAUNCH] {}: steam shortcuts.vdf not found", title);

    }

    let appid = find_shortcuts_vdf()

        .and_then(|vdf| fix_core::find_shortcut_appid(&vdf, &title))

        .unwrap_or_else(|| fix_core::shortcut_appid(&exe, &title));

    let url = format!("steam://rungameid/{}", appid);

    #[cfg(windows)]

    let spawn_result = std::process::Command::new("cmd")

        .args(["/C", "start", "", &url])

        .stdin(std::process::Stdio::null())

        .stdout(std::process::Stdio::null())

        .stderr(std::process::Stdio::null())

        .spawn();

    #[cfg(not(windows))]

    let spawn_result = std::process::Command::new("xdg-open")

        .arg(&url)

        .stdin(std::process::Stdio::null())

        .stdout(std::process::Stdio::null())

        .stderr(std::process::Stdio::null())

        .spawn();

    spawn_result

        .map(|child| {

            eprintln!("[LAUNCH] {}: {} (pid {})", title, url, child.id());

            if added_now {

                format!("launched:{};restart", url)

            } else {

                format!("launched:{}", url)

            }

        })

        .map_err(|error| {

            let message = format!("open steam url: {}", error);

            eprintln!("[LAUNCH] {}: {}", title, message);

            message

        })

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

