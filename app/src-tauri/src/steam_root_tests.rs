use super::*;

#[test]
fn parse_reg_sz_value_reads_steam_path_from_reg_query_output() {

    let output = "\r\nHKEY_CURRENT_USER\\Software\\Valve\\Steam\r\n    SteamPath    REG_SZ    C:\\Program Files (x86)\\Steam\r\n\r\n";

    assert_eq!(
        parse_reg_sz_value(output, "SteamPath"),
        Some(String::from("C:\\Program Files (x86)\\Steam")),
    );

}

#[test]
fn parse_reg_sz_value_returns_none_when_value_missing() {

    let output = "ERROR: The system was unable to find the specified registry key or value.\r\n";

    assert_eq!(parse_reg_sz_value(output, "SteamPath"), None);

}
