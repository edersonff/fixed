use super::*;

const OWNER_WINDOWS_LOGINUSERS: &str = "\"users\"\n{\n\t\"76561198216286741\"\n\t{\n\t\t\"AccountName\"\t\t\"someone\"\n\t\t\"PersonaName\"\t\t\"someone\"\n\t\t\"RememberPassword\"\t\t\"1\"\n\t\t\"AutoLogin\"\t\t\"1\"\n\t\t\"Timestamp\"\t\t\"1790381748\"\n\t}\n}\n";

#[test]
fn active_account_reads_timestamp_only_file_from_real_windows_client() {

    assert_eq!(active_account_id(OWNER_WINDOWS_LOGINUSERS), Some(256_021_013));

}

#[test]
fn most_recent_flag_beats_newer_timestamp() {

    let text = "\"users\"\n{\n\"76561198000000001\"\n{\n\"MostRecent\" \"1\"\n\"Timestamp\" \"10\"\n}\n\"76561198000000002\"\n{\n\"MostRecent\" \"0\"\n\"Timestamp\" \"99\"\n}\n}\n";

    assert_eq!(active_account_id(text), Some((76_561_198_000_000_001u64 - STEAMID64_BASE) as u32));

}

#[test]
fn latest_timestamp_wins_when_nothing_flagged() {

    let text = "\"users\"\n{\n\"76561198000000001\"\n{\n\"Timestamp\" \"10\"\n}\n\"76561198000000002\"\n{\n\"Timestamp\" \"99\"\n}\n}\n";

    assert_eq!(active_account_id(text), Some((76_561_198_000_000_002u64 - STEAMID64_BASE) as u32));

}

#[test]
fn garbage_loginusers_yields_no_account() {

    assert_eq!(active_account_id(""), None);

    assert_eq!(active_account_id("not vdf {{{"), None);

}

#[test]
fn account_dir_picks_logged_in_user_over_other_userdata() {

    let root = tempfile::tempdir().unwrap();

    std::fs::create_dir_all(root.path().join("config")).unwrap();

    std::fs::write(root.path().join("config/loginusers.vdf"), OWNER_WINDOWS_LOGINUSERS).unwrap();

    std::fs::create_dir_all(root.path().join("userdata/111/config")).unwrap();

    std::fs::create_dir_all(root.path().join("userdata/256021013/config")).unwrap();

    assert_eq!(account_dir(root.path()), Some(root.path().join("userdata/256021013")));

}

#[test]
fn fresh_account_gets_a_shortcuts_file_steam_code_can_append_to() {

    let root = tempfile::tempdir().unwrap();

    let account = root.path().join("userdata/256021013");

    std::fs::create_dir_all(account.join("config")).unwrap();

    let vdf = ensure_shortcuts_vdf(&account).unwrap();

    let path = vdf.to_str().unwrap();

    let appid = fix_core::add_steam_shortcut(path, "BOMBANANA!", "C:\\Users\\Eder\\games\\BOMBANANA!\\Bombanana.exe", "C:\\Users\\Eder\\games\\BOMBANANA!\\", "").unwrap();

    assert_eq!(fix_core::find_shortcut_appid(path, "BOMBANANA!"), Some(appid));

    assert_eq!(ensure_shortcuts_vdf(&account).unwrap(), vdf);

    let independent: Vec<String> = steamlocate::SteamDir::from_dir(root.path())
        .unwrap()
        .shortcuts()
        .unwrap()
        .map(|shortcut| shortcut.unwrap().app_name)
        .collect();

    assert_eq!(independent, vec![String::from("BOMBANANA!")]);

}
