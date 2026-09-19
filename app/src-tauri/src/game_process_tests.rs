use super::parse_ps_game_pid;
use super::parse_tasklist_csv_pid;

const PS_SAMPLE: &str = "\
  4821 /home/user/.steam/steam/ubuntu12_32/reaper SteamLaunch AppId=2500492446 -- /home/user/.steam/steam/steamapps/common/SteamLinuxRuntime_sniper/_v2-entry-point --verb=waitforexitandrun -- /home/user/.steam/steam/steamapps/common/Proton Experimental/proton waitforexitandrun /home/user/games/BOMBANANA/BOMBANANA.exe
  4830 /home/user/.steam/steam/steamapps/common/SteamLinuxRuntime_sniper/pressure-vessel/bin/pressure-vessel-wrap --tmp-dir /tmp/pv
  4899 Z:\\home\\user\\games\\BOMBANANA\\BOMBANANA.exe
";

#[test]
fn parse_ps_game_pid_skips_the_wrapper_chain_and_finds_the_wine_process() {

    assert_eq!(parse_ps_game_pid(PS_SAMPLE, "BOMBANANA.exe"), Some(4899));

}

#[test]
fn parse_ps_game_pid_is_case_insensitive_on_the_exe_name() {

    assert_eq!(parse_ps_game_pid(PS_SAMPLE, "bombanana.exe"), Some(4899));

}

#[test]
fn parse_ps_game_pid_none_when_only_the_wrapper_chain_is_running() {

    let output = "  4821 /path/reaper SteamLaunch AppId=2500492446 -- /path/proton waitforexitandrun /home/user/games/BOMBANANA/BOMBANANA.exe\n";

    assert_eq!(parse_ps_game_pid(output, "BOMBANANA.exe"), None);

}

#[test]
fn parse_ps_game_pid_none_on_empty_output() {

    assert_eq!(parse_ps_game_pid("", "BOMBANANA.exe"), None);

}

#[test]
fn parse_tasklist_csv_pid_reads_the_pid_column() {

    let output = "\"BOMBANANA.exe\",\"4899\",\"Console\",\"1\",\"210,000 K\"\r\n";

    assert_eq!(parse_tasklist_csv_pid(output, "BOMBANANA.exe"), Some(4899));

}

#[test]
fn parse_tasklist_csv_pid_none_when_the_image_is_absent() {

    let output = "INFO: No tasks are running which match the specified criteria.\r\n";

    assert_eq!(parse_tasklist_csv_pid(output, "BOMBANANA.exe"), None);

}
