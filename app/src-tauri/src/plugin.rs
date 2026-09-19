#[tauri::command]

pub fn install_plugin(title: String, archive_path: String) -> Result<u32, String> {


    let folder = crate::game_folder(&title).ok_or_else(|| String::from("home dir not found"))?;

    let Some(exe) = fix_core::find_game_exe(&folder) else {

        let message = format!("no game exe found in {}", folder);

        eprintln!("[PLUGIN] {}: {}", title, message);

        return Err(message);

    };

    let game_dir = std::path::Path::new(&exe)

        .parent()

        .map(|parent| parent.to_string_lossy().to_string())

        .unwrap_or(folder.clone());

    let bep_core = std::path::Path::new(&game_dir).join("BepInEx").join("core");

    if !bep_core.exists() {

        eprintln!("[PLUGIN] {}: BepInEx loader not found, plugin placed anyway", title);

    }

    match fix_core::install_plugin(&archive_path, &game_dir) {

        Ok(count) => {

            eprintln!("[PLUGIN] {}: installed {} file(s) into {}", title, count, game_dir);

            match fix_core::apply_fix_repair(&folder, &game_dir) {

                Ok(repaired) if repaired > 0 => {

                    eprintln!("[PLUGIN] {}: fix repair reapplied ({} archive) over plugin", title, repaired);

                }

                Ok(_) => eprintln!("[PLUGIN] {}: no fix repair archive present", title),

                Err(error) => eprintln!("[PLUGIN] {}: fix repair FAILED: {}", title, error),

            }

            Ok(count)

        }

        Err(error) => {

            eprintln!("[PLUGIN] {}: FAILED: {}", title, error);

            Err(error)

        }

    }

}
