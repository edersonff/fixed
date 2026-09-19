use super::*;

#[test]
fn appid_cache_round_trips_title_to_id_through_the_cache_file() {

    let _guard = crate::test_support::HOME_ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

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

#[test]
fn search_terms_keeps_the_raw_title_first_then_the_bracket_stripped_one() {

    assert_eq!(search_terms("Dayz (dayzavr)"), vec!["Dayz (dayzavr)", "Dayz"]);

    assert_eq!(search_terms("Palworld [v0.3.1] (repack)"), vec!["Palworld [v0.3.1] (repack)", "Palworld"]);

}

#[test]
fn search_terms_returns_one_term_when_there_is_nothing_to_strip() {

    assert_eq!(search_terms("Friendly Steps"), vec!["Friendly Steps"]);

}

#[test]
fn search_terms_drops_a_title_that_is_entirely_bracketed_rather_than_searching_for_nothing() {

    assert_eq!(search_terms("(dayzavr)"), vec!["(dayzavr)"]);

}
