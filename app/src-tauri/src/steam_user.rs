use std::path::Path;

use std::path::PathBuf;

const STEAMID64_BASE: u64 = 76_561_197_960_265_728;

// Steam writes shortcuts.vdf only once the account adds its first non-Steam game, so a fresh
// account has none (measured 2026-09-25 on the owner's Windows: userdata/<id>/config held
// localconfig.vdf and no shortcuts.vdf; every Play died "steam shortcuts.vdf not found").
const EMPTY_SHORTCUTS_VDF: &[u8] = b"\x00shortcuts\x00\x08\x08";

struct LoginUser {
    account_id: u32,
    most_recent: bool,
    timestamp: u64,
}

fn read_login_users(text: &str) -> Vec<LoginUser> {

    let Ok(parsed) = keyvalues_parser::parse(text) else {

        return Vec::new();

    };

    let Some(users) = parsed.value.get_obj() else {

        return Vec::new();

    };

    let mut found = Vec::new();

    for (steam_id, values) in users.iter() {

        let Ok(id64) = steam_id.parse::<u64>() else { continue };

        let Some(fields) = values.first().and_then(|value| value.get_obj()) else { continue };

        let field = |name: &str| {

            fields
                .get(name)
                .and_then(|values| values.first())
                .and_then(|value| value.get_str())
                .map(String::from)

        };

        found.push(LoginUser {
            account_id: id64.saturating_sub(STEAMID64_BASE) as u32,
            most_recent: field("MostRecent").as_deref() == Some("1"),
            timestamp: field("Timestamp").and_then(|text| text.parse().ok()).unwrap_or(0),
        });

    }

    found

}

// Newer clients stopped writing MostRecent (the owner's Windows loginusers.vdf carries only
// Timestamp), so the latest Timestamp decides when no entry is flagged.
pub(crate) fn active_account_id(login_users_vdf: &str) -> Option<u32> {

    let users = read_login_users(login_users_vdf);

    if let Some(flagged) = users.iter().find(|user| user.most_recent) {

        return Some(flagged.account_id);

    }

    users.iter().max_by_key(|user| user.timestamp).map(|user| user.account_id)

}

fn newest_userdata_dir(userdata: &Path) -> Option<PathBuf> {

    std::fs::read_dir(userdata)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {

            let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("");

            name != "0" && name.parse::<u32>().is_ok() && path.join("config").is_dir()

        })
        .max_by_key(|path| path.join("config").metadata().and_then(|meta| meta.modified()).ok())

}

pub(crate) fn account_dir(root: &Path) -> Option<PathBuf> {

    let userdata = root.join("userdata");

    let login_users = std::fs::read_to_string(root.join("config/loginusers.vdf")).unwrap_or_default();

    if let Some(account) = active_account_id(&login_users) {

        let dir = userdata.join(account.to_string());

        if dir.is_dir() {

            return Some(dir);

        }

    }

    newest_userdata_dir(&userdata)

}

pub(crate) fn ensure_shortcuts_vdf(account_dir: &Path) -> Result<PathBuf, String> {

    let config = account_dir.join("config");

    let vdf = config.join("shortcuts.vdf");

    if vdf.is_file() {

        return Ok(vdf);

    }

    std::fs::create_dir_all(&config).map_err(|error| format!("create {}: {}", config.display(), error))?;

    std::fs::write(&vdf, EMPTY_SHORTCUTS_VDF).map_err(|error| format!("create {}: {}", vdf.display(), error))?;

    crate::flog(&format!("[STEAM] created empty shortcuts.vdf at {}", vdf.display()));

    Ok(vdf)

}

pub fn shortcuts_vdf() -> Result<String, String> {

    let root = crate::steam_root::steam_root().ok_or_else(|| String::from(crate::user_error::STEAM_NOT_INSTALLED))?;

    let account = account_dir(&root).ok_or_else(|| String::from(crate::user_error::STEAM_NEVER_LOGGED_IN))?;

    let vdf = ensure_shortcuts_vdf(&account)?;

    Ok(vdf.to_string_lossy().into_owned())

}

#[cfg(test)]
#[path = "steam_user_tests.rs"]
mod steam_user_tests;
