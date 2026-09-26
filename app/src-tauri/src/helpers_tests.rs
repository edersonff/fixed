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

#[cfg(unix)]
#[test]
fn game_folder_reuses_an_existing_install_whose_raw_title_was_legal() {

    let _guard = crate::test_support::HOME_ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let home = tempfile::tempdir().unwrap();

    let previous = std::env::var("HOME").ok();

    std::env::set_var("HOME", home.path());

    let fresh = game_folder("Game: Subtitle").unwrap();

    std::fs::create_dir_all(home.path().join("games").join("Game: Subtitle")).unwrap();

    let legacy = game_folder("Game: Subtitle").unwrap();

    if let Some(value) = previous {

        std::env::set_var("HOME", value);

    }

    assert!(fresh.ends_with("Game_ Subtitle"));

    assert!(legacy.ends_with("Game: Subtitle"));

}
