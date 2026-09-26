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
fn archive_in_ignores_files_that_are_not_rar() {

    let dir = tempfile::tempdir().unwrap();

    std::fs::write(dir.path().join("fake.rar"), b"not a rar").unwrap();

    assert_eq!(archive_in(dir.path()), None);

    std::fs::write(dir.path().join("real.rar"), b"Rar!\x1a\x07\x01\x00rest").unwrap();

    assert_eq!(archive_in(dir.path()), Some(dir.path().join("real.rar")));

}

#[test]
fn download_bytes_is_zero_without_an_archive() {

    let dir = tempfile::tempdir().unwrap();

    assert_eq!(download_bytes(dir.path()), 0);

}

#[test]
fn download_bytes_reports_the_kept_archive_size() {

    let dir = tempfile::tempdir().unwrap();

    std::fs::write(dir.path().join("real.rar"), b"Rar!\x1a\x07\x01\x00rest").unwrap();

    assert_eq!(download_bytes(dir.path()), 12);

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
