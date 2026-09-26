use std::path::Path;

use std::path::PathBuf;

const DLLLIST_FILENAME: &str = "dlllist.txt";

const MAX_DEPTH: u32 = 4;

pub(crate) fn parse(text: &str) -> Vec<String> {

    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect()

}

fn walk(dir: &Path, depth_left: u32, found: &mut Vec<PathBuf>) {

    let Ok(entries) = std::fs::read_dir(dir) else {

        return;

    };

    for entry in entries.flatten() {

        let path = entry.path();

        if path.is_dir() {

            if depth_left > 0 {

                walk(&path, depth_left - 1, found);

            }

            continue;

        }

        if path.file_name().map(|name| name.eq_ignore_ascii_case(DLLLIST_FILENAME)).unwrap_or(false) {

            found.push(path);

        }

    }

}

fn find_all(folder: &Path) -> Vec<PathBuf> {

    let mut found = Vec::new();

    walk(folder, MAX_DEPTH, &mut found);

    found

}

fn relative_to(folder: &Path, path: &Path) -> Option<String> {

    let rel = path.strip_prefix(folder).ok()?;

    let parts: Vec<String> = rel.components().map(|part| part.as_os_str().to_string_lossy().to_string()).collect();

    Some(parts.join("/"))

}

// Every online-fix game ships a dlllist.txt beside its exe naming the fix DLLs; an install made by
// an older app version has no .fixed-files manifest at all, so this is the only signal for those.
pub(crate) fn missing(folder: &Path) -> Vec<String> {

    let mut result = Vec::new();

    for dlllist_path in find_all(folder) {

        let Some(dir) = dlllist_path.parent() else {

            continue;

        };

        let Ok(text) = std::fs::read_to_string(&dlllist_path) else {

            continue;

        };

        for name in parse(&text) {

            let full = dir.join(&name);

            if full.exists() {

                continue;

            }

            if let Some(entry) = relative_to(folder, &full) {

                result.push(entry);

            }

        }

    }

    result

}

#[cfg(test)]
#[path = "dlllist_tests.rs"]
mod dlllist_tests;
