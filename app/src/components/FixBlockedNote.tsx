import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { FILES_REMOVED_AGAIN, fixErrorMessage } from "../lib/fixErrors";

const DECLINED_LINE = "Windows did not allow the folder. Press the button again and choose Yes.";

export function FixBlockedNote({ onAllowed }: { onAllowed: () => void }) {

  const [busy, setBusy] = useState(false);

  const [declined, setDeclined] = useState(false);

  return (

    <div className="fix-blocked">

      <button

        type="button"

        className="ghost"

        disabled={busy}

        onClick={(event: React.MouseEvent) => {

          event.stopPropagation();

          setBusy(true);

          setDeclined(false);

          invoke<boolean>("allow_games_folder")
            .then((allowed) => {

              if (allowed) {

                onAllowed();

              } else {

                setDeclined(true);

              }

            })
            .catch(() => setDeclined(true))
            .finally(() => setBusy(false));

        }}

      >

        {busy ? "Allowing…" : "Allow games folder"}

      </button>

      <p className="fix-blocked-line">{declined ? DECLINED_LINE : fixErrorMessage(FILES_REMOVED_AGAIN)}</p>

    </div>

  );

}
