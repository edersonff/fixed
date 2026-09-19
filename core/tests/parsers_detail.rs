fn detail() -> fix_core::GameDetail {

    let bytes = std::fs::read("tests/fixtures/detail.html").expect("detail fixture readable");

    fix_core::parse_detail(&fix_core::decode_cp1251(&bytes))

}

#[test]
fn parse_detail_reads_title_and_build() {

    let detail = detail();

    assert_eq!(detail.title, "Ready or Not по сети");

    assert_eq!(detail.build, "10092026");

}

#[test]
fn parse_detail_finds_steam_ext_token_link() {

    let detail = detail();

    assert_eq!(
        detail.steam_ext_url,
        "https://online-fix.me/ext/R-vBLXs5iFbz_JmEpvGnKW-4n99sGU-VKZrR4xXUw5LgydNLP8q2pbm5FqjQ1R9BTvBXXYohxXaoEVnMMM3gsg=="
    );

}

#[test]
fn parse_detail_collects_every_download_lane_kind() {

    let detail = detail();

    let kinds: Vec<&str> = detail.lanes.iter().map(|lane| lane.kind.as_str()).collect();

    assert_eq!(kinds, vec!["hosters", "drive", "direct", "torrent"]);

    let hosters = detail.lanes.iter().find(|lane| lane.kind == "hosters").unwrap();

    assert_eq!(hosters.url, "https://hosters.online-fix.me:2053/Ready%20or%20Not");

    let torrent = detail.lanes.iter().find(|lane| lane.kind == "torrent").unwrap();

    assert_eq!(torrent.url, "https://uploads.online-fix.me:2053/torrents/Ready%20or%20Not/");

}

#[test]
fn parse_detail_flags_fix_repair_mention() {

    assert!(detail().mentions_fix_repair);

}

#[test]
fn parse_detail_picks_the_first_matching_video_marker_not_the_earliest_occurrence() {

    assert_eq!(detail().video_id, "Z4GTWcOvyic");

}
