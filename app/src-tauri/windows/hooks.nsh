!include "LogicLib.nsh"

; Windows removes online-fix's injected dlls from the games folder unless it is excluded from
; real-time scanning. Asking here, once, at install time, is the "definitive popup" the owner
; asked for instead of walking the player through the full Windows Security UI. The elevated
; script owns the HKLM marker: it writes it only when it truly added the exclusion, so a cancelled
; UAC prompt or a folder the user had already excluded never leaves a marker behind.
!macro NSIS_HOOK_POSTINSTALL
    IfSilent allow_games_folder_skip

    MessageBox MB_YESNO|MB_ICONQUESTION "Allow the FIXED games folder in Windows Security?$\r$\nWindows removes the files that make games play online unless this folder is allowed. (Recommended)" IDNO allow_games_folder_skip

    ExecShellWait "runas" "powershell.exe" '-NoProfile -ExecutionPolicy RemoteSigned -File "$INSTDIR\windows-security\allow-games-folder.ps1" -GamesFolder "$PROFILE\games"' SW_HIDE

    allow_games_folder_skip:
!macroend

; Only re-scans the folder if this install (or the app's runtime prompt) is the one that excluded
; it: the marker lives in the 64-bit HKLM view the elevated script wrote to, and the script's own
; -Remove path re-checks the marker before touching Defender, so an exclusion the user made
; themselves is never removed.
!macro NSIS_HOOK_PREUNINSTALL
    SetRegView 64

    ReadRegDWORD $0 HKLM "SOFTWARE\FIXED" "GamesFolderAllowed"

    ${If} $0 == 1
        ExecShellWait "runas" "powershell.exe" '-NoProfile -ExecutionPolicy RemoteSigned -File "$INSTDIR\windows-security\allow-games-folder.ps1" -GamesFolder "$PROFILE\games" -Remove' SW_HIDE
    ${EndIf}
!macroend
