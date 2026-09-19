pub fn looks_like_rar(path: &std::path::Path) -> bool {

    let Ok(mut file) = std::fs::File::open(path) else {

        return false;

    };

    use std::io::Read;

    let mut magic = [0u8; 7];

    file.read_exact(&mut magic).is_ok() && &magic[..6] == b"Rar!\x1a\x07"

}

pub fn extract_first_rar(folder: &str) -> Result<u32, String> {

    let mut rars: Vec<std::path::PathBuf> = std::fs::read_dir(folder)

        .map_err(|error| format!("read {}: {}", folder, error))?

        .flatten()

        .map(|entry| entry.path())

        .filter(|path| path.extension().map(|ext| ext == "rar").unwrap_or(false))

        .collect();

    rars.sort_by_key(|path| {

        let size = path.metadata().map(|meta| meta.len()).unwrap_or(0);

        std::cmp::Reverse(size)

    });

    let mut last_error = String::from("no .rar found in download folder");

    for rar in &rars {

        if !looks_like_rar(rar) {

            eprintln!("[DL] skipping invalid archive: {}", rar.display());

            continue;

        }

        match fix_core::extract_archive(&rar.to_string_lossy(), folder) {

            Ok(count) => return Ok(count),

            Err(error) => {

                eprintln!("[DL] extract failed for {}: {}", rar.display(), error);

                last_error = error;

            }

        }

    }

    Err(last_error)

}

pub fn home_dir() -> Option<String> {

    if let Ok(home) = std::env::var("HOME") {

        if !home.is_empty() {

            return Some(home);

        }

    }

    std::env::var("USERPROFILE").ok().filter(|home| !home.is_empty())

}

pub fn games_root() -> Option<std::path::PathBuf> {

    home_dir().map(|home| std::path::PathBuf::from(home).join("games"))

}

pub fn game_folder(title: &str) -> Option<String> {

    games_root().map(|root| root.join(title).to_string_lossy().to_string())

}

pub fn find_shortcuts_vdf() -> Option<String> {

    let mut roots: Vec<std::path::PathBuf> = Vec::new();

    if let Some(home) = home_dir() {

        let home_path = std::path::PathBuf::from(&home);

        roots.push(home_path.join(".steam/steam/userdata"));

        roots.push(home_path.join(".local/share/Steam/userdata"));

    }

    if let Ok(x86) = std::env::var("ProgramFiles(x86)") {

        roots.push(std::path::PathBuf::from(&x86).join("Steam/userdata"));

    }

    if let Ok(program_files) = std::env::var("ProgramFiles") {

        roots.push(std::path::PathBuf::from(&program_files).join("Steam/userdata"));

    }

    if let Ok(steam) = std::env::var("STEAM_PATH") {

        roots.push(std::path::PathBuf::from(steam).join("userdata"));

    }

    for userdata in roots {

        let Ok(entries) = std::fs::read_dir(&userdata) else {

            continue;

        };

        for entry in entries.flatten() {

            let candidate = entry.path().join("config").join("shortcuts.vdf");

            if candidate.exists() {

                return candidate.to_str().map(String::from);

            }

        }

    }

    None

}

pub fn add_game_to_steam(title: &str, folder: &str) -> bool {

    let Some(vdf) = find_shortcuts_vdf() else {

        eprintln!("[DL] {}: steam shortcuts.vdf not found", title);

        return false;

    };

    if let Some(appid) = fix_core::find_shortcut_appid(&vdf, title) {

        eprintln!("[DL] {}: already in Steam (appid {})", title, appid);

        return false;

    }

    let Some(exe) = fix_core::find_game_exe(folder) else {

        eprintln!("[DL] {}: no game exe found in {}", title, folder);

        return false;

    };

    let start_dir = std::path::Path::new(&exe)

        .parent()

        .map(|parent| format!("{}{}", parent.to_string_lossy(), std::path::MAIN_SEPARATOR))

        .unwrap_or_default();

    match fix_core::add_steam_shortcut(&vdf, title, &exe, &start_dir, fix_core::ONLINE_FIX_LAUNCH_OPTIONS) {

        Ok(index) => {

            eprintln!("[DL] {}: added to Steam (index {})", title, index);

            true

        }

        Err(error) => {

            eprintln!("[DL] {}: steam shortcut FAILED: {}", title, error);

            false

        }

    }

}

pub fn delete_installers(folder: &str) {

    let Ok(entries) = std::fs::read_dir(folder) else {

        return;

    };

    for entry in entries.flatten() {

        let path = entry.path();

        let is_rar = path.extension().map(|ext| ext == "rar").unwrap_or(false);

        if !is_rar {

            continue;

        }

        match std::fs::remove_file(&path) {

            Ok(()) => eprintln!("[DL] installer removed: {}", path.display()),

            Err(error) => eprintln!("[DL] installer remove FAILED: {}: {}", path.display(), error),

        }

    }

}

pub fn log_fail(title: &str, step: &str, error: String) -> String {

    eprintln!("[DL] {}: {} FAILED: {}", title, step, error);

    error

}

