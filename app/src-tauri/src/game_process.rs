#[cfg(not(windows))]
use std::process::Stdio;

// A CDP RunGame call returning only means the JS ran; Steam can still silently drop it (measured
// 2026-09-19: gid rounded by JS float precision, RunGame "succeeded", no launch). The reaper wrapper
// Steam spawns per launch — `reaper SteamLaunch AppId=<id> -- ...` — is the one process signature
// that proves the launch actually began. Windows Steam has no reaper equivalent, so the CDP return
// is the only signal available there.
#[cfg(not(windows))]
pub fn game_launch_started(appid: u32) -> bool {

    let pattern = format!("reaper SteamLaunch AppId={}", appid);

    crate::quiet_command::quiet_command("pgrep")
        .args(["-f", &pattern])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)

}

#[cfg(windows)]
pub fn game_launch_started(_appid: u32) -> bool {

    true

}

// Under Proton the game itself is a grandchild of reaper, wrapped again by proton/pressure-vessel;
// those wrapper commands also carry the exe path as an argument, so they are excluded by name
// before the last path component is matched against the exe's own basename.
#[cfg(any(not(windows), test))]
const WRAPPER_MARKERS: [&str; 5] = ["reaper", "steamlaunch", "proton", "entry-point", "pressure-vessel"];

#[cfg(any(not(windows), test))]
fn parse_ps_game_pid(output: &str, exe_basename: &str) -> Option<u32> {

    let needle = exe_basename.to_lowercase();

    output
        .lines()
        .filter(|line| {

            let lower = line.to_lowercase();

            !WRAPPER_MARKERS.iter().any(|marker| lower.contains(marker))

        })
        .find_map(|line| {

            let trimmed = line.trim_start();

            let mut parts = trimmed.splitn(2, char::is_whitespace);

            let pid = parts.next()?.parse::<u32>().ok()?;

            let args = parts.next().unwrap_or("").trim();

            let tail = args.rsplit(['/', '\\']).next().unwrap_or(args);

            if tail.to_lowercase() == needle {

                Some(pid)

            } else {

                None

            }

        })

}

#[cfg(not(windows))]
pub fn find_game_pid(exe_basename: &str) -> Option<u32> {

    let output = crate::quiet_command::quiet_command("ps").args(["-eo", "pid=,args="]).output().ok()?;

    parse_ps_game_pid(&String::from_utf8_lossy(&output.stdout), exe_basename)

}

#[cfg(any(windows, test))]
fn parse_tasklist_csv_pid(output: &str, image_name: &str) -> Option<u32> {

    let needle = image_name.to_lowercase();

    output.lines().find_map(|line| {

        let fields: Vec<&str> = line.split(',').map(|field| field.trim_matches('"')).collect();

        let name = fields.first()?.to_lowercase();

        if name != needle {

            return None;

        }

        fields.get(1)?.parse::<u32>().ok()

    })

}

#[cfg(windows)]
pub fn find_game_pid(exe_basename: &str) -> Option<u32> {

    let filter = format!("IMAGENAME eq {}", exe_basename);

    let output = crate::quiet_command::quiet_command("tasklist").args(["/FI", &filter, "/FO", "CSV", "/NH"]).output().ok()?;

    parse_tasklist_csv_pid(&String::from_utf8_lossy(&output.stdout), exe_basename)

}

#[cfg(not(windows))]
pub fn pid_alive(pid: u32) -> bool {

    std::path::Path::new(&format!("/proc/{}", pid)).exists()

}

#[cfg(windows)]
pub fn pid_alive(pid: u32) -> bool {

    let filter = format!("PID eq {}", pid);

    crate::quiet_command::quiet_command("tasklist")
        .args(["/FI", &filter, "/NH"])
        .output()
        .map(|output| crate::steam_client::tasklist_reports_running(&String::from_utf8_lossy(&output.stdout), &pid.to_string()))
        .unwrap_or(false)

}

#[cfg(test)]
#[path = "game_process_tests.rs"]
mod game_process_tests;
