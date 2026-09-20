use crate::flog;
use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::Duration;
use std::time::Instant;

const REAPER_TIMEOUT: Duration = Duration::from_secs(15);
const GAME_PID_TIMEOUT: Duration = Duration::from_secs(30);
const FAST_POLL: Duration = Duration::from_millis(500);
const RUNNING_POLL: Duration = Duration::from_secs(5);
const QUICK_EXIT_THRESHOLD: Duration = Duration::from_secs(5);

fn active_titles() -> &'static Mutex<HashSet<String>> {

    static REGISTRY: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

    REGISTRY.get_or_init(|| Mutex::new(HashSet::new()))

}

// One title, one monitor: the guard removes the title from the registry on every exit path
// (early failure or the background watcher finishing), so a second `Play` click while the first
// launch is still being confirmed or the game is still running is rejected instead of racing it.
struct TitleGuard(String);

impl TitleGuard {

    fn acquire(title: &str) -> Option<Self> {

        let mut set = active_titles().lock().ok()?;

        if set.insert(title.to_string()) {

            Some(TitleGuard(title.to_string()))

        } else {

            None

        }

    }

}

impl Drop for TitleGuard {

    fn drop(&mut self) {

        if let Ok(mut set) = active_titles().lock() {

            set.remove(&self.0);

        }

    }

}

fn poll_until<F: FnMut() -> bool>(timeout: Duration, interval: Duration, mut check: F) -> bool {

    let deadline = Instant::now() + timeout;

    while Instant::now() < deadline {

        if check() {

            return true;

        }

        std::thread::sleep(interval);

    }

    false

}

fn poll_until_some<T, F: FnMut() -> Option<T>>(timeout: Duration, interval: Duration, mut check: F) -> Option<T> {

    let deadline = Instant::now() + timeout;

    while Instant::now() < deadline {

        if let Some(value) = check() {

            return Some(value);

        }

        std::thread::sleep(interval);

    }

    None

}

fn exe_basename(exe: &str) -> String {

    std::path::Path::new(exe)
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| exe.to_string())

}

// Blocking: run off the async executor (the caller wraps this in `spawn_blocking`). Confirms the
// launch in two stages Steam can each drop silently — reaper never appearing, or reaper appearing
// but the game exe never starting under it (Proton/runtime failure) — before handing the process
// to a background watcher that reports `exited` whenever it later goes away.
pub(crate) fn confirm_and_track(app: &tauri::AppHandle, title: &str, appid: u32, exe: &str) -> Result<u32, String> {

    let guard = TitleGuard::acquire(title).ok_or_else(|| format!("{} is already being launched", title))?;

    if !poll_until(REAPER_TIMEOUT, FAST_POLL, || crate::game_process::game_launch_started(appid)) {

        let message = format!("no reaper process for appid {} within {}s of RunGame", appid, REAPER_TIMEOUT.as_secs());

        flog(&format!("[LAUNCH] {}: {}", title, message));

        crate::launch_progress::emit_progress(app, title, "failed", &message);

        return Err(message);

    }

    let basename = exe_basename(exe);

    let Some(pid) = poll_until_some(GAME_PID_TIMEOUT, FAST_POLL, || crate::game_process::find_game_pid(&basename)) else {

        let message = format!("steam began the launch but {} never appeared within {}s", basename, GAME_PID_TIMEOUT.as_secs());

        flog(&format!("[LAUNCH] {}: {}", title, message));

        crate::launch_progress::emit_progress(app, title, "failed", &message);

        return Err(message);

    };

    flog(&format!("[LAUNCH] {}: running, pid {} ({})", title, pid, basename));

    crate::launch_progress::emit_progress(app, title, "running", &format!("pid {}", pid));

    #[cfg(not(windows))]
    watch_fix_activation(title.to_string());

    let watch_app = app.clone();

    let watch_title = title.to_string();

    std::thread::spawn(move || watch_exit(&watch_app, &watch_title, pid, guard));

    Ok(pid)

}

fn watch_exit(app: &tauri::AppHandle, title: &str, pid: u32, _guard: TitleGuard) {

    let started = Instant::now();

    while crate::game_process::pid_alive(pid) {

        std::thread::sleep(RUNNING_POLL);

    }

    let ran_for = started.elapsed();

    let detail = format!("pid {} ran {}s", pid, ran_for.as_secs());

    if ran_for < QUICK_EXIT_THRESHOLD {

        flog(&format!("[LAUNCH] {}: exited within {}s of starting, {} — likely failed to run", title, QUICK_EXIT_THRESHOLD.as_secs(), detail));

    } else {

        flog(&format!("[LAUNCH] {}: exited, {}", title, detail));

    }

    crate::launch_progress::emit_progress(app, title, "exited", &detail);

}

#[cfg(not(windows))]
fn fix_site_opened() -> bool {

    let Ok(entries) = std::fs::read_dir("/proc") else {

        return false;

    };

    let needle = b"online-fix.me";

    for entry in entries.flatten() {

        let Ok(cmdline) = std::fs::read(entry.path().join("cmdline")) else {

            continue;

        };

        if cmdline.windows(needle.len()).any(|window| window == needle) {

            return true;

        }

    }

    false

}

#[cfg(not(windows))]
fn watch_fix_activation(title: String) {

    std::thread::spawn(move || {

        for _ in 0..75 {

            std::thread::sleep(std::time::Duration::from_secs(2));

            if fix_site_opened() {

                crate::flog(&format!("[FIX] {}: online-fix site opened by the game, fix active", title));

                return;

            }

        }

        crate::flog(&format!("[FIX] {}: no online-fix site open within 150s of launch", title));

    });

}

#[cfg(test)]
#[path = "launch_monitor_tests.rs"]
mod launch_monitor_tests;
