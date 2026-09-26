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

fn entry_indices(data: &[u8]) -> Vec<u32> {

    let marker: &[u8] = b"\x02appid\x00";

    let mut indices = Vec::new();

    let mut pos = 0usize;

    while let Some(found) = find_subslice(data, marker, pos) {

        if let Some(digits_end) = found.checked_sub(1) {

            if let Some(start) = digits_before(data, digits_end) {

                if let Ok(text) = std::str::from_utf8(&data[start..digits_end]) {

                    if let Ok(value) = text.parse() {

                        indices.push(value);

                    }

                }

            }

        }

        pos = found + marker.len();

    }

    indices

}

#[test]
fn remove_steam_shortcut_keeps_the_other_entry_findable_and_renumbers_from_zero() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, empty_shortcuts_vdf()).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    crate::add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", crate::ONLINE_FIX_LAUNCH_OPTIONS).expect("add foo");

    crate::add_steam_shortcut(path, "Bar", "/games/Bar/Bar.exe", "/games/Bar/", crate::ONLINE_FIX_LAUNCH_OPTIONS).expect("add bar");

    let removed = remove_steam_shortcut(path, "Foo").expect("remove foo");

    assert!(removed);

    assert!(crate::find_shortcut_appid(path, "Foo").is_none());

    assert!(crate::find_shortcut_appid(path, "Bar").is_some());

    let data = std::fs::read(&vdf_path).expect("read vdf");

    assert_eq!(entry_indices(&data), vec![0]);

}

#[test]
fn remove_steam_shortcut_returns_false_and_leaves_the_file_untouched_when_the_name_is_absent() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    let original = empty_shortcuts_vdf();

    std::fs::write(&vdf_path, &original).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    let removed = remove_steam_shortcut(path, "Nothing Here").expect("remove call");

    assert!(!removed);

    let after = std::fs::read(&vdf_path).expect("read vdf after");

    assert_eq!(original, after);

    assert!(!dir.path().join("shortcuts.vdf.bak-fixed").exists());

}

#[test]
fn remove_steam_shortcut_writes_a_rollback_copy_of_the_original_bytes() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, empty_shortcuts_vdf()).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    crate::add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", crate::ONLINE_FIX_LAUNCH_OPTIONS).expect("add foo");

    let before_remove = std::fs::read(&vdf_path).expect("read vdf before remove");

    remove_steam_shortcut(path, "Foo").expect("remove foo");

    let backup = std::fs::read(dir.path().join("shortcuts.vdf.bak-fixed")).expect("read backup");

    assert_eq!(backup, before_remove);

}

#[test]
fn remove_steam_shortcut_with_three_entries_keeps_the_survivors_contiguous() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, empty_shortcuts_vdf()).expect("write fixture");

    let path = vdf_path.to_str().unwrap();

    crate::add_steam_shortcut(path, "Foo", "/games/Foo/Foo.exe", "/games/Foo/", crate::ONLINE_FIX_LAUNCH_OPTIONS).expect("add foo");

    crate::add_steam_shortcut(path, "Bar", "/games/Bar/Bar.exe", "/games/Bar/", crate::ONLINE_FIX_LAUNCH_OPTIONS).expect("add bar");

    crate::add_steam_shortcut(path, "Baz", "/games/Baz/Baz.exe", "/games/Baz/", crate::ONLINE_FIX_LAUNCH_OPTIONS).expect("add baz");

    remove_steam_shortcut(path, "Bar").expect("remove bar");

    let data = std::fs::read(&vdf_path).expect("read vdf");

    let mut indices = entry_indices(&data);

    indices.sort();

    assert_eq!(indices, vec![0, 1]);

    assert!(crate::find_shortcut_appid(path, "Foo").is_some());

    assert!(crate::find_shortcut_appid(path, "Baz").is_some());

    assert!(crate::find_shortcut_appid(path, "Bar").is_none());

}

#[test]
fn remove_by_appid_drops_only_that_entry_whatever_its_name() {

    let dir = tempfile::tempdir().unwrap();

    let path = dir.path().join("shortcuts.vdf");

    std::fs::write(&path, empty_shortcuts_vdf()).unwrap();

    let path = path.to_str().unwrap();

    let colon = crate::add_steam_shortcut(path, "Game: Subtitle", "/g/Game_ Subtitle/g.exe", "/g/Game_ Subtitle/", "").unwrap();

    crate::add_steam_shortcut(path, "Other", "/g/Other/o.exe", "/g/Other/", "").unwrap();

    assert!(remove_steam_shortcut_by_appid(path, colon).unwrap());

    assert_eq!(crate::find_shortcut_appid(path, "Game: Subtitle"), None);

    assert!(crate::find_shortcut_appid(path, "Other").is_some());

    assert!(!remove_steam_shortcut_by_appid(path, colon).unwrap());

}
