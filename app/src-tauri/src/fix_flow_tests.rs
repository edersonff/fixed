use super::*;

#[test]
fn search_key_ignores_case_and_punctuation() {

    assert_eq!(search_key("Baldur's Gate 3"), search_key("baldurs gate 3"));

}

#[test]
fn search_key_distinguishes_different_titles() {

    assert_ne!(search_key("Portal"), search_key("Portal 2"));

}
