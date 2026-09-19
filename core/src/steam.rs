use crate::*;

pub const ONLINE_FIX_LAUNCH_OPTIONS: &str = "WINEDLLOVERRIDES=\"winhttp=n,b;WINMM=n,b;SteamOverlay64=n,b;steam_api64=n,b\" %command%";

pub fn shortcut_appid(exe_path: &str, app_name: &str) -> u32 {

    let mut hash: u32 = 2166136261;

    for byte in exe_path.bytes().chain(app_name.bytes()) {

        hash ^= byte as u32;

        hash = hash.wrapping_mul(16777619);

    }

    hash | 0x80000000

}

pub fn find_shortcut_appid(vdf_path: &str, app_name: &str) -> Option<u32> {

    let data = std::fs::read(vdf_path).ok()?;

    let name_marker = vdf_string("AppName", app_name);

    let found = find_subslice(&data, &name_marker, 0)?;

    let seg_start = found.saturating_sub(160);

    let seg = &data[seg_start..found];

    let marker = b"\x02appid\x00";

    let idx = find_subslice(seg, marker, 0)?;

    let value_start = idx + marker.len();

    if value_start + 4 > seg.len() {

        return None;

    }

    let bytes: [u8; 4] = [

        seg[value_start],

        seg[value_start + 1],

        seg[value_start + 2],

        seg[value_start + 3],

    ];

    Some(u32::from_le_bytes(bytes))

}

fn vdf_string(key: &str, value: &str) -> Vec<u8> {

    let mut out = vec![1u8];

    out.extend_from_slice(key.as_bytes());

    out.push(0);

    out.extend_from_slice(value.as_bytes());

    out.push(0);

    out

}

fn vdf_int(key: &str, value: u32) -> Vec<u8> {

    let mut out = vec![2u8];

    out.extend_from_slice(key.as_bytes());

    out.push(0);

    out.extend_from_slice(&value.to_le_bytes());

    out

}

fn entry_index_at(data: &[u8], marker_start: usize) -> Option<u32> {

    let digits_end = marker_start.checked_sub(1)?;

    if data.get(digits_end)? != &0u8 {

        return None;

    }

    let mut start = digits_end;

    while start > 0 && data[start - 1].is_ascii_digit() {

        start -= 1;

    }

    if start == digits_end || start == 0 || data[start - 1] != 0u8 {

        return None;

    }

    std::str::from_utf8(&data[start..digits_end]).ok()?.parse().ok()

}

fn max_entry_index(data: &[u8]) -> u32 {

    let marker: &[u8] = b"\x02appid\x00";

    let mut max: u32 = 0;

    let mut pos = 0;

    while let Some(found) = find_subslice(data, marker, pos) {

        if let Some(index) = entry_index_at(data, found) {

            if index > max {

                max = index;

            }

        }

        pos = found + marker.len();

    }

    max

}

fn find_subslice(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {

    if needle.is_empty() || from >= haystack.len() {

        return None;

    }

    haystack[from..]

        .windows(needle.len())

        .position(|window| window == needle)

        .map(|offset| offset + from)

}

pub fn find_game_exe(folder: &str) -> Option<String> {

    const EXCLUSIONS: [&str; 9] = [

        "unitycrashhandler",

        "vc_redist",

        "vcredist",

        "dxsetup",

        "dotnet",

        "unins000",

        "launchersetting",

        "redist",

        "settings",

    ];

    fn walk(dir: &Path, depth: u8, candidates: &mut Vec<(bool, String)>) {

        if depth == 0 {

            return;

        }

        if let Ok(entries) = std::fs::read_dir(dir) {

            for entry in entries.flatten() {

                let path = entry.path();

                if path.is_dir() {

                    walk(&path, depth - 1, candidates);

                } else {

                    let name = path.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();

                    if !name.ends_with(".exe") {

                        continue;

                    }

                    if EXCLUSIONS.iter().any(|exclusion| name.contains(exclusion)) {

                        continue;

                    }

                    let stem = name.trim_end_matches(".exe");

                    let parent = path.parent()

                        .and_then(|p| p.file_name())

                        .map(|n| n.to_string_lossy().to_lowercase())

                        .unwrap_or_default();

                    candidates.push((stem == parent, path.to_string_lossy().to_string()));

                }

            }

        }

    }

    let mut candidates: Vec<(bool, String)> = Vec::new();

    walk(Path::new(folder), 4, &mut candidates);

    candidates.sort_by(|a, b| b.0.cmp(&a.0));

    candidates.first().map(|(_, path)| path.clone())

}

// shortcuts.vdf holds every shortcut the user ever made by hand; a bad write loses all of them and
// Steam offers no undo. The rollback copy is written before the mutation or the mutation is refused.
fn backup_vdf(vdf_path: &str, original: &[u8]) -> Result<(), String> {

    let backup_path = format!("{}.bak-fixed", vdf_path);

    std::fs::write(&backup_path, original)

        .map_err(|error| format!("backup {}: {}", backup_path, error))

}

pub fn add_steam_shortcut(vdf_path: &str, app_name: &str, exe_path: &str, start_dir: &str, launch_options: &str) -> Result<u32, String> {

    let data = std::fs::read(vdf_path).map_err(|error| format!("read {}: {}", vdf_path, error))?;

    let name_marker = vdf_string("AppName", app_name);

    if let Some(found) = find_subslice(&data, &name_marker, 0) {

        if let Some(index) = find_subslice(&data, b"\x02appid\x00", found.saturating_sub(64))

            .and_then(|marker| entry_index_at(&data, marker))

        {

            return Ok(index);

        }

    }

    let index = max_entry_index(&data) + 1;

    let mut entry: Vec<u8> = Vec::new();

    entry.push(0);

    entry.extend_from_slice(index.to_string().as_bytes());

    entry.push(0);

    entry.extend_from_slice(&vdf_int("appid", shortcut_appid(exe_path, app_name)));

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

    if data.last() != Some(&8u8) {

        return Err(String::from("vdf does not end with map terminator"));

    }

    let mut out = data[..data.len() - 1].to_vec();

    out.extend_from_slice(&entry);

    out.push(8);

    backup_vdf(vdf_path, &data)?;

    std::fs::write(vdf_path, &out).map_err(|error| format!("write {}: {}", vdf_path, error))?;

    let verify = std::fs::read(vdf_path).map_err(|error| format!("re-read {}: {}", vdf_path, error))?;

    if !find_subslice(&verify, &name_marker, 0).is_some() {

        return Err(String::from("verification failed: entry not found after write"));

    }

    Ok(index)

}

#[cfg(test)]
#[path = "steam_tests.rs"]
mod steam_tests;
