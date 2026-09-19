use crate::LaunchProgress;
use tauri::Emitter;

pub(crate) fn emit_progress(app: &tauri::AppHandle, title: &str, phase: &str, detail: &str) {

    let _ = app.emit("launch-progress", LaunchProgress {

        title: title.to_string(),

        phase: phase.to_string(),

        detail: detail.to_string(),

    });

}

// Steam only answers steam:// once its webhelper is up, and that can take tens of seconds after a
// cold start; a frozen "Starting" label with no counter reads as hung, so this re-emits the elapsed
// time roughly every 2s instead of once at the end.
pub(crate) fn wait_with_progress(app: &tauri::AppHandle, title: &str) -> bool {

    let start = std::time::Instant::now();

    let deadline = start + crate::steam_client::READY_TIMEOUT;

    let mut last_emit = start;

    emit_progress(app, title, "waiting-steam", "0s");

    while std::time::Instant::now() < deadline {

        if crate::steam_client::is_steam_ready() {

            return true;

        }

        if last_emit.elapsed() >= std::time::Duration::from_secs(2) {

            emit_progress(app, title, "waiting-steam", &format!("{}s", start.elapsed().as_secs()));

            last_emit = std::time::Instant::now();

        }

        std::thread::sleep(crate::steam_client::POLL);

    }

    false

}
