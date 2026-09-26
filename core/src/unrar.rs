use std::process::Command;

use std::process::Output;

pub const RAR_PASSWORD: &str = "online-fix.me";

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

// A GUI-subsystem app that spawns a console program (unrar, tasklist, reg, cmd) gets a console
// window flashed per call unless CREATE_NO_WINDOW is set.
pub fn quiet_command(program: impl AsRef<std::ffi::OsStr>) -> Command {

    #[allow(unused_mut)]
    let mut command = Command::new(program);

    #[cfg(windows)]
    {

        use std::os::windows::process::CommandExt;

        command.creation_flags(CREATE_NO_WINDOW);

    }

    command

}

fn unrar_candidates() -> Vec<String> {

    let mut candidates = Vec::new();

    if let Some(dir) = std::env::current_exe().ok().and_then(|exe| exe.parent().map(|dir| dir.to_path_buf())) {

        candidates.push(dir.join(if cfg!(windows) { "unrar.exe" } else { "unrar" }).to_string_lossy().to_string());

    }

    if cfg!(windows) {

        candidates.push(String::from("unrar.exe"));

        candidates.push(String::from("UnRAR"));

    } else {

        candidates.push(String::from("unrar"));

    }

    candidates

}

fn run_unrar(args: &[String]) -> Result<Output, String> {

    let candidates = unrar_candidates();

    let mut spawn_error = String::new();

    for binary in &candidates {

        match quiet_command(binary).args(args).output() {

            Ok(output) => return Ok(output),

            Err(error) => spawn_error = format!("{}: {}", binary, error),

        }

    }

    Err(format!("spawn unrar (tried [{}]): {}", candidates.join(", "), spawn_error))

}

fn require_success(output: &Output) -> Result<(), String> {

    if output.status.success() {

        return Ok(());

    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    let lines: Vec<String> = String::from_utf8_lossy(&output.stdout).lines().map(String::from).collect();

    let stdout_tail = lines[lines.len().saturating_sub(5)..].join(" | ");

    Err(format!("unrar exit {:?}: stderr=[{}] stdout_tail=[{}]", output.status.code(), stderr, stdout_tail))

}

fn extracted_count(output: &Output) -> u32 {

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| line.starts_with("Extracting"))
        .count() as u32

}

fn destination_arg(dest_dir: &str) -> Result<String, String> {

    let trimmed = dest_dir.trim_end_matches(['/', '\\']);

    std::fs::create_dir_all(trimmed).map_err(|error| format!("create {}: {}", trimmed, error))?;

    Ok(format!("{}{}", trimmed, std::path::MAIN_SEPARATOR))

}

pub fn extract_archive(archive_path: &str, dest_dir: &str) -> Result<u32, String> {

    let args = vec![
        String::from("x"),
        format!("-p{}", RAR_PASSWORD),
        String::from("-o+"),
        String::from("-idp"),
        archive_path.to_string(),
        destination_arg(dest_dir)?,
    ];

    let output = run_unrar(&args)?;

    require_success(&output)?;

    Ok(extracted_count(&output))

}

// Bare listing: one archive-relative path per line, directories included.
pub fn list_archive_entries(archive_path: &str) -> Result<Vec<String>, String> {

    let args = vec![String::from("lb"), String::from("-scfr"), format!("-p{}", RAR_PASSWORD), archive_path.to_string()];

    let output = run_unrar(&args)?;

    require_success(&output)?;

    Ok(parse_listing(&String::from_utf8_lossy(&output.stdout)))

}

// bundled unrar 7.30 beta appends the four literal characters `\x0d` (not an actual carriage
// return) before the real newline on every `lb` line, on every flag variant we tried.
fn parse_listing(text: &str) -> Vec<String> {

    text.lines()
        .map(|line| line.trim_end_matches('\r').trim_end_matches("\\x0d").to_string())
        .filter(|line| !line.is_empty())
        .collect()

}

// Windows caps a process command line at 32767 chars, so a long missing-files list is split
// across several unrar calls instead of risking one oversized argv.
const EXTRACT_CHUNK_SIZE: usize = 100;

fn extract_entries_chunk(archive_path: &str, dest_dir: &str, entries: &[String]) -> Result<u32, String> {

    let mut args = vec![
        String::from("x"),
        format!("-p{}", RAR_PASSWORD),
        String::from("-o+"),
        String::from("-idp"),
        archive_path.to_string(),
    ];

    args.extend(entries.iter().cloned());

    args.push(destination_arg(dest_dir)?);

    let output = run_unrar(&args)?;

    require_success(&output)?;

    Ok(extracted_count(&output))

}

pub fn extract_entries(archive_path: &str, dest_dir: &str, entries: &[String]) -> Result<u32, String> {

    if entries.is_empty() {

        return Ok(0);

    }

    let mut total = 0;

    for chunk in entries.chunks(EXTRACT_CHUNK_SIZE) {

        total += extract_entries_chunk(archive_path, dest_dir, chunk)?;

    }

    Ok(total)

}

#[cfg(test)]
#[path = "unrar_tests.rs"]
mod unrar_tests;
