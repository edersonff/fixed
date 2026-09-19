use super::*;

const MODERN_APP_FIXTURE: &str = include_str!("../tests/fixtures/getitems_4656000.json");

const LEGACY_APP_FIXTURE: &str = include_str!("../tests/fixtures/getitems_221100.json");

const NOT_FOUND_FIXTURE: &str = include_str!("../tests/fixtures/getitems_notfound.json");

#[test]
fn parse_get_items_reads_every_field_for_a_modern_appid() {

    let manifest = parse_get_items(MODERN_APP_FIXTURE);

    assert_eq!(manifest.asset_url_format.as_deref(), Some("steam/apps/4656000/${FILENAME}?t=1789738286"));

    assert_eq!(manifest.library_hero_2x.as_deref(), Some("6432b9026df018c6db23dd752352a79faf880ebc/library_hero_2x.jpg"));

    assert_eq!(manifest.hero_capsule.as_deref(), Some("641c5b2c093a8af5cca204609fc9018ebfbbcaef/hero_capsule.jpg"));

    assert_eq!(manifest.page_background_path.as_deref(), Some("app/4656000?t=1789738286"));

    assert_eq!(manifest.main_capsule.as_deref(), Some("5c8c9fb42a56e6fd25f2877053158fd207ffb028/capsule_616x353.jpg"));

}

#[test]
fn parse_get_items_reads_the_hash_less_legacy_appid_shape_too() {

    let manifest = parse_get_items(LEGACY_APP_FIXTURE);

    assert_eq!(manifest.library_hero.as_deref(), Some("library_hero.jpg"));

    assert_eq!(manifest.library_capsule_2x.as_deref(), Some("library_600x900_2x.jpg"));

    assert_eq!(manifest.header.as_deref(), Some("header.jpg"));

}

#[test]
fn parse_get_items_returns_all_none_when_the_appid_has_no_assets() {

    assert_eq!(parse_get_items(NOT_FOUND_FIXTURE), AssetManifest::default());

}

#[test]
fn hero_candidates_orders_library_hero_2x_first_then_falls_back_to_background_and_header() {

    let manifest = parse_get_items(MODERN_APP_FIXTURE);

    let candidates = hero_candidates(&manifest);

    assert_eq!(
        candidates,
        vec![
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/4656000/6432b9026df018c6db23dd752352a79faf880ebc/library_hero_2x.jpg?t=1789738286",
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/4656000/6432b9026df018c6db23dd752352a79faf880ebc/library_hero.jpg?t=1789738286",
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/4656000/641c5b2c093a8af5cca204609fc9018ebfbbcaef/hero_capsule.jpg?t=1789738286",
            "https://store.akamai.steamstatic.com/images/storepagebackground/app/4656000?t=1789738286",
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/4656000/99c086faba625a8d3bc459bab444087d257ccbe1/header.jpg?t=1789738286",
        ]
    );

}

#[test]
fn hero_candidates_substitutes_the_filename_template_for_the_hash_less_legacy_shape() {

    let manifest = parse_get_items(LEGACY_APP_FIXTURE);

    let candidates = hero_candidates(&manifest);

    assert_eq!(
        candidates,
        vec![
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/221100/library_hero_2x.jpg?t=1789118874",
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/221100/library_hero.jpg?t=1789118874",
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/221100/hero_capsule.jpg?t=1789118874",
            "https://store.akamai.steamstatic.com/images/storepagebackground/app/221100?t=1789118874",
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/221100/header.jpg?t=1789118874",
        ]
    );

}

#[test]
fn cover_candidates_orders_library_capsule_2x_first_then_falls_back_to_main_capsule() {

    let manifest = parse_get_items(MODERN_APP_FIXTURE);

    let candidates = cover_candidates(&manifest);

    assert_eq!(
        candidates,
        vec![
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/4656000/feaeb8f6d4b829252a902f0acd2d8fd385271871/library_capsule_2x.jpg?t=1789738286",
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/4656000/feaeb8f6d4b829252a902f0acd2d8fd385271871/library_capsule.jpg?t=1789738286",
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/4656000/5c8c9fb42a56e6fd25f2877053158fd207ffb028/capsule_616x353.jpg?t=1789738286",
        ]
    );

}

#[test]
fn hero_candidates_and_cover_candidates_are_empty_when_the_appid_has_no_assets() {

    let manifest = parse_get_items(NOT_FOUND_FIXTURE);

    assert!(hero_candidates(&manifest).is_empty());

    assert!(cover_candidates(&manifest).is_empty());

}
