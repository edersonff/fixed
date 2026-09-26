use crate::flog;
pub fn looks_like_rar(path: &std::path::Path) -> bool {

    let Ok(mut file) = std::fs::File::open(path) else {

        return false;

    };

    use std::io::Read;

    let mut magic = [0u8; 7];

    file.read_exact(&mut magic).is_ok() && &magic[..6] == b"Rar!\x1a\x07"

}

pub fn extract_first_rar(folder: &str) -> Result<u32, String> {

    let mut rars: Vec<std::path::PathBuf> = std::fs::read_dir(folder)

        .map_err(|error| format!("read {}: {}", folder, error))?

        .flatten()

        .map(|entry| entry.path())

        .filter(|path| path.extension().map(|ext| ext == "rar").unwrap_or(false))

        .collect();

    rars.sort_by_key(|path| {

        let size = path.metadata().map(|meta| meta.len()).unwrap_or(0);

        std::cmp::Reverse(size)

    });

    let mut last_error = String::from("no .rar found in download folder");

    for rar in &rars {

        if !looks_like_rar(rar) {

            flog(&format!("[DL] skipping invalid archive: {}", rar.display()));

            continue;

        }

        match fix_core::extract_archive(&rar.to_string_lossy(), folder) {

            Ok(count) => return Ok(count),

            Err(error) => {

                flog(&format!("[DL] extract failed for {}: {}", rar.display(), error));

                last_error = error;

            }

        }

    }

    Err(last_error)

}

pub fn home_dir() -> Option<String> {

    if let Ok(home) = std::env::var("HOME") {

        if !home.is_empty() {

            return Some(home);

        }

    }

    std::env::var("USERPROFILE").ok().filter(|home| !home.is_empty())

}

pub fn games_root() -> Option<std::path::PathBuf> {

    home_dir().map(|home| std::path::PathBuf::from(home).join("games"))

}

const WINDOWS_RESERVED_NAMES: [&str; 22] = [
    "CON", "PRN", "AUX", "NUL",
    "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
    "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

// One folder name on every OS, valid on the strictest (Windows): catalog titles carry ':' and '?'
// ("Subtitle: Edition"), which create_dir_all rejects there.
pub fn safe_title(title: &str) -> String {

    let replaced: String = title
        .chars()
        .map(|character| if "<>:\"/\\|?*".contains(character) || character.is_control() { '_' } else { character })
        .collect();

    let trimmed = replaced.trim_end_matches(['.', ' ']).trim_start();

    let stem = trimmed.split('.').next().unwrap_or("").to_uppercase();

    if trimmed.is_empty() {

        return String::from("_");

    }

    if WINDOWS_RESERVED_NAMES.contains(&stem.as_str()) {

        return format!("{}_", trimmed);

    }

    trimmed.to_string()

}

// Linux installs made before safe_title keep their raw folder (':' was legal there).
pub fn game_folder(title: &str) -> Option<String> {

    let root = games_root()?;

    let legacy = root.join(title);

    let folder = if legacy.is_dir() { legacy } else { root.join(safe_title(title)) };

    Some(folder.to_string_lossy().to_string())

}

pub(crate) fn start_dir_of(exe: &str) -> String {

    std::path::Path::new(exe)
        .parent()
        .map(|parent| format!("{}{}", parent.to_string_lossy(), std::path::MAIN_SEPARATOR))
        .unwrap_or_default()

}

pub fn write_stable_shortcut(title: &str, exe: &str, appid: u32) -> Result<(), String> {

    let vdf = crate::steam_user::shortcuts_vdf()?;

    let _ = fix_core::remove_steam_shortcut(&vdf, title);

    let start_dir = start_dir_of(exe);

    fix_core::add_shortcut_with_appid(&vdf, title, exe, &start_dir, fix_core::launch_options(), appid)?;

    crate::flog(&format!("[STEAM] {}: stable shortcut written, appid {}", title, appid));

    Ok(())

}

pub fn add_game_to_steam(title: &str, folder: &str) -> bool {

    let vdf = match crate::steam_user::shortcuts_vdf() {

        Ok(vdf) => vdf,

        Err(error) => {

            flog(&format!("[DL] {}: not added to Steam: {}", title, error));

            return false;

        }

    };

    if let Some(appid) = fix_core::find_shortcut_appid(&vdf, title) {

        flog(&format!("[DL] {}: already in Steam (appid {})", title, appid));

        return false;

    }

    let Some(exe) = fix_core::find_game_exe(folder) else {

        flog(&format!("[DL] {}: no game exe found in {}", title, folder));

        return false;

    };

    let start_dir = start_dir_of(&exe);

    match fix_core::add_steam_shortcut(&vdf, title, &exe, &start_dir, fix_core::launch_options()) {

        Ok(appid) => {

            flog(&format!("[DL] {}: added to Steam (appid {})", title, appid));

            true

        }

        Err(error) => {

            flog(&format!("[DL] {}: steam shortcut FAILED: {}", title, error));

            false

        }

    }

}

pub fn delete_installers(folder: &str) {

    let Ok(entries) = std::fs::read_dir(folder) else {

        return;

    };

    for entry in entries.flatten() {

        let path = entry.path();

        let is_rar = path.extension().map(|ext| ext == "rar").unwrap_or(false);

        if !is_rar {

            continue;

        }

        match std::fs::remove_file(&path) {

            Ok(()) => flog(&format!("[DL] installer removed: {}", path.display())),

            Err(error) => flog(&format!("[DL] installer remove FAILED: {}: {}", path.display(), error)),

        }

    }

}

pub fn log_fail(title: &str, step: &str, error: String) -> String {

    flog(&format!("[DL] {}: {} FAILED: {}", title, step, error));

    error

}


#[cfg(test)]
#[path = "helpers_tests.rs"]
mod helpers_tests;
