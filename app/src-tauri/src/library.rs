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

    Some(InstalledGame {

        title,

        folder: folder.clone(),

        exe,

        bytes: folder_bytes(dir),

        has_plugins,

    })

}

#[tauri::command]
pub fn installed_games() -> Vec<InstalledGame> {

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
