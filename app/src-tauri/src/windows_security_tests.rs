use super::*;

#[test]
fn build_args_carries_the_file_and_games_folder_flags() {

    let args = build_args("C:\\FIXED\\windows-security\\allow-games-folder.ps1", "C:\\Users\\Eder\\games", false);

    assert_eq!(
        args,
        vec![
            "-NoProfile",
            "-ExecutionPolicy",
            "RemoteSigned",
            "-File",
            "C:\\FIXED\\windows-security\\allow-games-folder.ps1",
            "-GamesFolder",
            "C:\\Users\\Eder\\games",
        ],
    );

}

#[test]
fn build_args_appends_remove_only_when_asked() {

    let args = build_args("script.ps1", "games", true);

    assert_eq!(args.last().map(String::as_str), Some("-Remove"));

}

#[test]
fn build_args_omits_remove_by_default() {

    let args = build_args("script.ps1", "games", false);

    assert!(!args.contains(&String::from("-Remove")));

}

#[test]
fn ps_quote_doubles_an_embedded_single_quote() {

    assert_eq!(ps_quote("C:\\Users\\O'Brien\\games"), "'C:\\Users\\O''Brien\\games'");

}

#[test]
fn elevated_command_quotes_every_argument_and_waits_for_the_real_exit_code() {

    let command = elevated_command("C:\\FIXED\\windows-security\\allow-games-folder.ps1", "C:\\Users\\Eder\\games", false);

    assert!(command.contains("'-File','C:\\FIXED\\windows-security\\allow-games-folder.ps1'"));

    assert!(command.contains("'-GamesFolder','C:\\Users\\Eder\\games'"));

    assert!(command.contains("-Verb RunAs"));

    assert!(command.contains("-Wait -PassThru -WindowStyle Hidden"));

    assert!(command.contains("exit $p.ExitCode"));

}

#[test]
fn elevated_command_includes_remove_when_requested() {

    let command = elevated_command("script.ps1", "games", true);

    assert!(command.contains("'-Remove'"));

}

#[test]
fn script_relative_path_sits_beside_the_exe_in_its_own_subfolder() {

    assert_eq!(script_relative_path(), std::path::PathBuf::from("windows-security").join("allow-games-folder.ps1"));

}
