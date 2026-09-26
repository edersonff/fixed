const MAPPING_KEY: &str = "\"CompatToolMapping\"";

pub const DEFAULT_COMPAT_TOOL: &str = "proton_experimental";

pub fn config_vdf_path(steam_root: &str) -> String {

    format!("{}/config/config.vdf", steam_root)

}

pub fn has_mapping(body: &str, appid: u32) -> bool {

    let Some(start) = body.find(MAPPING_KEY) else {

        return false;

    };

    let needle = format!("\"{}\"", appid);

    body[start..].find(&needle).is_some()

}

pub(crate) fn insert_mapping(body: &str, appid: u32, tool: &str) -> Option<String> {

    if has_mapping(body, appid) {

        return None;

    }

    let start = body.find(MAPPING_KEY)?;

    let brace = body[start..].find('{')? + start;

    let entry = format!(
        "\n\t\t\t\t\t\"{}\"\n\t\t\t\t\t{{\n\t\t\t\t\t\t\"name\"\t\t\"{}\"\n\t\t\t\t\t\t\"config\"\t\t\"\"\n\t\t\t\t\t\t\"priority\"\t\t\"250\"\n\t\t\t\t\t}}",
        appid, tool,
    );

    let mut out = String::with_capacity(body.len() + entry.len());

    out.push_str(&body[..=brace]);

    out.push_str(&entry);

    out.push_str(&body[brace + 1..]);

    Some(out)

}

// A Windows .exe shortcut with no compat tool assigned has no way to run on Linux, and Steam says
// exactly that: "Game configuration unavailable". Measured 2026-09-19 — every launchable shortcut on
// this machine had a CompatToolMapping entry, the two our app wrote had none.
pub fn is_compat_tool_mapped(steam_root: &str, appid: u32) -> bool {

    if !compat_tool_needed() {

        return true;

    }

    std::fs::read_to_string(config_vdf_path(steam_root))

        .map(|body| has_mapping(&body, appid))

        .unwrap_or(false)

}

// Windows runs the .exe natively; a Proton mapping there is dead config and, read as "unmapped",
// forced a Steam restart on every Play.
pub fn compat_tool_needed() -> bool {

    !cfg!(windows)

}

pub fn ensure_compat_tool(steam_root: &str, appid: u32, tool: &str) -> Result<bool, String> {

    if !compat_tool_needed() {

        return Ok(false);

    }

    let path = config_vdf_path(steam_root);

    let body = std::fs::read_to_string(&path).map_err(|error| format!("read {}: {}", path, error))?;

    let Some(updated) = insert_mapping(&body, appid, tool) else {

        return Ok(false);

    };

    std::fs::write(format!("{}.bak-fixed", path), &body)

        .map_err(|error| format!("backup {}: {}", path, error))?;

    std::fs::write(&path, updated).map_err(|error| format!("write {}: {}", path, error))?;

    Ok(true)

}

pub(crate) fn remove_mapping(body: &str, appid: u32) -> Option<String> {

    let start = body.find(MAPPING_KEY)?;

    let needle = format!("\"{}\"", appid);

    let key_pos = start + body[start..].find(&needle)?;

    let newline_pos = body[..key_pos].rfind('\n')?;

    let brace_open = key_pos + body[key_pos..].find('{')?;

    let mut depth = 0i32;

    let mut entry_end = None;

    for (offset, ch) in body[brace_open..].char_indices() {

        match ch {

            '{' => depth += 1,

            '}' => {

                depth -= 1;

                if depth == 0 {

                    entry_end = Some(brace_open + offset + 1);

                    break;

                }

            }

            _ => {}

        }

    }

    let entry_end = entry_end?;

    let mut out = String::with_capacity(body.len());

    out.push_str(&body[..newline_pos]);

    out.push_str(&body[entry_end..]);

    Some(out)

}

pub fn remove_compat_tool(steam_root: &str, appid: u32) -> Result<bool, String> {

    let path = config_vdf_path(steam_root);

    let body = std::fs::read_to_string(&path).map_err(|error| format!("read {}: {}", path, error))?;

    let Some(updated) = remove_mapping(&body, appid) else {

        return Ok(false);

    };

    std::fs::write(format!("{}.bak-fixed", path), &body)

        .map_err(|error| format!("backup {}: {}", path, error))?;

    std::fs::write(&path, updated).map_err(|error| format!("write {}: {}", path, error))?;

    Ok(true)

}

#[cfg(test)]
#[path = "compat_tool_tests.rs"]
mod compat_tool_tests;
