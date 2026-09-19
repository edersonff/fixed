use super::*;

#[test]
fn add_steam_shortcut_rejects_file_without_map_terminator() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, b"not a vdf file").expect("write fixture");

    let result = add_steam_shortcut(vdf_path.to_str().unwrap(), "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS);

    assert!(result.is_err());

}

#[test]
fn add_steam_shortcut_round_trips_through_find_shortcut_appid() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, crate::test_support::empty_shortcuts_vdf()).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    let returned = add_steam_shortcut(path, "Friendly Steps", "/games/Friendly Steps/Friendly Steps.exe", "/games/Friendly Steps/", ONLINE_FIX_LAUNCH_OPTIONS)
        .expect("add shortcut");

    let expected_appid = crate::shortcut_appid("/games/Friendly Steps/Friendly Steps.exe", "Friendly Steps");

    assert_eq!(returned, expected_appid);

    let found = crate::find_shortcut_appid(path, "Friendly Steps").expect("shortcut findable");

    assert_eq!(found, expected_appid);

}

#[test]
fn add_steam_shortcut_is_idempotent_for_the_same_app_name() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, crate::test_support::empty_shortcuts_vdf()).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    let first = add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS).expect("first add");

    let second = add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS).expect("second add");

    assert_eq!(first, second);

}

#[test]
fn add_steam_shortcut_returns_the_written_appid_for_each_distinct_app() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, crate::test_support::empty_shortcuts_vdf()).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    let first = add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS).expect("add foo");

    let second = add_steam_shortcut(path, "Bar", "/games/Bar/Bar.exe", "/games/Bar/", ONLINE_FIX_LAUNCH_OPTIONS).expect("add bar");

    assert_eq!(first, crate::shortcut_appid("/games/Foo/Foo.exe", "Foo"));

    assert_eq!(second, crate::shortcut_appid("/games/Bar/Bar.exe", "Bar"));

    assert_ne!(first, second);

    assert!(crate::find_shortcut_appid(path, "Foo").is_some());

    assert!(crate::find_shortcut_appid(path, "Bar").is_some());

}

#[test]
fn add_steam_shortcut_writes_a_rollback_copy_of_the_original_bytes() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    let original = crate::test_support::empty_shortcuts_vdf();

    std::fs::write(&vdf_path, &original).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS).expect("add foo");

    let backup = std::fs::read(dir.path().join("shortcuts.vdf.bak-fixed")).expect("read backup");

    assert_eq!(backup, original);

    assert_ne!(std::fs::read(&vdf_path).expect("read vdf"), original);

}

#[test]
fn add_steam_shortcut_leaves_the_backup_untouched_when_the_name_already_exists() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, crate::test_support::empty_shortcuts_vdf()).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    let first = add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS).expect("add foo");

    let backup_after_first = std::fs::read(dir.path().join("shortcuts.vdf.bak-fixed")).expect("read backup");

    let second = add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS).expect("add foo again");

    assert_eq!(first, second);

    assert_eq!(std::fs::read(dir.path().join("shortcuts.vdf.bak-fixed")).expect("read backup"), backup_after_first);

}

#[test]
fn add_steam_shortcut_refuses_to_mutate_when_the_backup_cannot_be_written() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    let original = crate::test_support::empty_shortcuts_vdf();

    std::fs::write(&vdf_path, &original).expect("write fixture");

    std::fs::create_dir(dir.path().join("shortcuts.vdf.bak-fixed")).expect("block backup path");

    let path = vdf_path.to_str().unwrap();

    let result = add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS);

    assert!(result.is_err());

    assert_eq!(std::fs::read(&vdf_path).expect("read vdf"), original);

}

#[test]
fn empty_shortcuts_vdf_fixture_is_a_correctly_terminated_empty_map() {

    let data = crate::test_support::empty_shortcuts_vdf();

    assert_eq!(&data[data.len() - 2..], [8u8, 8u8]);

    assert_eq!(crate::test_support::top_level_keys(&data), vec![String::from("shortcuts")]);

    assert!(crate::test_support::keys_under(&data, "shortcuts").is_empty());

}

#[test]
fn add_steam_shortcut_nests_new_entries_inside_shortcuts_not_as_document_root_siblings() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, crate::test_support::empty_shortcuts_vdf()).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS).expect("add foo");

    add_steam_shortcut(path, "Bar", "/games/Bar/Bar.exe", "/games/Bar/", ONLINE_FIX_LAUNCH_OPTIONS).expect("add bar");

    let data = std::fs::read(&vdf_path).expect("read vdf");

    assert_eq!(crate::test_support::top_level_keys(&data), vec![String::from("shortcuts")]);

    assert_eq!(crate::test_support::keys_under(&data, "shortcuts").len(), 2);

}
