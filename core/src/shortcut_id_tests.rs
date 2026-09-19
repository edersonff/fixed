use super::*;

#[test]
fn shortcut_appid_matches_fnv1a_golden_value() {

    assert_eq!(shortcut_appid("/games/Foo/Foo.exe", "Foo"), 2975550365);

    assert_eq!(shortcut_appid("/games/Friendly Steps/Friendly Steps.exe", "Friendly Steps"), 2705359545);

}

#[test]
fn shortcut_appid_sets_top_bit_even_when_the_raw_hash_does_not() {

    assert_eq!(shortcut_appid("/x", "Probe10"), 0x902b7925);

}

#[test]
fn shortcut_appid_changes_with_input() {

    let a = shortcut_appid("/games/Foo/Foo.exe", "Foo");

    let b = shortcut_appid("/games/Bar/Bar.exe", "Bar");

    assert_ne!(a, b);

}

#[test]
fn shortcut_gameid_shifts_the_appid_into_the_high_word_with_the_shortcut_marker() {

    assert_eq!(shortcut_gameid(3299031949), 14169234329447694336);

    assert_eq!(shortcut_gameid(3082930001), 13241083530185801728);

    assert_eq!(shortcut_gameid(1), 0x0000_0001_0200_0000);

}
