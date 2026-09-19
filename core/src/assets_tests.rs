use super::*;

#[test]
fn steam_hero_url_targets_the_cloudflare_cdn_path() {

    assert_eq!(steam_hero_url(1144200), "https://cdn.cloudflare.steamstatic.com/steam/apps/1144200/library_hero.jpg");

}

#[test]
fn steam_logo_url_targets_the_cloudflare_cdn_path() {

    assert_eq!(steam_logo_url(1144200), "https://cdn.cloudflare.steamstatic.com/steam/apps/1144200/logo_2x.png");

}

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
fn assets_table_lists_the_four_expected_steam_cdn_files() {

    let files: Vec<(&str, &str)> = ASSETS.iter().map(|spec| (spec.kind, spec.file)).collect();

    assert_eq!(
        files,
        vec![
            ("cover", "library_600x900_2x.jpg"),
            ("logo", "logo_2x.png"),
            ("hero", "library_hero.jpg"),
            ("capsule", "capsule_616x353.jpg"),
        ]
    );

}

#[test]
fn appid_cache_round_trips_title_to_id_through_the_cache_file() {

    let dir = tempfile::tempdir().expect("tempdir");

    let previous_home = std::env::var("HOME").ok();

    std::env::set_var("HOME", dir.path());

    let before = load_cache();

    assert!(before.is_empty());

    let entries = vec![("halo".to_string(), 1234u32), ("doom".to_string(), 5678u32)];

    save_cache(&entries);

    let after = load_cache();

    match previous_home {
        Some(home) => std::env::set_var("HOME", home),
        None => std::env::remove_var("HOME"),
    }

    assert_eq!(after, entries);

}
