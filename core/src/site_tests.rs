use super::*;

const BASE: &str = "https://uploads.online-fix.me:2053/uploads/Game/";

#[test]
fn absolutize_leaves_absolute_urls_untouched() {

    assert_eq!(absolutize(BASE, "https://mirror.example.com/x.rar"), "https://mirror.example.com/x.rar");

    assert_eq!(absolutize(BASE, "http://mirror.example.com/x.rar"), "http://mirror.example.com/x.rar");

}

#[test]
fn absolutize_joins_dot_slash_relative_paths_onto_the_trimmed_base() {

    assert_eq!(absolutize(BASE, "./part1.rar"), "https://uploads.online-fix.me:2053/uploads/Game/part1.rar");

}

#[test]
fn absolutize_prefixes_root_relative_paths_with_the_base_origin() {

    assert_eq!(
        absolutize(BASE, "/files/part1.rar"),
        "https://uploads.online-fix.me:2053/files/part1.rar"
    );

}

#[test]
fn absolutize_joins_plain_relative_paths_onto_the_trimmed_base() {

    assert_eq!(absolutize(BASE, "part1.rar"), "https://uploads.online-fix.me:2053/uploads/Game/part1.rar");

}
