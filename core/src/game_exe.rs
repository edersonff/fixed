use std::path::Path;

const EXCLUSIONS: [&str; 9] = [

    "unitycrashhandler",

    "vc_redist",

    "vcredist",

    "dxsetup",

    "dotnet",

    "unins000",

    "launchersetting",

    "redist",

    "settings",

];

fn walk(dir: &Path, depth: u8, candidates: &mut Vec<(bool, String)>) {

    if depth == 0 {

        return;

    }

    if let Ok(entries) = std::fs::read_dir(dir) {

        for entry in entries.flatten() {

            let path = entry.path();

            if path.is_dir() {

                walk(&path, depth - 1, candidates);

            } else {

                let name = path.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();

                if !name.ends_with(".exe") {

                    continue;

                }

                if EXCLUSIONS.iter().any(|exclusion| name.contains(exclusion)) {

                    continue;

                }

                let stem = name.trim_end_matches(".exe");

                let parent = path.parent()

                    .and_then(|p| p.file_name())

                    .map(|n| n.to_string_lossy().to_lowercase())

                    .unwrap_or_default();

                candidates.push((stem == parent, path.to_string_lossy().to_string()));

            }

        }

    }

}

pub fn find_game_exe(folder: &str) -> Option<String> {

    let mut candidates: Vec<(bool, String)> = Vec::new();

    walk(Path::new(folder), 4, &mut candidates);

    candidates.sort_by(|a, b| b.0.cmp(&a.0));

    candidates.first().map(|(_, path)| path.clone())

}
