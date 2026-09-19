use super::*;

#[test]
fn decode_cp1251_converts_cyrillic_bytes() {

    let bytes = [0xf2, 0xe5, 0xf1, 0xf2, 0x20, 0x31, 0x32, 0x33];

    assert_eq!(decode_cp1251(&bytes), "тест 123");

}

#[test]
fn decode_cp1251_falls_back_to_utf8_when_bytes_are_already_utf8() {

    let bytes = "hello".as_bytes();

    assert_eq!(decode_cp1251(bytes), "hello");

}

#[test]
fn category_from_url_reads_segment_after_games() {

    let url = "https://online-fix.me/games/survival/17320-dayz-po-seti.html";

    assert_eq!(category_from_url(url), "survival");

}

#[test]
fn category_from_url_defaults_to_unknown_without_games_segment() {

    let url = "https://online-fix.me/news/17320-something.html";

    assert_eq!(category_from_url(url), "unknown");

}

#[test]
fn last_number_returns_the_last_all_digit_token() {

    let info = "24 марта 2026, 01:34 4723716 91";

    assert_eq!(last_number(info), 91);

}

#[test]
fn last_number_skips_tokens_with_non_digit_characters() {

    assert_eq!(last_number("2026, 01:34 42"), 42);

}

#[test]
fn last_number_returns_zero_without_digits() {

    assert_eq!(last_number("марта 2026,"), 0);

}

#[test]
fn first_build_reads_digits_after_marker() {

    assert_eq!(first_build("Обновлено до Build 10092026.\n"), "10092026");

}

#[test]
fn first_build_empty_without_marker() {

    assert_eq!(first_build("no build info here"), "");

}

#[test]
fn lane_kind_classifies_known_hosts() {

    assert_eq!(lane_kind("https://hosters.online-fix.me:2053/Game"), Some("hosters"));

    assert_eq!(lane_kind("https://drive.online-fix.me:2053/Game"), Some("drive"));

    assert_eq!(lane_kind("https://pixeldrain.com/u/abc123"), Some("mirror"));

    assert_eq!(lane_kind("https://uploads.online-fix.me:2053/uploads/Game/"), Some("direct"));

    assert_eq!(lane_kind("https://uploads.online-fix.me:2053/torrents/Game/"), Some("torrent"));

    assert_eq!(lane_kind("https://example.com/"), None);

}

#[test]
fn ext_lane_kind_classifies_russian_labels() {

    assert_eq!(ext_lane_kind("скачать торрент"), Some("torrent"));

    assert_eq!(ext_lane_kind("mega.nz зеркало"), Some("mega"));

    assert_eq!(ext_lane_kind("yandex диск"), Some("yandex"));

    assert_eq!(ext_lane_kind("google drive"), Some("google-drive"));

    assert_eq!(ext_lane_kind("скачать"), Some("mirror"));

    assert_eq!(ext_lane_kind("unrelated text"), None);

}

#[test]
fn find_video_id_prefers_earlier_marker_in_the_fixed_list() {

    let html = "watch?v=dQw4w9WgXcQ later youtube-nocookie.com/embed/Z4GTWcOvyic?rel=0";

    assert_eq!(find_video_id(html), "Z4GTWcOvyic");

}

#[test]
fn find_video_id_empty_without_any_marker() {

    assert_eq!(find_video_id("no video here"), "");

}

#[test]
fn find_video_id_rejects_wrong_length_candidate() {

    let html = "youtube.com/embed/short";

    assert_eq!(find_video_id(html), "");

}

#[test]
fn parse_ru_date_maps_month_name_to_iso() {

    assert_eq!(parse_ru_date("15 сентября 2026"), "2026-09-15");

    assert_eq!(parse_ru_date("1 января 2025"), "2025-01-01");

}

#[test]
fn parse_ru_date_empty_on_unknown_month() {

    assert_eq!(parse_ru_date("15 zzz 2026"), "");

}
