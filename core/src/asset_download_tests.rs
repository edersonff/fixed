use super::*;

#[test]
fn extension_for_maps_png_content_type() {

    assert_eq!(extension_for("image/png"), "png");

}

#[test]
fn extension_for_defaults_to_jpg_for_anything_else() {

    assert_eq!(extension_for("image/jpeg"), "jpg");

    assert_eq!(extension_for(""), "jpg");

}

#[test]
fn save_app_assets_reports_none_and_writes_nothing_when_every_url_resolved_empty() {

    let _guard = crate::test_support::HOME_ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let previous_home = std::env::var("HOME").ok();

    let home = tempfile::tempdir().expect("home tempdir");

    std::env::set_var("HOME", home.path());

    crate::asset_cache::save_asset_cache(&[(999_999_999, crate::asset_cache::ResolvedAssets::default())]);

    let out = tempfile::tempdir().expect("out tempdir");

    let saved = save_app_assets(999_999_999, out.path().to_str().unwrap()).expect("save");

    match previous_home {

        Some(value) => std::env::set_var("HOME", value),

        None => std::env::remove_var("HOME"),

    }

    assert_eq!(saved, vec!["hero none", "cover none", "logo none"]);

    let written: Vec<_> = std::fs::read_dir(out.path().join("999999999")).expect("read dir").collect();

    assert!(written.is_empty());

}
