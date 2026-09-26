#[cfg(windows)]
use std::path::PathBuf;

use std::process::Command;

use std::process::Stdio;

use std::time::Duration;

use std::time::Instant;

pub(crate) const READY_TIMEOUT: Duration = Duration::from_secs(45);

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) const POLL: Duration = Duration::from_millis(500);

#[cfg(windows)]
const STEAM_PROCESS: &str = "steam.exe";

#[cfg(windows)]
const STEAM_WEBHELPER_PROCESS: &str = "steamwebhelper.exe";

#[cfg(not(windows))]
const STEAM_PROCESS: &str = "steam";

#[cfg(not(windows))]
const STEAM_WEBHELPER_PROCESS: &str = "steamwebhelper";

pub use crate::steam_root::steam_root;

#[cfg(not(windows))]
fn process_alive(name: &str) -> bool {

    crate::quiet_command::quiet_command("pgrep")
        .args(["-x", name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)

}

// `tasklist` exits 0 even when nothing matches the filter, printing "INFO: No tasks are running
// which match the specified criteria." instead — the exit code alone cannot tell a match from a
// miss, so the decision reads the output text.
#[cfg(windows)]
fn process_alive(image_name: &str) -> bool {

    let filter = format!("IMAGENAME eq {}", image_name);

    crate::quiet_command::quiet_command("tasklist")
        .args(["/FI", &filter, "/NH"])
        .output()
        .map(|output| tasklist_reports_running(&String::from_utf8_lossy(&output.stdout), image_name))
        .unwrap_or(false)

}

#[cfg(any(windows, test))]
pub(crate) fn tasklist_reports_running(output: &str, image_name: &str) -> bool {

    let needle = image_name.to_lowercase();

    output
        .lines()
        .any(|line| !line.starts_with("INFO:") && line.to_lowercase().contains(&needle))

}

pub fn is_steam_running() -> bool {

    process_alive(STEAM_PROCESS)

}

// The client answers steam:// only once its webhelper is up; a URL fired at a cold client is
// consumed as a startup argument instead (measured 2026-09-19: Steam booted, pulled a 190MB update,
// showed the dash, launched nothing). So readiness is the webhelper, not the steam process.
pub fn is_steam_ready() -> bool {

    is_steam_running() && process_alive(STEAM_WEBHELPER_PROCESS)

}

// Steam spawned from a bundled app (AppImage/deb) inherits the bundle's env; with those entries
// Steam configures compat sessions but every child spawn dies before exec (measured 2026-09-20:
// sessions released in 3-14s, zero execve, no proton log; same shortcuts launch fine under a
// desktop-started Steam). Vectors measured: LD_LIBRARY_PATH (webview libs) AND AppRun exports
// like PYTHONHOME/PYTHONPATH pointing into the bundle, which kill Proton's python wrapper at
// spawn. So: LD drops whole (Steam rebuilds its own), and ANY var whose value carries a bundle
// path drops too; PATH only loses the bundle entries (Steam needs a working PATH).
#[cfg(not(windows))]
fn bundle_markers() -> Vec<String> {

    let mut markers = vec![String::from("squashfs-root"), String::from(".mount_"), String::from("AppDir")];

    if let Some(dir) = std::env::current_exe().ok().and_then(|exe| exe.parent().map(|d| d.to_path_buf())) {

        let text = dir.to_string_lossy().into_owned();

        // a deb install lives in /usr/bin, which as a marker would strip /usr/bin itself from PATH

        let is_system_dir = ["/usr/bin", "/bin", "/usr/local/bin", "/usr/sbin", "/sbin"].contains(&text.as_str());

        if !is_system_dir {

            markers.push(text);

        }

    }

    markers

}

#[cfg(not(windows))]
pub(crate) fn strip_bundle_paths(value: &str, markers: &[String]) -> String {

    let kept: Vec<std::path::PathBuf> = std::env::split_paths(value)

        .filter(|entry| {

            if entry.as_os_str().is_empty() {

                return false;

            }

            let text = entry.to_string_lossy();

            !markers.iter().any(|marker| text.contains(marker.as_str()))

        })

        .collect();

    std::env::join_paths(kept)

        .map(|joined| joined.into_string().unwrap_or_default())

        .unwrap_or_default()

}

#[cfg(not(windows))]
pub(crate) fn carries_bundle_path(value: &str, markers: &[String]) -> bool {

    markers.iter().any(|marker| value.contains(marker.as_str()))

}

#[cfg(not(windows))]
fn sanitize_env(command: &mut Command) {

    command.env_remove("LD_LIBRARY_PATH");

    command.env_remove("LD_PRELOAD");

    let markers = bundle_markers();

    for (key, value) in std::env::vars_os() {

        let Some(text) = value.to_str() else { continue };

        let Some(name) = key.to_str() else { continue };

        if name != "PATH" && carries_bundle_path(text, &markers) {

            command.env_remove(name);

        }

    }

    if let Ok(path) = std::env::var("PATH") {

        let clean = strip_bundle_paths(&path, &markers);

        command.env("PATH", if clean.is_empty() { String::from("/usr/local/bin:/usr/bin:/bin") } else { clean });

    }

}

fn spawn_steam(command: &mut Command) -> Result<(), String> {

    #[cfg(not(windows))]

    sanitize_env(command);

    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("start steam: {}", error))

}

fn wait_for_shutdown() -> Result<(), String> {

    let deadline = Instant::now() + SHUTDOWN_TIMEOUT;

    while Instant::now() < deadline {

        if !is_steam_running() {

            return Ok(());

        }

        std::thread::sleep(POLL);

    }

    Err(String::from("steam did not exit"))

}

fn run_shutdown(command: &mut Command) -> Result<(), String> {

    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| format!("steam -shutdown: {}", error))?;

    wait_for_shutdown()

}

#[cfg(windows)]
fn steam_executable() -> Result<PathBuf, String> {

    steam_root()
        .map(|root| root.join("steam.exe"))
        .ok_or_else(|| String::from(crate::user_error::STEAM_NOT_INSTALLED))

}

#[cfg(windows)]
pub fn start_silent() -> Result<(), String> {

    let exe = steam_executable()?;

    let mut command = crate::quiet_command::quiet_command(exe);

    command.arg("-silent");

    spawn_steam(&mut command)

}

#[cfg(not(windows))]
fn steam_command() -> Command {

    let flatpak = steam_root().map(|root| crate::steam_root::is_flatpak_steam(&root)).unwrap_or(false);

    if !flatpak {

        return crate::quiet_command::quiet_command("steam");

    }

    let mut command = crate::quiet_command::quiet_command("flatpak");

    command.args(["run", "com.valvesoftware.Steam"]);

    command

}

#[cfg(not(windows))]
pub fn start_silent() -> Result<(), String> {

    let mut command = steam_command();

    command.arg("-silent");

    spawn_steam(&mut command)

}

#[cfg(windows)]
pub fn shutdown() -> Result<(), String> {

    let exe = steam_executable()?;

    let mut command = crate::quiet_command::quiet_command(exe);

    command.arg("-shutdown");

    run_shutdown(&mut command)

}

#[cfg(not(windows))]
pub fn shutdown() -> Result<(), String> {

    let mut command = steam_command();

    command.arg("-shutdown");

    run_shutdown(&mut command)

}

pub fn shutdown_full() -> Result<(), String> {

    shutdown()?;

    let mut tries = 0;

    while is_steam_running() && tries < 25 {

        std::thread::sleep(std::time::Duration::from_secs(1));

        tries += 1;

    }

    if is_steam_running() {

        crate::flog(&format!("[STEAM] shutdown: process still alive after {}s, file edits may race the flush", tries));

    } else {

        std::thread::sleep(std::time::Duration::from_secs(3));

    }

    Ok(())

}

pub fn open_url(url: &str) -> Result<u32, String> {

    #[cfg(windows)]
    let mut command = {

        let mut command = crate::quiet_command::quiet_command("cmd");

        command.args(["/C", "start", "", url]);

        command

    };

    #[cfg(not(windows))]
    let mut command = {

        let mut command = steam_command();

        command.arg(url);

        command

    };

    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|child| child.id())
        .map_err(|error| format!("open {}: {}", url, error))

}

#[cfg(test)]
#[path = "steam_client_tests.rs"]
mod steam_client_tests;
