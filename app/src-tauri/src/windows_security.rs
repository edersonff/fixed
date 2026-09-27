use crate::flog;

// The installer's NSIS_HOOK_POSTINSTALL runs this same script beside the app, so the two callers
// must resolve it to the identical path: exe-parent-relative, the same convention unrar.exe
// already uses (measured: NSIS places a bundle.resources target flat under $INSTDIR, not under a
// "resources" subfolder, so this mirrors the tauri.conf.json target key exactly).
fn script_relative_path() -> std::path::PathBuf {

    std::path::PathBuf::from("windows-security").join("allow-games-folder.ps1")

}

fn script_path() -> Result<std::path::PathBuf, String> {

    let exe = std::env::current_exe().map_err(|error| format!("current exe: {}", error))?;

    let dir = exe.parent().ok_or_else(|| String::from("exe has no parent dir"))?;

    Ok(dir.join(script_relative_path()))

}

// Every argument travels quoted through the outer -Command string, so a single quote inside a
// path (an apostrophe in a Windows username) cannot close the PowerShell string early.
fn ps_quote(value: &str) -> String {

    format!("'{}'", value.replace('\'', "''"))

}

// Shared with the NSIS hook by convention only (NSIS cannot call Rust): this is the one place the
// flag names are typed, and a test locks them so a rename here is caught before hooks.nsh drifts.
// RemoteSigned (never Bypass) is the documented policy that already runs a local, installer-written
// script — Bypass reads as tampering to a scanner and is not needed here.
pub(crate) fn build_args(script: &str, games_folder: &str, remove: bool) -> Vec<String> {

    let mut args = vec![
        String::from("-NoProfile"),
        String::from("-ExecutionPolicy"),
        String::from("RemoteSigned"),
        String::from("-File"),
        script.to_string(),
        String::from("-GamesFolder"),
        games_folder.to_string(),
    ];

    if remove {

        args.push(String::from("-Remove"));

    }

    args

}

// Start-Process -Verb RunAs is the standard Windows elevation prompt; -Wait -PassThru hands the
// elevated process's own exit code back through $p.ExitCode, which becomes this outer
// powershell.exe's exit code so Rust reads success from one place: Output::status. WindowStyle
// Hidden keeps a console from flashing after the user already consented at the UAC prompt.
pub(crate) fn elevated_command(script: &str, games_folder: &str, remove: bool) -> String {

    let quoted: Vec<String> = build_args(script, games_folder, remove).iter().map(|arg| ps_quote(arg)).collect();

    format!(
        "try {{ $p = Start-Process -FilePath 'powershell.exe' -ArgumentList {} -Verb RunAs -Wait -PassThru -WindowStyle Hidden -ErrorAction Stop; exit $p.ExitCode }} catch {{ exit 1 }}",
        quoted.join(","),
    )

}

#[tauri::command]
pub fn allow_games_folder() -> Result<bool, String> {

    if !cfg!(windows) {

        return Ok(false);

    }

    let script = script_path()?;

    let games_folder = crate::games_root().ok_or_else(|| String::from("home dir not found"))?;

    let command = elevated_command(&script.to_string_lossy(), &games_folder.to_string_lossy(), false);

    let output = crate::quiet_command::quiet_command("powershell.exe")
        .args(["-NoProfile", "-ExecutionPolicy", "RemoteSigned", "-Command", &command])
        .output()
        .map_err(|error| format!("spawn powershell: {}", error))?;

    if !output.status.success() {

        flog(&format!("[SECURITY] allow_games_folder elevated run failed or was declined: {:?}", output.status.code()));

    }

    Ok(output.status.success())

}

#[cfg(test)]
#[path = "windows_security_tests.rs"]
mod windows_security_tests;
