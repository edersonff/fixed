use super::*;

fn empty_shortcuts_vdf() -> Vec<u8> {

    let mut data = Vec::new();

    data.push(0);

    data.extend_from_slice(b"shortcuts");

    data.push(0);

    data.push(8);

    data.push(8);

    data

}

#[test]
fn shortcut_appid_matches_fnv1a_golden_value() {

    assert_eq!(shortcut_appid("/games/Foo/Foo.exe", "Foo"), 2975550365);

    assert_eq!(shortcut_appid("/games/Friendly Steps/Friendly Steps.exe", "Friendly Steps"), 2705359545);

}

#[test]
fn shortcut_appid_sets_top_bit_even_when_the_raw_hash_does_not() {

    assert_eq!(shortcut_appid("/x", "Probe10"), 0x902b7925);

}

#[test]
fn shortcut_appid_changes_with_input() {

    let a = shortcut_appid("/games/Foo/Foo.exe", "Foo");

    let b = shortcut_appid("/games/Bar/Bar.exe", "Bar");

    assert_ne!(a, b);

}

#[test]
fn vdf_string_matches_binary_vdf_type_one_layout() {

    let bytes = vdf_string("AppName", "Foo");

    assert_eq!(bytes, [1u8, b'A', b'p', b'p', b'N', b'a', b'm', b'e', 0, b'F', b'o', b'o', 0]);

}

#[test]
fn vdf_int_matches_binary_vdf_type_two_layout() {

    let bytes = vdf_int("appid", 300u32);

    let mut expected = vec![2u8];

    expected.extend_from_slice(b"appid");

    expected.push(0);

    expected.extend_from_slice(&300u32.to_le_bytes());

    assert_eq!(bytes, expected);

}

#[test]
fn find_subslice_finds_needle_after_given_offset() {

    let haystack = b"aaXbbXcc";

    assert_eq!(find_subslice(haystack, b"X", 0), Some(2));

    assert_eq!(find_subslice(haystack, b"X", 3), Some(5));

    assert_eq!(find_subslice(haystack, b"X", 6), None);

}

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

    std::fs::write(&vdf_path, empty_shortcuts_vdf()).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    let index = add_steam_shortcut(path, "Friendly Steps", "/games/Friendly Steps/Friendly Steps.exe", "/games/Friendly Steps/", ONLINE_FIX_LAUNCH_OPTIONS)
        .expect("add shortcut");

    assert_eq!(index, 1);

    let expected_appid = shortcut_appid("/games/Friendly Steps/Friendly Steps.exe", "Friendly Steps");

    let found = find_shortcut_appid(path, "Friendly Steps").expect("shortcut findable");

    assert_eq!(found, expected_appid);

}

#[test]
fn add_steam_shortcut_is_idempotent_for_the_same_app_name() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, empty_shortcuts_vdf()).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    let first = add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS).expect("first add");

    let second = add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS).expect("second add");

    assert_eq!(first, second);

}

#[test]
fn add_steam_shortcut_assigns_increasing_indices_for_distinct_apps() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, empty_shortcuts_vdf()).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    let first = add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS).expect("add foo");

    let second = add_steam_shortcut(path, "Bar", "/games/Bar/Bar.exe", "/games/Bar/", ONLINE_FIX_LAUNCH_OPTIONS).expect("add bar");

    assert_eq!(first, 1);

    assert_eq!(second, 2);

    assert!(find_shortcut_appid(path, "Foo").is_some());

    assert!(find_shortcut_appid(path, "Bar").is_some());

}

#[test]
fn find_shortcut_appid_returns_none_when_app_name_absent() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, empty_shortcuts_vdf()).expect("write fixture");

    assert_eq!(find_shortcut_appid(vdf_path.to_str().unwrap(), "Nothing Here"), None);

}

#[test]
fn add_steam_shortcut_writes_a_rollback_copy_of_the_original_bytes() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    let original = empty_shortcuts_vdf();

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

    std::fs::write(&vdf_path, empty_shortcuts_vdf()).expect("write fixture");

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

    let original = empty_shortcuts_vdf();

    std::fs::write(&vdf_path, &original).expect("write fixture");

    std::fs::create_dir(dir.path().join("shortcuts.vdf.bak-fixed")).expect("block backup path");

    let path = vdf_path.to_str().unwrap();

    let result = add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", ONLINE_FIX_LAUNCH_OPTIONS);

    assert!(result.is_err());

    assert_eq!(std::fs::read(&vdf_path).expect("read vdf"), original);

}
