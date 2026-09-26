use super::*;

#[test]
fn missing_entries_lists_only_what_left_the_disk() {

    let dir = tempfile::tempdir().unwrap();

    std::fs::create_dir_all(dir.path().join("Game")).unwrap();

    std::fs::write(dir.path().join("Game").join("Game.exe"), b"x").unwrap();

    let entries = vec![
        String::from("Game"),
        String::from("Game/Game.exe"),
        String::from("Game/OnlineFix64.dll"),
        String::from("Game\\winmm.dll"),
    ];

    assert_eq!(
        missing_entries(dir.path(), &entries),
        vec![String::from("Game/OnlineFix64.dll"), String::from("Game\\winmm.dll")],
    );

}

#[test]
fn no_manifest_means_nothing_missing() {

    let dir = tempfile::tempdir().unwrap();

    assert!(missing_files(dir.path()).is_empty());

}

#[test]
fn missing_files_unions_manifest_and_dlllist_without_duplicates() {

    let dir = tempfile::tempdir().unwrap();

    let game_dir = dir.path().join("Game");

    std::fs::create_dir_all(&game_dir).unwrap();

    std::fs::write(game_dir.join("Game.exe"), b"x").unwrap();

    std::fs::write(dir.path().join(MANIFEST), "Game/OnlineFix64.dll\nGame/OnlineFix.ini").unwrap();

    std::fs::write(game_dir.join("dlllist.txt"), "SteamOverlay64.dll\r\nOnlineFix64.dll").unwrap();

    let mut missing = missing_files(dir.path());

    missing.sort();

    assert_eq!(
        missing,
        vec![String::from("Game/OnlineFix.ini"), String::from("Game/OnlineFix64.dll"), String::from("Game/SteamOverlay64.dll")],
    );

}

#[test]
fn match_missing_to_archive_finds_by_case_insensitive_suffix() {

    let missing = vec![String::from("Game/OnlineFix64.dll")];

    let archive_entries = vec![String::from("Install/game/ONLINEFIX64.DLL"), String::from("Install/game/other.dll")];

    assert_eq!(match_missing_to_archive(&missing, &archive_entries), vec![String::from("Install/game/ONLINEFIX64.DLL")]);

}

#[test]
fn match_missing_to_archive_drops_an_entry_the_archive_never_had() {

    let missing = vec![String::from("Game/NeverPacked.dll")];

    let archive_entries = vec![String::from("Game/OnlineFix64.dll")];

    assert!(match_missing_to_archive(&missing, &archive_entries).is_empty());

}

#[test]
fn archive_in_ignores_files_that_are_not_rar() {

    let dir = tempfile::tempdir().unwrap();

    std::fs::write(dir.path().join("fake.rar"), b"not a rar").unwrap();

    assert_eq!(archive_in(dir.path()), None);

    std::fs::write(dir.path().join("real.rar"), b"Rar!\x1a\x07\x01\x00rest").unwrap();

    assert_eq!(archive_in(dir.path()), Some(dir.path().join("real.rar")));

}

#[test]
fn files_removed_again_is_true_when_a_restored_file_reappears_as_missing() {

    let restored = vec![String::from("Game/OnlineFix64.dll"), String::from("Game/winmm.dll")];

    let missing_now = vec![String::from("Game/OnlineFix64.dll")];

    assert!(files_removed_again(&restored, &missing_now));

}

#[test]
fn files_removed_again_is_false_when_the_restored_files_stay() {

    let restored = vec![String::from("Game/OnlineFix64.dll"), String::from("Game/winmm.dll")];

    assert!(!files_removed_again(&restored, &[]));

}

#[test]
fn should_close_for_vanished_files_keeps_the_game_when_nothing_new_is_missing() {

    assert!(!should_close_for_vanished_files(&[], &[]));

}

#[test]
fn should_close_for_vanished_files_closes_when_a_file_vanishes_after_launch() {

    let missing_after = vec![String::from("Game/winmm.dll")];

    assert!(should_close_for_vanished_files(&[], &missing_after));

}

#[test]
fn should_close_for_vanished_files_ignores_a_file_that_was_already_missing_before() {

    let missing_before = vec![String::from("Game/OnlineFix.ini")];

    let missing_after = vec![String::from("Game/OnlineFix.ini")];

    assert!(!should_close_for_vanished_files(&missing_before, &missing_after));

}
