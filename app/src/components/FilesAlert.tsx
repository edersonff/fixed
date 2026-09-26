import { AnimatePresence, motion } from "framer-motion";

import { ShieldAlert } from "lucide-react";

import { openUrl } from "@tauri-apps/plugin-opener";

import { useFilesAlert } from "../hooks/useFilesAlert";

import { alertHeadline, alertLine, anyCanRestore, restoreButtonLabel } from "../lib/filesAlert";

const DEFENDER_SETTINGS_URL = "windowsdefender://threatsettings";

export function FilesAlert() {

  const { visible, affected, status, removedAgain, hide, restore } = useFilesAlert();

  if (!visible) {

    return null;

  }

  const success = status === "success";

  const busy = status === "restoring" || status === "watching";

  return (

    <AnimatePresence>

      <motion.div

        className={success ? "files-alert files-alert-ok" : "files-alert"}

        role="alert"

        initial={{ opacity: 0, y: -10 }}

        animate={{ opacity: 1, y: 0 }}

        exit={{ opacity: 0, y: -10 }}

      >

        <span className="files-alert-icon">

          <ShieldAlert size={18} strokeWidth={2} />

        </span>

        <div className="files-alert-body">

          <strong>{success ? "All files are back" : alertHeadline(affected, removedAgain)}</strong>

          {!success && <p>{alertLine(affected, removedAgain)}</p>}

        </div>

        {!success && (

          <div className="files-alert-actions">

            {removedAgain && (

              <button type="button" className="ghost" onClick={() => openUrl(DEFENDER_SETTINGS_URL).catch(() => undefined)}>

                Open Windows Security

              </button>

            )}

            {anyCanRestore(affected) && (

              <button type="button" className="press-lift hero-cta" disabled={busy} onClick={restore}>

                {restoreButtonLabel(status)}

              </button>

            )}

            <button type="button" className="files-alert-hide" onClick={hide}>

              Hide

            </button>

          </div>

        )}

      </motion.div>

    </AnimatePresence>

  );

}
