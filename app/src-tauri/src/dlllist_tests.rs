use super::*;

#[test]
fn parse_reads_the_exact_bytes_shipped_with_online_fix_games() {

    let text = "SteamOverlay64.dll\r\nOnlineFix64.dll";

    assert_eq!(parse(text), vec![String::from("SteamOverlay64.dll"), String::from("OnlineFix64.dll")]);

}

#[test]
fn parse_skips_empty_lines() {

    let text = "SteamOverlay64.dll\r\n\r\nOnlineFix64.dll\r\n";

    assert_eq!(parse(text), vec![String::from("SteamOverlay64.dll"), String::from("OnlineFix64.dll")]);

}

#[test]
fn missing_reports_a_dll_the_list_names_when_there_is_no_manifest_at_all() {

    let dir = tempfile::tempdir().unwrap();

    let game_dir = dir.path().join("Game");

    std::fs::create_dir_all(&game_dir).unwrap();

    std::fs::write(game_dir.join("Game.exe"), b"x").unwrap();

    std::fs::write(game_dir.join("SteamOverlay64.dll"), b"x").unwrap();

    std::fs::write(game_dir.join("dlllist.txt"), "SteamOverlay64.dll\r\nOnlineFix64.dll").unwrap();

    assert_eq!(missing(dir.path()), vec![String::from("Game/OnlineFix64.dll")]);

}

#[test]
fn missing_finds_nothing_past_the_max_depth() {

    let dir = tempfile::tempdir().unwrap();

    let deep = dir.path().join("a").join("b").join("c").join("d").join("e");

    std::fs::create_dir_all(&deep).unwrap();

    std::fs::write(deep.join("dlllist.txt"), "OnlineFix64.dll").unwrap();

    assert!(missing(dir.path()).is_empty());

}
