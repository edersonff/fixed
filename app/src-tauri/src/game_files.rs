use std::path::Path;

use std::path::PathBuf;

use crate::flog;

const MANIFEST: &str = ".fixed-files";

// Measured 2026-09-26 in the Windows lab: a restored file gets removed again 11-20s after it lands,
// so the watch has to span the full window in short steps rather than one settle-and-check.
const WATCH_STEP: std::time::Duration = std::time::Duration::from_secs(2);

const WATCH_CHECKS: u32 = 10;

#[derive(serde::Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameFilesStatus {
    pub missing: Vec<String>,
    pub can_restore: bool,
}

#[derive(serde::Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RestoreOutcome {
    pub missing: Vec<String>,
    pub removed_again: bool,
    pub can_restore: bool,
}

pub(crate) fn archive_in(folder: &Path) -> Option<PathBuf> {

    let mut rars: Vec<PathBuf> = std::fs::read_dir(folder)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().map(|ext| ext.eq_ignore_ascii_case("rar")).unwrap_or(false))
        .filter(|path| crate::looks_like_rar(path))
        .collect();

    rars.sort_by_key(|path| std::cmp::Reverse(path.metadata().map(|meta| meta.len()).unwrap_or(0)));

    rars.into_iter().next()

}

pub fn download_bytes(folder: &Path) -> u64 {

    archive_in(folder).and_then(|path| path.metadata().ok()).map(|meta| meta.len()).unwrap_or(0)

}

pub(crate) fn missing_entries(folder: &Path, entries: &[String]) -> Vec<String> {

    entries
        .iter()
        .filter(|entry| !folder.join(entry.replace('\\', std::path::MAIN_SEPARATOR_STR)).exists())
        .cloned()
        .collect()

}

fn read_manifest(folder: &Path) -> Vec<String> {

    std::fs::read_to_string(folder.join(MANIFEST))
        .map(|text| text.lines().filter(|line| !line.is_empty()).map(String::from).collect())
        .unwrap_or_default()

}

pub fn missing_files(folder: &Path) -> Vec<String> {

    missing_entries(folder, &read_manifest(folder))

}

// The archive is never deleted for the player: it is the only source a restore can pull removed
// files back from, and the owner ruled there is no reason to delete it as part of this flow at all.
fn after_extract(title: &str, folder: &str) {

    let folder_path = Path::new(folder);

    let Some(archive) = archive_in(folder_path) else {

        return;

    };

    match fix_core::list_archive_entries(&archive.to_string_lossy()) {

        Ok(entries) => {

            if let Err(error) = std::fs::write(folder_path.join(MANIFEST), entries.join("\n")) {

                flog(&format!("[FILES] {}: manifest write failed: {}", title, error));

            }

        }

        Err(error) => flog(&format!("[FILES] {}: archive listing failed: {}", title, error)),

    }

}

pub fn extract_and_verify(title: &str, folder: &str) -> Result<u32, String> {

    let count = crate::extract_first_rar(folder)?;

    after_extract(title, folder);

    Ok(count)

}

pub fn status(folder: &Path) -> GameFilesStatus {

    GameFilesStatus {
        missing: missing_files(folder),
        can_restore: archive_in(folder).is_some(),
    }

}

// Pure: given the files a restore just wrote back and what is missing at a later check, true means
// at least one of them vanished again. This is the only signal that real-time protection is on —
// no system query, ever.
pub(crate) fn files_removed_again(restored: &[String], missing_now: &[String]) -> bool {

    restored.iter().any(|entry| missing_now.contains(entry))

}

pub fn restore(title: &str, folder: &Path) -> Result<RestoreOutcome, String> {

    let missing_before = missing_files(folder);

    let Some(archive) = archive_in(folder) else {

        flog(&format!("[FILES] {}: no archive kept, restore not possible", title));

        return Ok(RestoreOutcome { missing: missing_before, removed_again: false, can_restore: false });

    };

    let count = fix_core::extract_entries(&archive.to_string_lossy(), &folder.to_string_lossy(), &missing_before)?;

    flog(&format!("[FILES] {}: restored {} of {} missing files", title, count, missing_before.len()));

    let missing_after_extract = missing_files(folder);

    let restored: Vec<String> = missing_before
        .iter()
        .filter(|entry| !missing_after_extract.contains(entry))
        .cloned()
        .collect();

    let mut removed_again = false;

    let checks = if cfg!(windows) { WATCH_CHECKS } else { 0 };

    for _ in 0..checks {

        std::thread::sleep(WATCH_STEP);

        if files_removed_again(&restored, &missing_files(folder)) {

            removed_again = true;

        }

    }

    if removed_again {

        flog(&format!("[FILES] {}: restored files vanished again during the watch", title));

    }

    Ok(RestoreOutcome { missing: missing_files(folder), removed_again, can_restore: archive_in(folder).is_some() })

}

#[tauri::command]
pub async fn game_files_status(title: String) -> Result<GameFilesStatus, String> {

    let folder = crate::game_folder(&title).ok_or_else(|| String::from("home dir not found"))?;

    tokio::task::spawn_blocking(move || status(Path::new(&folder)))
        .await
        .map_err(|error| format!("join: {}", error))

}

#[tauri::command]
pub async fn restore_game_files(title: String) -> Result<RestoreOutcome, String> {

    let folder = crate::game_folder(&title).ok_or_else(|| String::from("home dir not found"))?;

    tokio::task::spawn_blocking(move || restore(&title, Path::new(&folder)))
        .await
        .map_err(|error| format!("join: {}", error))?

}

#[tauri::command]
pub async fn delete_game_download(title: String) -> Result<u64, String> {

    let folder = crate::game_folder(&title).ok_or_else(|| String::from("home dir not found"))?;

    tokio::task::spawn_blocking(move || {

        let bytes = download_bytes(Path::new(&folder));

        crate::delete_installers(&folder);

        bytes

    })
        .await
        .map_err(|error| format!("join: {}", error))

}

#[cfg(test)]
#[path = "game_files_tests.rs"]
mod game_files_tests;
