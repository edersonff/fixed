use super::*;

fn game(title: &str) -> fix_core::GameEntry {

    fix_core::GameEntry {
        title: title.to_string(),
        page_url: format!("https://online-fix.me/{}.html", title),
        poster_url: String::new(),
        category: String::new(),
        published_at: String::new(),
        views: 0,
        comments: 0,
    }

}

#[test]
fn normalized_title_ignores_case_and_punctuation() {

    assert_eq!(normalized_title("Baldur's Gate 3"), normalized_title("baldurs gate 3"));

}

#[test]
fn normalized_title_distinguishes_different_titles() {

    assert_ne!(normalized_title("Portal"), normalized_title("Portal 2"));

}

#[test]
fn exact_title_match_finds_the_identical_title() {

    let games = vec![game("Bombanana Deluxe"), game("Portal 2")];

    let found = exact_title_match("Bombanana Deluxe", &games).expect("match expected");

    assert_eq!(found.title, "Bombanana Deluxe");

}

#[test]
fn exact_title_match_returns_none_when_absent() {

    let games = vec![game("Portal 2")];

    assert!(exact_title_match("Half-Life 3", &games).is_none());

}

#[test]
fn exact_title_match_rejects_a_near_miss() {

    let games = vec![game("BOMBANANA!")];

    assert!(exact_title_match("Bombanana Deluxe", &games).is_none());

}

#[test]
fn decide_fix_restores_when_archive_present() {

    let entry = game("Portal 2");

    assert_eq!(decide_fix(true, None, Some(&entry)), FixDecision::Restore);

}

#[test]
fn decide_fix_redownloads_from_the_kept_source_marker() {

    let decision = decide_fix(false, Some("https://online-fix.me/portal-2.html"), None);

    assert_eq!(decision, FixDecision::Redownload { page_url: String::from("https://online-fix.me/portal-2.html") });

}

#[test]
fn decide_fix_redownloads_from_an_exact_match_when_marker_missing() {

    let entry = game("Portal 2");

    let decision = decide_fix(false, None, Some(&entry));

    assert_eq!(decision, FixDecision::Redownload { page_url: entry.page_url.clone() });

}

#[test]
fn decide_fix_needs_the_game_page_with_no_archive_no_marker_and_no_match() {

    assert_eq!(decide_fix(false, None, None), FixDecision::GamePageNeeded);

}
