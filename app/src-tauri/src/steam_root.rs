use std::path::PathBuf;

const FLATPAK_MARKER: &str = "com.valvesoftware.Steam";

// steamlocate covers native, Flatpak, Snap and debian-installation on Linux and the HKLM
// InstallPath on Windows; HKCU SteamPath is Steam's own per-launch record and catches installs
// the machine-wide key misses.
fn candidates() -> Vec<PathBuf> {

    #[cfg_attr(not(windows), allow(unused_mut))]
    let mut found: Vec<PathBuf> = steamlocate::locate_all()
        .map(|dirs| dirs.into_iter().map(|dir| dir.path().to_path_buf()).collect())
        .unwrap_or_default();

    #[cfg(windows)]
    if let Some(path) = steam_root_from_registry() {

        found.push(path);

    }

    found

}

pub fn steam_root() -> Option<PathBuf> {

    candidates()
        .into_iter()
        .find(|candidate| candidate.join("config/config.vdf").is_file())

}

pub fn is_flatpak_steam(root: &std::path::Path) -> bool {

    root.to_string_lossy().contains(FLATPAK_MARKER)

}

#[cfg(windows)]
fn steam_root_from_registry() -> Option<PathBuf> {

    let output = crate::quiet_command::quiet_command("reg")
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

#[cfg(test)]
#[path = "steam_root_tests.rs"]
mod steam_root_tests;
