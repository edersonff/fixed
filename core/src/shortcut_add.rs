use crate::backup_vdf;
use crate::find_shortcut_appid;
use crate::find_subslice;
use crate::max_entry_index;
use crate::shortcut_appid;
use crate::vdf_int;
use crate::vdf_string;

pub const ONLINE_FIX_DLL_OVERRIDES: &str = "winhttp=n,b;WINMM=n,b;SteamOverlay64=n,b;steam_api64=n,b";

pub const ONLINE_FIX_LAUNCH_OPTIONS: &str = "WINEDLLOVERRIDES=\"winhttp=n,b;OnlineFix64=n;SteamOverlay64=n;winmm=n,b;dnet=n;steam_api64=n\" %command%";

// WINEDLLOVERRIDES only means something under Wine/Proton; native Windows loads the game-folder
// DLLs by itself.
pub fn launch_options() -> &'static str {

    if cfg!(windows) { "" } else { ONLINE_FIX_LAUNCH_OPTIONS }

}

pub fn add_steam_shortcut(vdf_path: &str, app_name: &str, exe_path: &str, start_dir: &str, launch_options: &str) -> Result<u32, String> {

    let data = std::fs::read(vdf_path).map_err(|error| format!("read {}: {}", vdf_path, error))?;

    let name_marker = vdf_string("AppName", app_name);

    if find_subslice(&data, &name_marker, 0).is_some() {

        if let Some(appid) = find_shortcut_appid(vdf_path, app_name) {

            return Ok(appid);

        }

    }

    let index = max_entry_index(&data) + 1;

    let appid = shortcut_appid(exe_path, app_name);

    let mut entry: Vec<u8> = Vec::new();

    entry.push(0);

    entry.extend_from_slice(index.to_string().as_bytes());

    entry.push(0);

    entry.extend_from_slice(&vdf_int("appid", appid));

    entry.extend_from_slice(&vdf_string("AppName", app_name));

    entry.extend_from_slice(&vdf_string("Exe", &format!("\"{}\"", exe_path)));

    entry.extend_from_slice(&vdf_string("StartDir", start_dir));

    entry.extend_from_slice(&vdf_string("icon", ""));

    entry.extend_from_slice(&vdf_string("ShortcutPath", ""));

    entry.extend_from_slice(&vdf_string("LaunchOptions", launch_options));

    entry.extend_from_slice(&vdf_int("IsHidden", 0));

    entry.extend_from_slice(&vdf_int("AllowDesktopConfig", 1));

    entry.extend_from_slice(&vdf_int("AllowOverlay", 1));

    entry.extend_from_slice(&vdf_int("OpenVR", 0));

    entry.extend_from_slice(&vdf_int("Devkit", 0));

    entry.extend_from_slice(&vdf_string("DevkitGameID", ""));

    entry.extend_from_slice(&vdf_int("DevkitOverrideAppID", 0));

    entry.extend_from_slice(&vdf_int("LastPlayTime", 0));

    entry.extend_from_slice(&vdf_string("FlatpakAppID", ""));

    entry.extend_from_slice(&vdf_string("sortas", ""));

    entry.push(0);

    entry.extend_from_slice(b"tags");

    entry.push(0);

    entry.push(8);

    entry.push(8);

    // Two map terminators close the file (`shortcuts`, then root). Stripping only one would nest
    // the new entry outside `shortcuts`, where Steam's parser never sees it — so both are stripped
    // here and both re-appended after the entry.
    if data.len() < 2 || data[data.len() - 2..] != [8u8, 8u8] {

        return Err(String::from("vdf does not end with two map terminators"));

    }

    let mut out = data[..data.len() - 2].to_vec();

    out.extend_from_slice(&entry);

    out.push(8);

    out.push(8);

    backup_vdf(vdf_path, &data)?;

    std::fs::write(vdf_path, &out).map_err(|error| format!("write {}: {}", vdf_path, error))?;

    let verify = std::fs::read(vdf_path).map_err(|error| format!("re-read {}: {}", vdf_path, error))?;

    if !find_subslice(&verify, &name_marker, 0).is_some() {

        return Err(String::from("verification failed: entry not found after write"));

    }

    Ok(appid)

}

pub fn add_shortcut_with_appid(vdf_path: &str, app_name: &str, exe_path: &str, start_dir: &str, launch_options: &str, appid: u32) -> Result<(), String> {

    let data = std::fs::read(vdf_path).map_err(|error| format!("read {}: {}", vdf_path, error))?;

    let index = max_entry_index(&data) + 1;

    let mut entry: Vec<u8> = Vec::new();

    entry.push(0);

    entry.extend_from_slice(index.to_string().as_bytes());

    entry.push(0);

    entry.extend_from_slice(&vdf_int("appid", appid));

    entry.extend_from_slice(&vdf_string("AppName", app_name));

    entry.extend_from_slice(&vdf_string("Exe", &format!("\"{}\"", exe_path)));

    entry.extend_from_slice(&vdf_string("StartDir", start_dir));

    entry.extend_from_slice(&vdf_string("icon", ""));

    entry.extend_from_slice(&vdf_string("ShortcutPath", ""));

    entry.extend_from_slice(&vdf_string("LaunchOptions", launch_options));

    entry.extend_from_slice(&vdf_int("IsHidden", 0));

    entry.extend_from_slice(&vdf_int("AllowDesktopConfig", 1));

    entry.extend_from_slice(&vdf_int("AllowOverlay", 1));

    entry.extend_from_slice(&vdf_int("OpenVR", 0));

    entry.extend_from_slice(&vdf_int("Devkit", 0));

    entry.extend_from_slice(&vdf_string("DevkitGameID", ""));

    entry.extend_from_slice(&vdf_int("DevkitOverrideAppID", 0));

    entry.extend_from_slice(&vdf_int("LastPlayTime", 0));

    entry.extend_from_slice(&vdf_string("FlatpakAppID", ""));

    entry.extend_from_slice(&vdf_string("sortas", ""));

    entry.push(0);

    entry.extend_from_slice(b"tags");

    entry.push(0);

    entry.push(8);

    entry.push(8);

    if data.len() < 2 || data[data.len() - 2..] != [8u8, 8u8] {

        return Err(String::from("vdf does not end with two map terminators"));

    }

    let mut out = data[..data.len() - 2].to_vec();

    out.extend_from_slice(&entry);

    out.push(8);

    out.push(8);

    backup_vdf(vdf_path, &data)?;

    std::fs::write(vdf_path, &out).map_err(|error| format!("write {}: {}", vdf_path, error))?;

    let verify = std::fs::read(vdf_path).map_err(|error| format!("re-read {}: {}", vdf_path, error))?;

    let appid_marker = vdf_int("appid", appid);

    if find_subslice(&verify, &appid_marker, 0).is_none() {

        return Err(String::from("verification failed: appid not found after write"));

    }

    Ok(())

}

#[cfg(test)]
#[path = "shortcut_add_tests.rs"]
mod shortcut_add_tests;
