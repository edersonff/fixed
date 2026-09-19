fn search_entries() -> Vec<fix_core::GameEntry> {

    let html = std::fs::read_to_string("tests/fixtures/search.html").expect("search fixture readable");

    fix_core::parse_search(&html)

}

#[test]
fn parse_search_reads_absolute_href_entry() {

    let entries = search_entries();

    let first = &entries[0];

    assert_eq!(first.title, "Ready or Not");

    assert_eq!(first.page_url, "https://online-fix.me/games/shooter/111-ready-or-not-po-seti.html");

    assert_eq!(first.poster_url, "https://online-fix.me/uploads/posts/2026-01/1_poster.jpg");

    assert_eq!(first.category, "shooter");

    assert_eq!(first.published_at, "2026-09-15");

    assert_eq!(first.views, 0);

    assert_eq!(first.comments, 0);

}

#[test]
fn parse_search_prefixes_relative_href_with_site_origin() {

    let entries = search_entries();

    let second = &entries[1];

    assert_eq!(second.title, "Starsand Island");

    assert_eq!(second.page_url, "https://online-fix.me/games/sandbox/222-starsand-island-po-seti.html");

    assert_eq!(second.poster_url, "/uploads/posts/2026-02/2_poster.jpg");

    assert_eq!(second.category, "sandbox");

    assert_eq!(second.published_at, "2025-01-01");

}
