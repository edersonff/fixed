use crate::find_shortcuts_vdf;

#[tauri::command]

pub fn install_plugin(title: String, archive_path: String) -> Result<u32, String> {


    let folder = crate::game_folder(&title).ok_or_else(|| String::from("home dir not found"))?;

    let Some(exe) = fix_core::find_game_exe(&folder) else {

        let message = format!("no game exe found in {}", folder);

        eprintln!("[PLUGIN] {}: {}", title, message);

        return Err(message);

    };

    let game_dir = std::path::Path::new(&exe)

        .parent()

        .map(|parent| parent.to_string_lossy().to_string())

        .unwrap_or(folder.clone());

    let bep_core = std::path::Path::new(&game_dir).join("BepInEx").join("core");

    if !bep_core.exists() {

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

// Steam is the only thing that can hand the game a real session: `reaper SteamLaunch AppId=...`
// registers the process with the running client, which is what makes online-fix's replaced
// steam_api64 find the client socket. A bare Proton wrapper starts the game with zero Steam env
// (measured 2026-09-19: `grep -c "^Steam" /proc/<pid>/environ` = 0) and the networking — the whole
// product — is silently dead. So we never launch the executable ourselves.
//
// Steam reads shortcuts.vdf and config.vdf only at startup and rewrites shortcuts.vdf from memory on
// exit, so registering a game costs one client restart. `-silent` makes that restart invisible.
pub(crate) fn steam_paths() -> Result<(String, std::path::PathBuf), String> {

    let vdf = find_shortcuts_vdf().ok_or_else(|| String::from("steam shortcuts.vdf not found"))?;

    let root = crate::steam_client::steam_root().ok_or_else(|| String::from("steam root not found"))?;

    Ok((vdf, root))

}

fn is_registered(title: &str) -> Result<bool, String> {

    let (vdf, root) = steam_paths()?;

    let Some(appid) = fix_core::find_shortcut_appid(&vdf, title) else {

        return Ok(false);

    };

    Ok(fix_core::is_compat_tool_mapped(&root.to_string_lossy(), appid))

}

fn register(app: &tauri::AppHandle, title: &str, folder: &str) -> Result<(), String> {

    let was_running = crate::steam_client::is_steam_running();

    if was_running {

        crate::launch_progress::emit_progress(app, title, "stopping-steam", "");

        crate::steam_client::shutdown()?;

    }

    crate::launch_progress::emit_progress(app, title, "registering", "");

    crate::add_game_to_steam(title, folder);

    let (vdf, root) = steam_paths()?;

    let appid = fix_core::find_shortcut_appid(&vdf, title)

        .ok_or_else(|| String::from("shortcut not found after write"))?;

    fix_core::ensure_compat_tool(&root.to_string_lossy(), appid, fix_core::DEFAULT_COMPAT_TOOL)?;

    eprintln!("[STEAM] {}: registered as appid {}", title, appid);

    crate::launch_progress::emit_progress(app, title, "starting-steam", "");

    crate::steam_client::start_silent()?;

    Ok(())

}

fn ensure_registered_and_running(app: &tauri::AppHandle, title: &str, folder: &str) -> Result<(), String> {

    if !is_registered(title)? {

        register(app, title, folder)?;

    } else if !crate::steam_client::is_steam_running() {

        crate::launch_progress::emit_progress(app, title, "starting-steam", "");

        crate::steam_client::start_silent()?;

    }

    Ok(())

}

// `done` only means the URL reached Steam, never that the game process is up — nothing on this
// side can observe the game itself starting.
fn run_launch(app: &tauri::AppHandle, title: &str) -> Result<String, String> {

    crate::launch_progress::emit_progress(app, title, "checking", "");

    let folder = crate::game_folder(title).ok_or_else(|| String::from("home dir not found"))?;

    if fix_core::find_game_exe(&folder).is_none() {

        let message = format!("no game exe found in {}", folder);

        eprintln!("[LAUNCH] {}: {}", title, message);

        return Err(message);

    }

    ensure_registered_and_running(app, title, &folder)?;

    if !crate::launch_progress::wait_with_progress(app, title) {

        let message = String::from("steam did not come up");

        eprintln!("[LAUNCH] {}: {}", title, message);

        return Err(message);

    }

    crate::launch_progress::emit_progress(app, title, "launching", "");

    let (vdf, _) = steam_paths()?;

    let appid = fix_core::find_shortcut_appid(&vdf, title)

        .ok_or_else(|| String::from("shortcut missing after register"))?;

    let url = format!("steam://rungameid/{}", fix_core::shortcut_gameid(appid));

    let pid = crate::steam_client::open_url(&url)?;

    eprintln!("[LAUNCH] {}: {} (pid {})", title, url, pid);

    crate::launch_progress::emit_progress(app, title, "done", &url);

    Ok(format!("launched:{}", url))

}

#[tauri::command]

pub fn launch_game(app: tauri::AppHandle, title: String) -> Result<String, String> {

    let result = run_launch(&app, &title);

    if let Err(error) = &result {

        crate::launch_progress::emit_progress(&app, &title, "failed", error);

    }

    result

}
