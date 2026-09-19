use std::path::PathBuf;

use std::process::Command;

use std::process::Stdio;

use std::time::Duration;

use std::time::Instant;

pub(crate) const READY_TIMEOUT: Duration = Duration::from_secs(45);

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) const POLL: Duration = Duration::from_millis(500);

pub fn steam_root() -> Option<PathBuf> {

    let home = PathBuf::from(crate::home_dir()?);

    for candidate in [home.join(".local/share/Steam"), home.join(".steam/steam")] {

        if candidate.join("config/config.vdf").is_file() {

            return Some(candidate);

        }

    }

    None

}

fn process_alive(name: &str) -> bool {

    Command::new("pgrep")
        .args(["-x", name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)

}

pub fn is_steam_running() -> bool {

    process_alive("steam")

}

// The client answers steam:// only once its webhelper is up; a URL fired at a cold client is
// consumed as a startup argument instead (measured 2026-09-19: Steam booted, pulled a 190MB update,
// showed the dash, launched nothing). So readiness is the webhelper, not the steam process.
pub fn is_steam_ready() -> bool {

    is_steam_running() && process_alive("steamwebhelper")

}

pub fn start_silent() -> Result<(), String> {

    Command::new("steam")
        .arg("-silent")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("start steam: {}", error))

}

pub fn shutdown() -> Result<(), String> {

    Command::new("steam")
        .arg("-shutdown")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| format!("steam -shutdown: {}", error))?;

    let deadline = Instant::now() + SHUTDOWN_TIMEOUT;

    while Instant::now() < deadline {

        if !is_steam_running() {

            return Ok(());

        }

        std::thread::sleep(POLL);

    }

    Err(String::from("steam did not exit"))

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
