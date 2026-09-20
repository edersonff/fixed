use crate::flog;
use crate::games_root;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledGame {

    pub title: String,

    pub folder: String,

    pub exe: String,

    pub bytes: u64,

    pub has_plugins: bool,

    pub build: Option<String>,

}

fn folder_bytes(dir: &std::path::Path) -> u64 {

    let Ok(entries) = std::fs::read_dir(dir) else {

        return 0;

    };

    entries
        .flatten()
        .map(|entry| match entry.file_type() {

            Ok(kind) if kind.is_dir() => folder_bytes(&entry.path()),

            Ok(_) => entry.metadata().map(|meta| meta.len()).unwrap_or(0),

            Err(_) => 0,

        })
        .sum()

}

fn read_installed(dir: &std::path::Path) -> Option<InstalledGame> {

    let title = dir.file_name()?.to_string_lossy().to_string();

    let folder = dir.to_string_lossy().to_string();

    let exe = fix_core::find_game_exe(&folder)?;

    let has_plugins = std::path::Path::new(&exe)
        .parent()
        .map(|parent| parent.join("BepInEx").join("plugins").is_dir())
        .unwrap_or(false);

    let build = std::fs::read_to_string(dir.join(".fixed-build"))
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    Some(InstalledGame {

        title,

        folder: folder.clone(),

        exe,

        bytes: folder_bytes(dir),

        has_plugins,

        build,

    })

}

#[tauri::command]
pub fn installed_games() -> Vec<InstalledGame> {

    let start = std::time::Instant::now();

    let result = installed_games_scan();

    flog(&format!("[PROF] installed_games {}ms", start.elapsed().as_millis()));

    result

}

fn installed_games_scan() -> Vec<InstalledGame> {

    let Some(root) = games_root() else {

        return Vec::new();

    };

    let Ok(entries) = std::fs::read_dir(root) else {

        return Vec::new();

    };

    let mut games: Vec<InstalledGame> = entries
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| read_installed(&entry.path()))
        .collect();

    games.sort_by(|left, right| left.title.to_lowercase().cmp(&right.title.to_lowercase()));

    games

}

fn is_safe_title(title: &str) -> bool {

    !title.is_empty() && !title.contains("..") && !title.contains('/') && !title.contains('\\')

}

// The folder is confirmed a real directory (not a symlink) by the caller before this runs;
// canonicalize is what catches a games root reached through a different symlinked prefix.
fn require_child_of_games_root(root: &std::path::Path, target: &std::path::Path) -> Result<(), String> {

    let canonical_root = std::fs::canonicalize(root).map_err(|error| format!("canonicalize {}: {}", root.display(), error))?;

    let canonical_target = std::fs::canonicalize(target).map_err(|error| format!("canonicalize {}: {}", target.display(), error))?;

    if canonical_target.parent() != Some(canonical_root.as_path()) {

        return Err(format!("refusing to delete outside games root: {}", target.display()));

    }

    Ok(())

}

fn resolve_uninstall_target(title: &str) -> Result<std::path::PathBuf, String> {

    if !is_safe_title(title) {

        return Err(format!("unsafe title: {}", title));

    }

    let root = games_root().ok_or_else(|| String::from("home dir not found"))?;

    let target = root.join(title);

    let meta = target.symlink_metadata().map_err(|_| format!("game folder not found: {}", target.display()))?;

    if meta.file_type().is_symlink() {

        return Err(format!("refusing to delete a symlink: {}", target.display()));

    }

    if !meta.is_dir() {

        return Err(format!("game folder not found: {}", target.display()));

    }

    require_child_of_games_root(&root, &target)?;

    Ok(target)

}

fn remove_steam_registration(title: &str) -> Result<Vec<String>, String> {

    if crate::steam_client::is_steam_running() {

        return Err(String::from("steam is running, close it before removing this game"));

    }

    let (vdf, root) = crate::launch::steam_paths()?;

    let appid = fix_core::find_shortcut_appid(&vdf, title);

    let mut removed = Vec::new();

    if let Some(appid) = appid {

        if fix_core::remove_compat_tool(&root.to_string_lossy(), appid)? {

            removed.push(String::from("compat tool mapping"));

        }

    }

    if fix_core::remove_steam_shortcut(&vdf, title)? {

        removed.push(String::from("steam shortcut"));

    }

    Ok(removed)

}

#[tauri::command]
pub fn uninstall_game(title: String, remove_from_steam: bool) -> Result<String, String> {

    let target = resolve_uninstall_target(&title)?;

    let mut removed = Vec::new();

    if remove_from_steam {

        removed = remove_steam_registration(&title)?;

    }

    std::fs::remove_dir_all(&target).map_err(|error| format!("remove {}: {}", target.display(), error))?;

    removed.push(format!("game folder {}", target.display()));

    Ok(removed.join(", "))

}

#[tauri::command]
pub fn open_game_folder(folder: String) -> Result<(), String> {

    #[cfg(windows)]
    let opener = "explorer";

    #[cfg(target_os = "macos")]
    let opener = "open";

    #[cfg(all(unix, not(target_os = "macos")))]
    let opener = "xdg-open";

    std::process::Command::new(opener)
        .arg(&folder)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("open {}: {}", folder, error))

}

#[cfg(test)]
#[path = "library_tests.rs"]
mod library_tests;
