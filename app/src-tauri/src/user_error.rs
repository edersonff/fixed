pub const STEAM_NOT_INSTALLED: &str = "Steam is not installed. Install Steam from store.steampowered.com, sign in once, then press Play again.";

pub const STEAM_NEVER_LOGGED_IN: &str = "Steam has no signed-in account yet. Open Steam, sign in, then press Play again.";

pub const STEAM_DID_NOT_START: &str = "Steam did not finish starting. Open Steam yourself, wait until the library shows, then press Play again.";

pub const STEAM_LAUNCH_FAILED: &str = "Steam could not start the game. Close Steam completely, open it again, then press Play. Details are in the app log.";

pub const FILES_REMOVED: &str = "game-files-missing";

pub const ARCHIVE_GONE: &str = "The game download was already deleted, so these files cannot be restored. Uninstall the game and download it again.";

pub const PLUGIN_UNREADABLE: &str = "This plugin file could not be installed. Pick the .zip, .rar or .dll file the mod page gives you.";

pub const GAME_EXE_MISSING: &str = "The game files are incomplete (no game .exe found). Uninstall and download it again.";

const USER_FACING: [&str; 5] = [STEAM_NOT_INSTALLED, STEAM_NEVER_LOGGED_IN, STEAM_DID_NOT_START, GAME_EXE_MISSING, FILES_REMOVED];

// These conditions fail identically on every launch path, so the fallback path is skipped and the
// person gets the step they must take instead of a second, unrelated failure.
pub fn needs_person(message: &str) -> bool {

    USER_FACING.contains(&message)

}

// Internal failures (CEF, vdf, process) mean nothing to a player; the raw text stays in the log.
pub fn for_person(message: String) -> String {

    if needs_person(&message) {

        return message;

    }

    String::from(STEAM_LAUNCH_FAILED)

}
