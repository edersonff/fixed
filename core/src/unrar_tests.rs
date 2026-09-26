use super::*;

#[test]
fn strips_the_literal_x0d_suffix_unrar_appends() {

    let raw = "BOMBANANA!\\baselib.dll\\x0d\n";

    assert_eq!(parse_listing(raw), vec![String::from("BOMBANANA!\\baselib.dll")]);

}

#[test]
fn strips_a_real_carriage_return_too() {

    let raw = "BOMBANANA!\\baselib.dll\r\n";

    assert_eq!(parse_listing(raw), vec![String::from("BOMBANANA!\\baselib.dll")]);

}
