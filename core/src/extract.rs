
const PLUGIN_META_FILES: [&str; 4] = ["manifest.json", "icon.png", "readme.md", "changelog.md"];


pub const RAR_PASSWORD: &str = "online-fix.me";

pub fn install_plugin(archive_path: &str, game_dir: &str) -> Result<u32, String> {

    let data = std::fs::read(archive_path).map_err(|error| format!("read {}: {}", archive_path, error))?;

    if data.starts_with(b"MZ") {

        let name = std::path::Path::new(archive_path)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| String::from("plugin.dll"));

        let plugins_dir = format!("{}/BepInEx/plugins", game_dir);

        std::fs::create_dir_all(&plugins_dir).map_err(|error| format!("create {}: {}", plugins_dir, error))?;

        std::fs::write(format!("{}/{}", plugins_dir, name), &data).map_err(|error| format!("write dll: {}", error))?;

        return Ok(1);

    }

    if data.starts_with(b"Rar!") {

        let temp_dir = format!("{}/.fixed-plugin-tmp", game_dir);

        extract_archive(archive_path, &temp_dir)?;

        let count = place_extracted(&temp_dir, game_dir)?;

        std::fs::remove_dir_all(&temp_dir).map_err(|error| format!("cleanup: {}", error))?;

        return Ok(count);

    }

    let reader = std::io::Cursor::new(data);

    let mut archive = zip::ZipArchive::new(reader).map_err(|error| format!("open zip: {}", error))?;

    let mut count: u32 = 0;

    for index in 0..archive.len() {

        let mut file = archive.by_index(index).map_err(|error| format!("zip entry: {}", error))?;

        if file.is_dir() {

            continue;

        }

        let name = file.name().to_string();

        let base = name.rsplit('/').next().unwrap_or("").to_lowercase();

        if PLUGIN_META_FILES.contains(&base.as_str()) {

            continue;

        }

        let mut bytes: Vec<u8> = Vec::new();

        std::io::Read::read_to_end(&mut file, &mut bytes).map_err(|error| format!("read {}: {}", name, error))?;

        let root_dll = !name.contains('/') && name.to_lowercase().ends_with(".dll");

        let dest = if root_dll {

            format!("{}/BepInEx/plugins/{}", game_dir, name)

        } else {

            format!("{}/{}", game_dir, name)

        };

        if let Some(parent) = std::path::Path::new(&dest).parent() {

            std::fs::create_dir_all(parent).map_err(|error| format!("create dirs: {}", error))?;

        }

        std::fs::write(&dest, &bytes).map_err(|error| format!("write {}: {}", dest, error))?;

        count += 1;

    }

    Ok(count)

}

pub fn apply_fix_repair(title_folder: &str, game_dir: &str) -> Result<u32, String> {

    let repair_dir = format!("{}/Fix Repair", title_folder);

    let entries = std::fs::read_dir(&repair_dir).map_err(|error| format!("read {}: {}", repair_dir, error))?;

    let mut applied: u32 = 0;

    for entry in entries.flatten() {

        let path = entry.path();

        let is_rar = path

            .extension()

            .map(|ext| ext.to_string_lossy().to_lowercase() == "rar")

            .unwrap_or(false);

        if !is_rar {

            continue;

        }

        let rar_path = path.to_string_lossy().to_string();

        extract_archive(&rar_path, game_dir)?;

        applied += 1;

    }

    Ok(applied)

}

fn place_extracted(source: &str, game_dir: &str) -> Result<u32, String> {

    let mut count: u32 = 0;

    let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(source)

        .map_err(|error| format!("read {}: {}", source, error))?

        .flatten()

        .map(|entry| entry.path())

        .collect();

    while let Some(path) = paths.pop() {

        if path.is_dir() {

            let children = std::fs::read_dir(&path).map_err(|error| format!("read dir: {}", error))?;

            for child in children.flatten() {

                paths.push(child.path());

            }

            continue;

        }

        let relative_path = path

            .strip_prefix(source)

            .map_err(|error| format!("relative: {}", error))?;

        let relative = relative_path.to_string_lossy().to_string();

        let base = relative_path.file_name().map(|name| name.to_string_lossy().to_lowercase()).unwrap_or_default();

        if PLUGIN_META_FILES.contains(&base.as_str()) {

            continue;

        }

        let root_dll = relative_path.components().count() == 1 && relative.to_lowercase().ends_with(".dll");

        let dest = if root_dll {

            format!("{}/BepInEx/plugins/{}", game_dir, relative)

        } else {

            format!("{}/{}", game_dir, relative)

        };

        if let Some(parent) = std::path::Path::new(&dest).parent() {

            std::fs::create_dir_all(parent).map_err(|error| format!("create dirs: {}", error))?;

        }

        std::fs::copy(&path, &dest).map_err(|error| format!("copy {}: {}", dest, error))?;

        count += 1;

    }

    Ok(count)

}

pub fn extract_archive(archive_path: &str, dest_dir: &str) -> Result<u32, String> {

    let trimmed_dest = dest_dir.trim_end_matches('/');

    std::fs::create_dir_all(trimmed_dest).map_err(|error| format!("create {}: {}", trimmed_dest, error))?;

    let destination = format!("{}/", trimmed_dest);

    let password_flag = format!("-p{}", RAR_PASSWORD);

    let mut unrar_candidates: Vec<String> = Vec::new();

    if let Ok(current_exe) = std::env::current_exe() {

        if let Some(dir) = current_exe.parent() {

            let sidecar = dir.join(if cfg!(windows) { "unrar.exe" } else { "unrar" });

            unrar_candidates.push(sidecar.to_string_lossy().to_string());

        }

    }

    if cfg!(windows) {

        unrar_candidates.push(String::from("unrar.exe"));

        unrar_candidates.push(String::from("UnRAR"));

    } else {

        unrar_candidates.push(String::from("unrar"));

    }

    let mut output: Option<std::process::Output> = None;

    let mut spawn_error: Option<String> = None;

    for binary in &unrar_candidates {

        match std::process::Command::new(binary)

            .arg("x")

            .arg(&password_flag)

            .arg("-o+")

            .arg("-idp")

            .arg(archive_path)

            .arg(&destination)

            .output()

        {

            Ok(result) => {

                output = Some(result);

                break;

            }

            Err(error) => {

                spawn_error = Some(format!("{}: {}", binary, error));

            }

        }

    }

    let output = output.ok_or_else(|| {

        let tried = unrar_candidates.join(", ");

        format!("spawn unrar (tried [{}]): {}", tried, spawn_error.unwrap_or_default())

    })?;

    if !output.status.success() {

        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        let lines: Vec<String> = String::from_utf8_lossy(&output.stdout)

            .lines()

            .map(String::from)

            .collect();

        let tail_start = lines.len().saturating_sub(5);

        let stdout_tail: String = lines[tail_start..].join(" | ");

        return Err(format!(

            "unrar exit {:?}: stderr=[{}] stdout_tail=[{}]",

            output.status.code(),

            stderr,

            stdout_tail

        ));

    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let count = stdout.lines().filter(|line| line.starts_with("Extracting")).count();

    Ok(count as u32)

}

#[cfg(test)]
#[path = "extract_tests.rs"]
mod extract_tests;
