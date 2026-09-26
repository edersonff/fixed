import { openUrl } from "@tauri-apps/plugin-opener";

const DEFENDER_SETTINGS_URL = "windowsdefender://threatsettings";

const PROTECTION_LINE = "Windows keeps deleting this game's files. Turn protection off, then press Fix.";

export function FixBlockedNote() {

  return (

    <div className="fix-blocked">

      <button type="button" className="ghost" onClick={() => openUrl(DEFENDER_SETTINGS_URL).catch(() => undefined)}>

        Turn off Windows protection

      </button>

      <p className="fix-blocked-line">{PROTECTION_LINE}</p>

    </div>

  );

}
