use crate::flog;

use tauri::Emitter;

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixProgress {
    pub title: String,
    pub phase: String,
    pub detail: String,
}

fn emit(app: &tauri::AppHandle, title: &str, phase: &str, detail: &str) {

    let _ = app.emit("fix-progress", FixProgress { title: title.to_string(), phase: phase.to_string(), detail: detail.to_string() });

}

fn search_key(title: &str) -> String {

    title.chars().filter(|c| c.is_ascii_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect()

}

// The game folder never kept a page url, so a game whose archive is gone is relocated on the site
// by title the same way the library already resolves a card into its detail page.
fn find_hosters_lane(title: &str) -> Result<String, String> {

    let key = search_key(title);

    let games = fix_core::search_games(title);

    let entry = games
        .games
        .iter()
        .find(|game| search_key(&game.title) == key)
        .or_else(|| games.games.first())
        .ok_or_else(|| String::from("could not find this game on the site anymore"))?;

    let detail = fix_core::fetch_detail(&entry.page_url);

    detail
        .lanes
        .into_iter()
        .find(|lane| lane.kind == "hosters")
        .map(|lane| lane.url)
        .ok_or_else(|| String::from("no direct download source available for this game"))

}

async fn fix_from_archive(app: tauri::AppHandle, title: String, folder: std::path::PathBuf) -> Result<String, String> {

    emit(&app, &title, "restoring", "Restoring game files");

    let restore_title = title.clone();

    let outcome = tauri::async_runtime::spawn_blocking(move || crate::game_files::restore(&restore_title, &folder))
        .await
        .map_err(|error| format!("join: {}", error))??;

    if outcome.removed_again {

        flog(&format!("[FIX] {}: files vanished again during the watch", title));

        emit(&app, &title, "removed-again", "");

        return Err(String::from(crate::user_error::FIX_FILES_REMOVED_AGAIN));

    }

    if !outcome.missing.is_empty() {

        flog(&format!("[FIX] {}: {} files still missing after restore", title, outcome.missing.len()));

        emit(&app, &title, "failed", "");

        return Err(String::from("some files could not be restored from the kept download"));

    }

    emit(&app, &title, "done", "");

    Ok(String::from("fixed"))

}

async fn fix_by_redownload(app: tauri::AppHandle, title: String) -> Result<String, String> {

    emit(&app, &title, "checking", "Looking for the download");

    let search_title = title.clone();

    let lane_url = tauri::async_runtime::spawn_blocking(move || find_hosters_lane(&search_title))
        .await
        .map_err(|error| format!("join: {}", error))??;

    crate::pipeline_http::run_http_download(app, title, lane_url, None).await

}

#[tauri::command]
pub async fn fix_game(app: tauri::AppHandle, title: String) -> Result<String, String> {

    let folder = crate::game_folder(&title).ok_or_else(|| String::from("home dir not found"))?;

    if crate::game_files::archive_in(std::path::Path::new(&folder)).is_some() {

        return fix_from_archive(app, title, std::path::PathBuf::from(folder)).await;

    }

    fix_by_redownload(app, title).await

}

#[cfg(test)]
#[path = "fix_flow_tests.rs"]
mod fix_flow_tests;
