use super::*;

#[test]
fn vdf_string_matches_binary_vdf_type_one_layout() {

    let bytes = vdf_string("AppName", "Foo");

    assert_eq!(bytes, [1u8, b'A', b'p', b'p', b'N', b'a', b'm', b'e', 0, b'F', b'o', b'o', 0]);

}

#[test]
fn vdf_int_matches_binary_vdf_type_two_layout() {

    let bytes = vdf_int("appid", 300u32);

    let mut expected = vec![2u8];

    expected.extend_from_slice(b"appid");

    expected.push(0);

    expected.extend_from_slice(&300u32.to_le_bytes());

    assert_eq!(bytes, expected);

}

#[test]
fn find_subslice_finds_needle_after_given_offset() {

    let haystack = b"aaXbbXcc";

    assert_eq!(find_subslice(haystack, b"X", 0), Some(2));

    assert_eq!(find_subslice(haystack, b"X", 3), Some(5));

    assert_eq!(find_subslice(haystack, b"X", 6), None);

}
