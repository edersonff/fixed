use super::*;

#[test]
fn find_shortcut_appid_returns_none_when_app_name_absent() {

    let dir = tempfile::tempdir().expect("tempdir");

    let vdf_path = dir.path().join("shortcuts.vdf");

    std::fs::write(&vdf_path, crate::test_support::empty_shortcuts_vdf()).expect("write fixture");

    assert_eq!(find_shortcut_appid(vdf_path.to_str().unwrap(), "Nothing Here"), None);

}
