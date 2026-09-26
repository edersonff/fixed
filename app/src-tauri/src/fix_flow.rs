use crate::flog;

const SOURCE_MARKER: &str = ".fixed-source";

fn emit(app: &tauri::AppHandle, title: &str, phase: &str, detail: &str) {

    crate::launch_progress::emit_named(app, "fix-progress", title, phase, detail);

}

fn normalized_title(title: &str) -> String {

    title.chars().filter(|c| c.is_ascii_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect()

}

// A near-miss like "Bombanana Deluxe" against "BOMBANANA!" must never match: the normalized keys
// have to be identical, never a prefix or substring, or a Fix could pull a different game's
// archive down over this install.
pub(crate) fn exact_title_match<'a>(title: &str, games: &'a [fix_core::GameEntry]) -> Option<&'a fix_core::GameEntry> {

    let key = normalized_title(title);

    games.iter().find(|game| normalized_title(&game.title) == key)

}

#[derive(Debug, PartialEq)]
pub(crate) enum FixDecision {
    Restore,
    Redownload { page_url: String },
    GamePageNeeded,
}

// Pure: archive_present and source_marker come from the game folder, exact_match from a title
// search already run by the caller. No filesystem or network call happens in here, so this is the
// part a test can drive with plain inputs.
pub(crate) fn decide_fix(archive_present: bool, source_marker: Option<&str>, exact_match: Option<&fix_core::GameEntry>) -> FixDecision {

    if archive_present {

        return FixDecision::Restore;

    }

    let marked = source_marker.map(str::trim).filter(|url| !url.is_empty());

    if let Some(page_url) = marked {

        return FixDecision::Redownload { page_url: page_url.to_string() };

    }

    match exact_match {

        Some(entry) => FixDecision::Redownload { page_url: entry.page_url.clone() },

        None => FixDecision::GamePageNeeded,

    }

}

fn hosters_lane_from_page(page_url: &str) -> Result<String, String> {

    let detail = fix_core::fetch_detail(page_url);

    detail
        .lanes
        .into_iter()
        .find(|lane| lane.kind == "hosters")
        .map(|lane| lane.url)
        .ok_or_else(|| String::from("no direct download source available for this game"))

}

// The game folder keeps the exact page it was downloaded from (written at download time by both
// pipelines). An install made before that marker existed falls back to an exact title match only —
// never the fuzzy "closest search result" a title can drift away from.
fn resolve_fix(title: &str, folder: &std::path::Path) -> FixDecision {

    let archive_present = crate::game_files::archive_in(folder).is_some();

    let marker = std::fs::read_to_string(folder.join(SOURCE_MARKER)).ok();

    let needs_search = !archive_present && marker.as_deref().map(str::trim).unwrap_or("").is_empty();

    let search_result = if needs_search { Some(fix_core::search_games(title)) } else { None };

    let exact_match = search_result.as_ref().and_then(|page| exact_title_match(title, &page.games));

    decide_fix(archive_present, marker.as_deref(), exact_match)

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

async fn fix_by_redownload(app: tauri::AppHandle, title: String, page_url: String) -> Result<String, String> {

    emit(&app, &title, "checking", "Looking for the download");

    let lookup_page_url = page_url.clone();

    let lane_url = tauri::async_runtime::spawn_blocking(move || hosters_lane_from_page(&lookup_page_url))
        .await
        .map_err(|error| format!("join: {}", error))??;

    crate::pipeline_http::run_http_download(app, title, page_url, lane_url, None).await

}

#[tauri::command]
pub async fn fix_game(app: tauri::AppHandle, title: String) -> Result<String, String> {

    let folder = crate::game_folder(&title).ok_or_else(|| String::from("home dir not found"))?;

    let folder_path = std::path::PathBuf::from(folder);

    let decide_title = title.clone();

    let decide_folder = folder_path.clone();

    let decision = tauri::async_runtime::spawn_blocking(move || resolve_fix(&decide_title, &decide_folder))
        .await
        .map_err(|error| format!("join: {}", error))?;

    match decision {

        FixDecision::Restore => fix_from_archive(app, title, folder_path).await,

        FixDecision::Redownload { page_url } => fix_by_redownload(app, title, page_url).await,

        FixDecision::GamePageNeeded => Err(String::from(crate::user_error::GAME_PAGE_NEEDED)),

    }

}

#[cfg(test)]
#[path = "fix_flow_tests.rs"]
mod fix_flow_tests;
