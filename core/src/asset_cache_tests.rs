use super::*;

#[test]
fn format_asset_cache_line_then_parse_asset_cache_line_round_trips() {

    let resolved = ResolvedAssets {
        hero_url: "https://example.com/hero.jpg".to_string(),
        logo_url: String::new(),
        cover_url: "https://example.com/cover.jpg".to_string(),
    };

    let line = format_asset_cache_line(4656000, &resolved);

    let (appid, parsed) = parse_asset_cache_line(line.trim_end()).expect("line parses");

    assert_eq!(appid, 4656000);

    assert_eq!(parsed, resolved);

}

#[test]
fn parse_asset_cache_line_rejects_a_line_missing_fields() {

    assert!(parse_asset_cache_line("4656000\tonly-hero").is_none());

}

#[test]
fn asset_cache_round_trips_resolved_urls_through_the_cache_file() {

    let _guard = crate::test_support::HOME_ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let dir = tempfile::tempdir().expect("tempdir");

    let previous_home = std::env::var("HOME").ok();

    std::env::set_var("HOME", dir.path());

    let before = load_asset_cache();

    assert!(before.is_empty());

    let resolved = ResolvedAssets {
        hero_url: "https://example.com/hero.jpg".to_string(),
        logo_url: "https://example.com/logo.png".to_string(),
        cover_url: String::new(),
    };

    save_asset_cache(&[(4656000, resolved.clone())]);

    let after = load_asset_cache();

    match previous_home {
        Some(home) => std::env::set_var("HOME", home),
        None => std::env::remove_var("HOME"),
    }

    assert_eq!(after, vec![(4656000, resolved)]);

}

#[test]
fn asset_cache_ignores_entries_written_by_an_older_schema_version() {

    let _guard = crate::test_support::HOME_ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let dir = tempfile::tempdir().expect("tempdir");

    let previous_home = std::env::var("HOME").ok();

    std::env::set_var("HOME", dir.path());

    let path = asset_cache_path().expect("cache path");

    fs::write(&path, "v0\n4656000\tstale\tstale\tstale\n").expect("write stale cache");

    let loaded = load_asset_cache();

    match previous_home {
        Some(home) => std::env::set_var("HOME", home),
        None => std::env::remove_var("HOME"),
    }

    assert!(loaded.is_empty());

}
