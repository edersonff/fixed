fn home_entries() -> Vec<fix_core::GameEntry> {

    let bytes = std::fs::read("tests/fixtures/home.html").expect("home fixture readable");

    fix_core::parse_home(&fix_core::decode_cp1251(&bytes))

}

#[test]
fn parse_home_finds_every_article_on_the_page() {

    let entries = home_entries();

    assert_eq!(entries.len(), 21);

}

#[test]
fn parse_home_reads_first_article_fields() {

    let entries = home_entries();

    let first = &entries[0];

    assert_eq!(first.title, "DayZ (DayZavr)");

    assert_eq!(first.page_url, "https://online-fix.me/games/survival/17320-dayz-dayzavr-po-seti.html");

    assert_eq!(first.poster_url, "https://online-fix.me/uploads/posts/2026-04/3512073803_poster.jpg");

    assert_eq!(first.category, "survival");

    assert_eq!(first.published_at, "2026-03-24T01:34:30+03:00");

    assert_eq!(first.comments, 91);

    assert_eq!(first.views, 4723716);

}

#[test]
fn parse_home_handles_zero_comments_without_eating_the_view_count() {

    let entries = home_entries();

    let second = &entries[1];

    assert_eq!(second.title, "Starsand Island");

    assert_eq!(second.category, "sandbox");

    assert_eq!(second.comments, 0);

    assert_eq!(second.views, 5505);

}
