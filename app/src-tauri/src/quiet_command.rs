use std::process::Command;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

// A GUI-subsystem app that spawns a console program (tasklist, reg, cmd) gets a console window
// flashed per call unless CREATE_NO_WINDOW is set; tasklist runs every 500ms while Steam boots.
pub fn quiet_command(program: impl AsRef<std::ffi::OsStr>) -> Command {

    #[allow(unused_mut)]
    let mut command = Command::new(program);

    #[cfg(windows)]
    {

        use std::os::windows::process::CommandExt;

        command.creation_flags(CREATE_NO_WINDOW);

    }

    command

}
