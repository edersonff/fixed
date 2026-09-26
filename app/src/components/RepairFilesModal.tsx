import { useSyncExternalStore } from "react";

import { AnimatePresence, motion } from "framer-motion";

import { ShieldAlert } from "lucide-react";

import { openUrl } from "@tauri-apps/plugin-opener";

import { useGameLaunch } from "../hooks/useGameLaunch";

import { useRepairFiles } from "../hooks/useRepairFiles";

import { displayTitle } from "../lib/format";

import { getRepairTitle, openRepair, subscribeRepair } from "../lib/repairStore";

import type { GameFilesStatus } from "../types";

const DEFENDER_SETTINGS_URL = "windowsdefender://threatsettings";

function headline(status: GameFilesStatus | null, game: string): string {

  const count = status?.missing.length ?? 0;

  const files = count === 1 ? "1 file" : `${count} files`;

  const who = status?.protectionOn === null ? "Your antivirus" : "Windows Defender";

  return `${who} deleted ${files} from ${game}`;

}

function ProtectionPill({ on }: { on: boolean | null | undefined }) {

  if (on === null || on === undefined) {

    return null;

  }

  return <span className={on ? "protection-pill on" : "protection-pill off"}>{on ? "Protection ON" : "Protection OFF"}</span>;

}

function RepairSteps({

  status,

  restoring,

  onRestore,

}: {

  status: GameFilesStatus;

  restoring: boolean;

  onRestore: () => void;

}) {

  const protectionOn = status.protectionOn === true;

  if (!status.canRestore) {

    return <p className="modal-hint">The game download was already deleted, so these files cannot be restored. Uninstall the game and download it again.</p>;

  }

  return (

    <ol className="repair-steps">

      <li className={protectionOn ? "current" : "done"}>

        <div>

          <strong>Turn off Real-time protection</strong>

          <ProtectionPill on={status.protectionOn} />

        </div>

        <button type="button" className="press-lift ghost" onClick={() => openUrl(DEFENDER_SETTINGS_URL).catch(() => undefined)}>

          Open Windows Security

        </button>

      </li>

      <li className={protectionOn ? "" : "current"}>

        <div>

          <strong>Get the files back</strong>

          {protectionOn && <span className="repair-warn">Turn protection off first, or Defender deletes them again.</span>}

        </div>

        <button type="button" className="press-lift hero-cta" disabled={restoring} onClick={onRestore}>

          {restoring ? "Restoring..." : "Restore files"}

        </button>

      </li>

    </ol>

  );

}

export function RepairFilesModal() {

  const title = useSyncExternalStore(subscribeRepair, getRepairTitle);

  const { status, restoring, restored, error, restore } = useRepairFiles(title);

  const { launch } = useGameLaunch(title ?? "");

  const game = displayTitle(title ?? "");

  const close = () => openRepair(null);

  return (

    <AnimatePresence>

      {title && (

        <motion.div className="modal-backdrop" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }} onClick={close}>

          <motion.div

            className="modal-card defender"

            role="dialog"

            aria-modal="true"

            aria-label="Game files removed"

            initial={{ opacity: 0, y: 14, scale: 0.97 }}

            animate={{ opacity: 1, y: 0, scale: 1 }}

            exit={{ opacity: 0, y: 10, scale: 0.98 }}

            onClick={(event) => event.stopPropagation()}

          >

            <span className="modal-icon">

              <ShieldAlert size={22} strokeWidth={1.9} />

            </span>

            {restored ? (

              <>

                <h2>All files are back</h2>

                <p className="modal-hint">{game} is ready. You can turn Real-time protection back on after you finish playing. If it deletes the files again, this window comes back.</p>

                <div className="modal-actions">

                  <button type="button" className="press-lift ghost" onClick={close}>Close</button>

                  <button type="button" className="press-lift hero-cta" onClick={() => { close(); launch(); }}>Play now</button>

                </div>

              </>

            ) : (

              <>

                <h2>{status ? headline(status, game) : "Checking game files..."}</h2>

                <p className="modal-hint">Antivirus apps often remove online-fix files by mistake. Without them the game cannot play online.</p>

                {status && <RepairSteps status={status} restoring={restoring} onRestore={restore} />}

                {error && <p className="repair-warn">{error}</p>}

                <div className="modal-actions">

                  <button type="button" className="press-lift ghost" onClick={close}>Later</button>

                </div>

              </>

            )}

          </motion.div>

        </motion.div>

      )}

    </AnimatePresence>

  );

}
