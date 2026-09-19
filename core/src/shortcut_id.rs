pub fn shortcut_appid(exe_path: &str, app_name: &str) -> u32 {

    let mut hash: u32 = 2166136261;

    for byte in exe_path.bytes().chain(app_name.bytes()) {

        hash ^= byte as u32;

        hash = hash.wrapping_mul(16777619);

    }

    hash | 0x80000000

}

// `steam://rungameid` wants the 64-bit shortcut gameid, never the 32-bit appid stored in the vdf.
// Measured 2026-09-19: the 32-bit form returns "Game configuration unavailable"; the 64-bit form
// launches through `reaper SteamLaunch AppId=... -- SteamLinuxRuntime_4/_v2-entry-point -- proton`.
pub fn shortcut_gameid(appid: u32) -> u64 {

    ((appid as u64) << 32) | 0x0200_0000

}

#[cfg(test)]
#[path = "shortcut_id_tests.rs"]
mod shortcut_id_tests;
