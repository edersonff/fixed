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

        while !crate::steam_ipc::cef_port_open() && tries < 10 {

            std::thread::sleep(std::time::Duration::from_secs(1));

            tries += 1;

        }

        if !crate::steam_ipc::cef_port_open() {

            return Err(String::from("cef debugging port never opened"));

        }

    }

    let (vdf, root) = steam_paths()?;

    let mut appid = fix_core::find_shortcut_appid_by_exe(&vdf, exe);

    if let Some(found) = appid {

        if !crate::steam_ipc::cef_app_known(found).await? {

            eprintln!("[LAUNCH] {}: appid {} unknown to running steam, re-registering", title, found);

            appid = None;

        }

    }

    if appid.is_none() {

        crate::launch_progress::emit_progress(app, title, "registering", "");

        appid = Some(crate::steam_ipc::cef_add_shortcut(title, exe).await?);

        let new_appid = appid.unwrap_or_default();

        crate::steam_ipc::cef_set_launch_options(new_appid, fix_core::ONLINE_FIX_LAUNCH_OPTIONS).await?;

    }

    let appid = appid.ok_or_else(|| String::from("no shortcut appid"))?;

    if !fix_core::is_compat_tool_mapped(&root.to_string_lossy(), appid) {

        if crate::steam_client::is_steam_running() {

            crate::launch_progress::emit_progress(app, title, "stopping-steam", "");

            crate::steam_client::shutdown()?;

        }

        crate::launch_progress::emit_progress(app, title, "registering", "");

        fix_core::ensure_compat_tool(&root.to_string_lossy(), appid, fix_core::DEFAULT_COMPAT_TOOL)?;

        crate::launch_progress::emit_progress(app, title, "starting-steam", "");

        crate::steam_client::start_silent()?;

        if !crate::launch_progress::wait_with_progress(app, title) {

            return Err(String::from("steam did not come up after compat mapping"));

        }

        let mut tries = 0;

        while !crate::steam_ipc::cef_port_open() && tries < 10 {

            std::thread::sleep(std::time::Duration::from_secs(1));

            tries += 1;

        }

    }

    crate::launch_progress::emit_progress(app, title, "launching", "");

    let gid = match crate::steam_ipc::cef_run_game(appid).await {

        Ok(gid) => gid,

        Err(error) => {

            eprintln!("[LAUNCH] {}: CEF RunGame failed ({}), falling back to url", title, error);

            let url = format!("steam://rungameid/{}", fix_core::shortcut_gameid(appid));

            crate::steam_client::open_url(&url)?;

            fix_core::shortcut_gameid(appid)

        }

    };

    eprintln!("[LAUNCH] {}: RunGame fired for gid {} (appid {}), confirming with steam", title, gid, appid);

    crate::launch_progress::emit_progress(app, title, "launching", "fired, waiting for steam to start it");

    let confirm_app = app.clone();

    let confirm_title = title.to_string();

    let confirm_exe = exe.to_string();

    let pid = tokio::task::spawn_blocking(move || {

        crate::launch_monitor::confirm_and_track(&confirm_app, &confirm_title, appid, &confirm_exe)

    })
        .await
        .map_err(|error| format!("launch monitor join: {}", error))??;

    eprintln!("[LAUNCH] {}: confirmed running (pid {}, gid {}, appid {})", title, pid, gid, appid);

    crate::launch_progress::emit_progress(app, title, "done", &gid.to_string());

    Ok(format!("launched:{}", gid))

}

fn legacy_flow(app: &tauri::AppHandle, title: &str, folder: &str) -> Result<String, String> {

    if !crate::steam_client::is_steam_running() {

        crate::launch_progress::emit_progress(app, title, "starting-steam", "");

        crate::steam_client::start_silent()?;

    }

    if !crate::launch_progress::wait_with_progress(app, title) {

        let message = String::from("steam did not come up");

        eprintln!("[LAUNCH] {}: {}", title, message);

        return Err(message);

    }

    crate::launch_progress::emit_progress(app, title, "launching", "");

    let (vdf, _) = steam_paths()?;

    let exe = fix_core::find_game_exe(folder).ok_or_else(|| String::from("game exe not found"))?;

    let appid = fix_core::find_shortcut_appid_by_exe(&vdf, &exe)

        .or_else(|| fix_core::find_shortcut_appid(&vdf, title))

        .ok_or_else(|| String::from("game not registered in steam yet — close and reopen steam, then press play again"))?;

    let url = format!("steam://rungameid/{}", fix_core::shortcut_gameid(appid));

    let pid = crate::steam_client::open_url(&url)?;

    eprintln!("[LAUNCH] {}: {} (pid {})", title, url, pid);

    crate::launch_progress::emit_progress(app, title, "done", &url);

    Ok(format!("launched:{}", url))

}

// The CEF path's `done` means the game process was observed running (see launch_monitor). The
// legacy vdf fallback below has no CDP session to confirm through, so its `done` only means the
// steam:// request reached Steam.
async fn run_launch(app: &tauri::AppHandle, title: &str) -> Result<String, String> {

    crate::launch_progress::emit_progress(app, title, "checking", "");

    let folder = crate::game_folder(title).ok_or_else(|| String::from("home dir not found"))?;

    let Some(exe) = fix_core::find_game_exe(&folder) else {

        let message = format!("no game exe found in {}", folder);

        eprintln!("[LAUNCH] {}: {}", title, message);

        return Err(message);

    };

    match cef_launch_flow(app, title, &exe).await {

        Ok(result) => Ok(result),

        Err(cef_error) => {

            eprintln!("[LAUNCH] {}: CEF flow failed ({}), falling back to legacy vdf flow", title, cef_error);

            legacy_flow(app, title, &folder)

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
