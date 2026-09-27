import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { FILES_REMOVED_AGAIN, fixErrorMessage } from "../lib/fixErrors";

export function FixBlockedNote({ onAllowed }: { onAllowed: () => void }) {

  const [busy, setBusy] = useState(false);

  return (

    <div className="fix-blocked">

      <button

        type="button"

        className="ghost"

        disabled={busy}

        onClick={(event: React.MouseEvent) => {

          event.stopPropagation();

          setBusy(true);

          invoke<boolean>("allow_games_folder")
            .then((allowed) => {

              if (allowed) {

                onAllowed();

              }

            })
            .catch(() => undefined)
            .finally(() => setBusy(false));

        }}

      >

        Allow games folder

      </button>

      <p className="fix-blocked-line">{fixErrorMessage(FILES_REMOVED_AGAIN)}</p>

    </div>

  );

}
