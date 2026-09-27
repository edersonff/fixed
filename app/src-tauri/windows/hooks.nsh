!include "LogicLib.nsh"

; Windows removes online-fix's injected dlls from the games folder unless it is excluded from
; real-time scanning. Asking here, once, at install time, is the "definitive popup" the owner
; asked for instead of walking the player through the full Windows Security UI.
!macro NSIS_HOOK_POSTINSTALL
    IfSilent allow_games_folder_skip

    MessageBox MB_YESNO|MB_ICONQUESTION "Allow the FIXED games folder in Windows Security?$\r$\nWindows removes the files that make games play online unless this folder is allowed. (Recommended)" IDNO allow_games_folder_skip

    ExecShellWait "runas" "powershell.exe" '-NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\windows-security\allow-games-folder.ps1" -GamesFolder "$PROFILE\games"' SW_HIDE

    WriteRegDWORD HKCU "Software\FIXED" "GamesFolderAllowed" 1

    allow_games_folder_skip:
!macroend

; Only asks Windows to re-scan the folder if this installer (or the app's own runtime prompt) is
; the one that excluded it — an install that never touched Defender leaves it untouched at uninstall too.
!macro NSIS_HOOK_PREUNINSTALL
    ReadRegDWORD $0 HKCU "Software\FIXED" "GamesFolderAllowed"

    ${If} $0 == 1
        ExecShellWait "runas" "powershell.exe" '-NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\windows-security\allow-games-folder.ps1" -GamesFolder "$PROFILE\games" -Remove' SW_HIDE

        DeleteRegValue HKCU "Software\FIXED" "GamesFolderAllowed"
    ${EndIf}
!macroend
