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

    Command::new("pgrep")
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

    Command::new("tasklist")
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

fn spawn_steam(command: &mut Command) -> Result<(), String> {

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
        .ok_or_else(|| String::from("steam root not found"))

}

#[cfg(windows)]
pub fn start_silent() -> Result<(), String> {

    let exe = steam_executable()?;

    let mut command = Command::new(exe);

    command.arg("-silent");

    spawn_steam(&mut command)

}

#[cfg(not(windows))]
pub fn start_silent() -> Result<(), String> {

    let mut command = Command::new("steam");

    command.arg("-silent");

    spawn_steam(&mut command)

}

#[cfg(windows)]
pub fn shutdown() -> Result<(), String> {

    let exe = steam_executable()?;

    let mut command = Command::new(exe);

    command.arg("-shutdown");

    run_shutdown(&mut command)

}

#[cfg(not(windows))]
pub fn shutdown() -> Result<(), String> {

    let mut command = Command::new("steam");

    command.arg("-shutdown");

    run_shutdown(&mut command)

}

pub fn open_url(url: &str) -> Result<u32, String> {

    #[cfg(windows)]
    let mut command = {

        let mut command = Command::new("cmd");

        command.args(["/C", "start", "", url]);

        command

    };

    #[cfg(not(windows))]
    let mut command = {

        let mut command = Command::new("steam");

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
