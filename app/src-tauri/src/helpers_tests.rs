use super::*;

#[test]
fn safe_title_strips_every_char_windows_rejects() {

    assert_eq!(safe_title("Subtitle: Edition?"), "Subtitle_ Edition_");

    assert_eq!(safe_title("A/B\\C|D*E\"F<G>H"), "A_B_C_D_E_F_G_H");

}

#[test]
fn safe_title_keeps_legal_titles_untouched() {

    assert_eq!(safe_title("BOMBANANA!"), "BOMBANANA!");

    assert_eq!(safe_title("Tom Clancy's Rainbow Six® Siege"), "Tom Clancy's Rainbow Six® Siege");

}

#[test]
fn safe_title_fixes_trailing_dots_and_reserved_names() {

    assert_eq!(safe_title("Wait..."), "Wait");

    assert_eq!(safe_title("CON"), "CON_");

    assert_eq!(safe_title("nul.txt"), "nul.txt_");

    assert_eq!(safe_title("???"), "___");

    assert_eq!(safe_title("..."), "_");

}

#[test]
fn start_dir_ends_with_the_platform_separator() {

    let exe = std::path::Path::new("games").join("Foo").join("Foo.exe");

    let expected = format!("{}{}", std::path::Path::new("games").join("Foo").display(), std::path::MAIN_SEPARATOR);

    assert_eq!(start_dir_of(&exe.to_string_lossy()), expected);

}
