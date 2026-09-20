use crate::flog;
use crate::find_shortcuts_vdf;

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

// CEF flow: Steam's own client API assigns the appid (Steam-emitted ids are the only ones that
// launch — measured 2026-09-19: FNV and CRC32 ids both die with AppError_9) and RunGame hands the
// game a real session. Same mechanism as Heroic/NonSteamLaunchers/Decky (steamwebhelper CDP).
async fn cef_launch_flow(app: &tauri::AppHandle, title: &str, exe: &str) -> Result<String, String> {

    crate::steam_ipc::ensure_cef_flag()?;

    if !crate::steam_ipc::cef_port_open() {

        if crate::steam_client::is_steam_running() {

            crate::launch_progress::emit_progress(app, title, "stopping-steam", "");

            crate::steam_client::shutdown()?;

        }

        crate::launch_progress::emit_progress(app, title, "starting-steam", "");

        crate::steam_client::start_silent()?;

        if !crate::launch_progress::wait_with_progress(app, title) {

            return Err(String::from("steam did not come up with cef debugging"));

        }

        let mut tries = 0;

        while !crate::steam_ipc::cef_shared_js_ready() && tries < 45 {

            crate::launch_progress::emit_progress(app, title, "waiting-steam", &format!("steam client {}s", tries * 2));

            std::thread::sleep(std::time::Duration::from_secs(2));

            tries += 1;

        }

        if !crate::steam_ipc::cef_shared_js_ready() {

            return Err(String::from("cef debugging target never appeared"));

        }

        std::thread::sleep(std::time::Duration::from_secs(12));

    }

    let (vdf, root) = steam_paths()?;

    let mut canonical: Option<u32> = None;

    for disk_id in fix_core::find_shortcut_appids_by_exe(&vdf, exe) {

        for attempt in 0..3 {

            if crate::steam_ipc::cef_app_known(disk_id).await? {

                canonical = Some(disk_id);

                break;

            }

            if attempt < 2 {

                std::thread::sleep(std::time::Duration::from_secs(3));

            }

        }

        if canonical.is_some() {

            break;

        }

    }

    let appid = match canonical {

        Some(known) => known,

        None => {

            crate::launch_progress::emit_progress(app, title, "registering", "");

            let emitted = crate::steam_ipc::cef_add_shortcut(title, exe).await?;

            crate::steam_ipc::cef_set_launch_options(emitted, fix_core::ONLINE_FIX_LAUNCH_OPTIONS).await?;

            let _ = crate::steam_ipc::cef_remove_shortcut(emitted).await;

            crate::steam_client::shutdown_full()?;

            crate::write_stable_shortcut(title, exe, emitted)?;

            fix_core::ensure_compat_tool(&root.to_string_lossy(), emitted, fix_core::DEFAULT_COMPAT_TOOL)?;

            for orphan in [3496708822u32, 3805198223, 3665257087, 3148280713] {

                let _ = fix_core::remove_compat_tool(&root.to_string_lossy(), orphan);

            }

            crate::launch_progress::emit_progress(app, title, "starting-steam", "");

            crate::steam_client::start_silent()?;

            if !crate::launch_progress::wait_with_progress(app, title) {

                return Err(String::from("steam did not come up after stable registration"));

            }

            let mut tries = 0;

            while !crate::steam_ipc::cef_port_open() && tries < 45 {

                crate::launch_progress::emit_progress(app, title, "waiting-steam", &format!("steam client {}s", tries * 2));

                std::thread::sleep(std::time::Duration::from_secs(2));

                tries += 1;

            }

            std::thread::sleep(std::time::Duration::from_secs(12));

            if !crate::steam_ipc::cef_app_known(emitted).await? {

                return Err(String::from("stable shortcut not recognized after steam reload; restart steam and try again"));

            }

            emitted

        }

    };

    let mapped = fix_core::is_compat_tool_mapped(&root.to_string_lossy(), appid);

    flog(&format!("[LAUNCH] {}: compat tool mapped for appid {}: {}", title, appid, mapped));

    if !mapped {

        if crate::steam_client::is_steam_running() {

            crate::launch_progress::emit_progress(app, title, "stopping-steam", "");

            crate::steam_client::shutdown_full()?;

        }

        crate::launch_progress::emit_progress(app, title, "registering", "");

        fix_core::ensure_compat_tool(&root.to_string_lossy(), appid, fix_core::DEFAULT_COMPAT_TOOL)?;

        flog(&format!("[LAUNCH] {}: compat mapping written for appid {}", title, appid));

        let mut close_tries = 0;

        while crate::steam_ipc::cef_port_open() && close_tries < 15 {

            std::thread::sleep(std::time::Duration::from_secs(1));

            close_tries += 1;

        }

        if crate::steam_ipc::cef_port_open() {

            flog(&format!("[LAUNCH] {}: cef port still open after steam shutdown, restart may race", title));

        }

        crate::launch_progress::emit_progress(app, title, "starting-steam", "");

        crate::steam_client::start_silent()?;

        if !crate::launch_progress::wait_with_progress(app, title) {

            return Err(String::from("steam did not come up after compat mapping"));

        }

        let mut tries = 0;

        while !crate::steam_ipc::cef_port_open() && tries < 45 {

            crate::launch_progress::emit_progress(app, title, "waiting-steam", &format!("steam client {}s", tries * 2));

            std::thread::sleep(std::time::Duration::from_secs(2));

            tries += 1;

        }

        std::thread::sleep(std::time::Duration::from_secs(12));

    }

    crate::launch_progress::emit_progress(app, title, "launching", "");

    let gid = match crate::steam_ipc::cef_run_game(appid).await {

        Ok(gid) => gid,

        Err(error) => {

            flog(&format!("[LAUNCH] {}: CEF RunGame failed ({}), falling back to url", title, error));

            let url = format!("steam://rungameid/{}", fix_core::shortcut_gameid(appid));

            crate::steam_client::open_url(&url)?;

            fix_core::shortcut_gameid(appid)

        }

    };

    flog(&format!("[LAUNCH] {}: RunGame fired for gid {} (appid {}), confirming with steam", title, gid, appid));

    crate::launch_progress::emit_progress(app, title, "launching", "fired, waiting for steam to start it");

    let confirm_app = app.clone();

    let confirm_title = title.to_string();

    let confirm_exe = exe.to_string();

    let pid = tokio::task::spawn_blocking(move || {

        crate::launch_monitor::confirm_and_track(&confirm_app, &confirm_title, appid, &confirm_exe)

    })
        .await
        .map_err(|error| format!("launch monitor join: {}", error))??;

    flog(&format!("[LAUNCH] {}: confirmed running (pid {}, gid {}, appid {})", title, pid, gid, appid));

    crate::launch_progress::emit_progress(app, title, "done", "");

    Ok(format!("launched:{}", gid))

}

async fn legacy_flow(app: &tauri::AppHandle, title: &str, folder: &str) -> Result<String, String> {

    if !crate::steam_client::is_steam_running() {

        crate::launch_progress::emit_progress(app, title, "starting-steam", "");

        crate::steam_client::start_silent()?;

    }

    if !crate::launch_progress::wait_with_progress(app, title) {

        let message = String::from("steam did not come up");

        flog(&format!("[LAUNCH] {}: {}", title, message));

        return Err(message);

    }

    // Webhelper spawns ~10s before login completes, and the URL path has no login signal of its
    // own; firing mid-login is the measured cause of in-game "Steam unavailable" dialogs. The CEF
    // port is the best available proxy (port opened 8-9s before "Welcome to" in cold boots), and
    // without it a fixed settle is the honest floor.
    let mut port_tries = 0;

    while !crate::steam_ipc::cef_port_open() && port_tries < 15 {

        crate::launch_progress::emit_progress(app, title, "waiting-steam", &format!("login settle {}s", port_tries * 2));

        std::thread::sleep(std::time::Duration::from_secs(2));

        port_tries += 1;

    }

    if !crate::steam_ipc::cef_port_open() {

        let message = String::from("steam debug port unavailable; launch could not be verified. restart steam and try again");

        flog(&format!("[LAUNCH] {}: {}", title, message));

        return Err(message);

    }

    std::thread::sleep(std::time::Duration::from_secs(12));

    crate::launch_progress::emit_progress(app, title, "launching", "");

    let exe = fix_core::find_game_exe(folder).ok_or_else(|| String::from("game exe not found"))?;

    let (vdf, _) = steam_paths()?;

    let mut appid = fix_core::find_shortcut_appid_by_exe(&vdf, &exe)

        .or_else(|| fix_core::find_shortcut_appid(&vdf, title))

        .ok_or_else(|| String::from("game not registered in steam yet — close and reopen steam, then press play again"))?;

    if !crate::steam_ipc::cef_app_known(appid).await? {

        flog(&format!("[LAUNCH] {}: appid {} diverged from running steam, restarting steam to reload shortcuts", title, appid));

        crate::steam_client::shutdown()?;

        crate::launch_progress::emit_progress(app, title, "starting-steam", "");

        crate::steam_client::start_silent()?;

        if !crate::launch_progress::wait_with_progress(app, title) {

            return Err(String::from("steam did not come back after reload"));

        }

        let mut tries = 0;

        while !crate::steam_ipc::cef_port_open() && tries < 45 {

            crate::launch_progress::emit_progress(app, title, "waiting-steam", &format!("steam client {}s", tries * 2));

            std::thread::sleep(std::time::Duration::from_secs(2));

            tries += 1;

        }

        std::thread::sleep(std::time::Duration::from_secs(12));

        let (vdf, _) = steam_paths()?;

        appid = fix_core::find_shortcut_appid_by_exe(&vdf, &exe)

            .or_else(|| fix_core::find_shortcut_appid(&vdf, title))

            .ok_or_else(|| String::from("shortcut missing after steam reload"))?;

    }

    let url = format!("steam://rungameid/{}", fix_core::shortcut_gameid(appid));

    let url_pid = crate::steam_client::open_url(&url)?;

    flog(&format!("[LAUNCH] {}: {} (pid {})", title, url, url_pid));

    crate::launch_progress::emit_progress(app, title, "launching", "fired, waiting for steam to start it");

    let confirm_app = app.clone();

    let confirm_title = title.to_string();

    let confirm_exe = exe.to_string();

    let pid = tokio::task::spawn_blocking(move || {

        crate::launch_monitor::confirm_and_track(&confirm_app, &confirm_title, appid, &confirm_exe)

    })

        .await

        .map_err(|error| format!("launch monitor join: {}", error))??;

    flog(&format!("[LAUNCH] {}: confirmed running (pid {}, url {})", title, pid, url));

    crate::launch_progress::emit_progress(app, title, "done", "");

    Ok(format!("launched:{}", url))

}

// Both paths confirm through launch_monitor before reporting `done`: the game process must be
// observed running, otherwise the phase turns `failed` and the card reverts to Play instead of
// waiting forever on a URL Steam silently dropped (measured 2026-09-20: legacy fired, handler pid
// died defunct, card sat on "Waiting for game" indefinitely).
async fn run_launch(app: &tauri::AppHandle, title: &str) -> Result<String, String> {

    crate::launch_progress::emit_progress(app, title, "checking", "");

    let folder = crate::game_folder(title).ok_or_else(|| String::from("home dir not found"))?;

    let Some(exe) = fix_core::find_game_exe(&folder) else {

        let message = format!("no game exe found in {}", folder);

        flog(&format!("[LAUNCH] {}: {}", title, message));

        return Err(message);

    };

    let basename = crate::launch_monitor::exe_basename(&exe);

    if let Some(pid) = crate::game_process::find_game_pid(&basename) {

        flog(&format!("[LAUNCH] {}: already running (pid {})", title, pid));

        crate::launch_progress::emit_progress(app, title, "running", "");

        return Ok(format!("launched:{}", pid));

    }

    match cef_launch_flow(app, title, &exe).await {

        Ok(result) => Ok(result),

        Err(cef_error) => {

            flog(&format!("[LAUNCH] {}: CEF flow failed ({}), falling back to legacy vdf flow", title, cef_error));

            legacy_flow(app, title, &folder).await

        }

    }

}

#[tauri::command]

pub async fn launch_game(app: tauri::AppHandle, title: String) -> Result<String, String> {

    let result = run_launch(&app, &title).await;

    if let Err(error) = &result {

        crate::launch_progress::emit_progress(&app, &title, "failed", error);

    }

    result

}
