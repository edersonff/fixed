import { openUrl } from "@tauri-apps/plugin-opener";

import { FILES_REMOVED_AGAIN, fixErrorMessage } from "../lib/fixErrors";

const DEFENDER_SETTINGS_URL = "windowsdefender://threatsettings";

export function FixBlockedNote() {

  return (

    <div className="fix-blocked">

      <button

        type="button"

        className="ghost"

        onClick={(event: React.MouseEvent) => {

          event.stopPropagation();

          openUrl(DEFENDER_SETTINGS_URL).catch(() => undefined);

        }}

      >

        Turn off Windows protection

      </button>

      <p className="fix-blocked-line">{fixErrorMessage(FILES_REMOVED_AGAIN)}</p>

    </div>

  );

}
