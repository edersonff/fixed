use std::path::PathBuf;

#[cfg(windows)]
use std::process::Command;

#[cfg(not(windows))]
pub fn steam_root() -> Option<PathBuf> {

    let home = PathBuf::from(crate::home_dir()?);

    for candidate in [home.join(".local/share/Steam"), home.join(".steam/steam")] {

        if candidate.join("config/config.vdf").is_file() {

            return Some(candidate);

        }

    }

    None

}

// Steam writes its own install path to the registry on every launch (SteamPath, REG_SZ), so that
// is checked before the Program Files defaults — a custom install drive would otherwise never be
// found. Confirmed against public SteamPath registry documentation; not exercised against a real
// Windows registry from this machine.
#[cfg(windows)]
fn steam_root_from_registry() -> Option<PathBuf> {

    let output = Command::new("reg")
        .args(["query", r"HKCU\Software\Valve\Steam", "/v", "SteamPath"])
        .output()
        .ok()?;

    if !output.status.success() {

        return None;

    }

    let text = String::from_utf8_lossy(&output.stdout);

    parse_reg_sz_value(&text, "SteamPath").map(PathBuf::from)

}

#[cfg(any(windows, test))]
fn parse_reg_sz_value(output: &str, value_name: &str) -> Option<String> {

    for line in output.lines() {

        let trimmed = line.trim_start();

        let Some(rest) = trimmed.strip_prefix(value_name) else {

            continue;

        };

        let Some(after_type) = rest.trim_start().strip_prefix("REG_SZ") else {

            continue;

        };

        let value = after_type.trim();

        if !value.is_empty() {

            return Some(String::from(value));

        }

    }

    None

}

#[cfg(windows)]
pub fn steam_root() -> Option<PathBuf> {

    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Some(path) = steam_root_from_registry() {

        candidates.push(path);

    }

    if let Ok(x86) = std::env::var("ProgramFiles(x86)") {

        candidates.push(PathBuf::from(x86).join("Steam"));

    }

    if let Ok(program_files) = std::env::var("ProgramFiles") {

        candidates.push(PathBuf::from(program_files).join("Steam"));

    }

    candidates
        .into_iter()
        .find(|candidate| candidate.join("config/config.vdf").is_file())

}

#[cfg(test)]
#[path = "steam_root_tests.rs"]
mod steam_root_tests;
