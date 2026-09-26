#[cfg(windows)]
pub fn realtime_protection_on() -> Option<bool> {

    let output = crate::quiet_command::quiet_command("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", "(Get-MpComputerStatus).RealTimeProtectionEnabled"])
        .output()
        .ok()?;

    parse_bool_line(&String::from_utf8_lossy(&output.stdout))

}

#[cfg(not(windows))]
pub fn realtime_protection_on() -> Option<bool> {

    None

}

#[cfg(any(windows, test))]
pub(crate) fn parse_bool_line(text: &str) -> Option<bool> {

    match text.trim() {

        "True" => Some(true),

        "False" => Some(false),

        _ => None,

    }

}
