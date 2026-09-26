use std::path::Path;

use std::path::PathBuf;

use crate::flog;

const MANIFEST: &str = ".fixed-files";

// Measured 2026-09-26 in the Windows lab: Defender quarantined OnlineFix64.dll and winmm.dll within
// one second of unrar writing them, so a short settle catches the removal before the archive goes.
const SETTLE_CHECKS: u32 = 4;

const SETTLE_STEP: std::time::Duration = std::time::Duration::from_secs(2);

#[derive(serde::Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameFilesStatus {
    pub missing: Vec<String>,
    pub can_restore: bool,
    pub protection_on: Option<bool>,
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

fn settle_and_check(folder: &Path) -> Vec<String> {

    let mut missing = missing_files(folder);

    let checks = if cfg!(windows) { SETTLE_CHECKS } else { 0 };

    for _ in 0..checks {

        std::thread::sleep(SETTLE_STEP);

        missing = missing_files(folder);

    }

    missing

}

#[derive(Debug, PartialEq)]
pub(crate) enum ArchiveDecision {
    Keep,
    Delete,
}

// The archive stays until every listed file is still on disk after the settle; otherwise it is the
// only source the restore can pull the removed files back from.
pub(crate) fn archive_decision(missing: &[String]) -> ArchiveDecision {

    if missing.is_empty() {

        ArchiveDecision::Delete

    } else {

        ArchiveDecision::Keep

    }

}

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

    let missing = settle_and_check(folder_path);

    if archive_decision(&missing) == ArchiveDecision::Delete {

        crate::delete_installers(folder);

        return;

    }

    flog(&format!("[FILES] {}: {} files removed after extraction (antivirus): {}", title, missing.len(), missing.join(", ")));

}

pub fn extract_and_verify(title: &str, folder: &str) -> Result<u32, String> {

    let count = crate::extract_first_rar(folder)?;

    after_extract(title, folder);

    Ok(count)

}

pub fn release_archive_if_intact(title: &str, folder: &str) {

    let missing = settle_and_check(Path::new(folder));

    if archive_decision(&missing) == ArchiveDecision::Delete {

        crate::delete_installers(folder);

        return;

    }

    flog(&format!("[FILES] {}: {} files removed while the game ran: {}", title, missing.len(), missing.join(", ")));

}

pub fn status(folder: &Path) -> GameFilesStatus {

    GameFilesStatus {
        missing: missing_files(folder),
        can_restore: archive_in(folder).is_some(),
        protection_on: crate::defender::realtime_protection_on(),
    }

}

pub fn restore(title: &str, folder: &Path) -> Result<GameFilesStatus, String> {

    let missing = missing_files(folder);

    let archive = archive_in(folder).ok_or_else(|| String::from(crate::user_error::ARCHIVE_GONE))?;

    let count = fix_core::extract_entries(&archive.to_string_lossy(), &folder.to_string_lossy(), &missing)?;

    flog(&format!("[FILES] {}: restored {} of {} missing files", title, count, missing.len()));

    let after = settle_and_check(folder);

    if archive_decision(&after) == ArchiveDecision::Delete {

        crate::delete_installers(&folder.to_string_lossy());

    }

    Ok(GameFilesStatus {
        missing: after,
        can_restore: archive_in(folder).is_some(),
        protection_on: crate::defender::realtime_protection_on(),
    })

}

#[tauri::command]
pub async fn game_files_status(title: String) -> Result<GameFilesStatus, String> {

    let folder = crate::game_folder(&title).ok_or_else(|| String::from("home dir not found"))?;

    tokio::task::spawn_blocking(move || status(Path::new(&folder)))
        .await
        .map_err(|error| format!("join: {}", error))

}

#[tauri::command]
pub async fn restore_game_files(title: String) -> Result<GameFilesStatus, String> {

    let folder = crate::game_folder(&title).ok_or_else(|| String::from("home dir not found"))?;

    tokio::task::spawn_blocking(move || restore(&title, Path::new(&folder)))
        .await
        .map_err(|error| format!("join: {}", error))?

}

#[cfg(test)]
#[path = "game_files_tests.rs"]
mod game_files_tests;
