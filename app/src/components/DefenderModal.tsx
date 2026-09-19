import { useState } from "react";

import { AnimatePresence, motion } from "framer-motion";

import { ShieldAlert } from "lucide-react";

import { openUrl } from "@tauri-apps/plugin-opener";

import { liftOnHover, pressDown } from "../lib/motion";

const STEPS = [

  "Settings > Privacy & Security > Windows Security",

  "Virus & threat protection > Manage settings",

  "Turn Real-time protection OFF",

  "Install your game, then turn it back ON",

];

export function DefenderModal({ open, onClose }: { open: boolean; onClose: () => void }) {

  const [opening, setOpening] = useState(false);

  return (

    <AnimatePresence>

      {open && (

        <motion.div

          className="modal-backdrop"

          initial={{ opacity: 0 }}

          animate={{ opacity: 1 }}

          exit={{ opacity: 0 }}

          transition={{ duration: 0.18 }}

          onClick={onClose}

        >

          <motion.div

            className="modal-card defender"

            role="dialog"

            aria-modal="true"

            aria-label="Windows Defender notice"

            initial={{ opacity: 0, y: 14, scale: 0.97, filter: "blur(6px)" }}

            animate={{ opacity: 1, y: 0, scale: 1, filter: "blur(0px)" }}

            exit={{ opacity: 0, y: 10, scale: 0.98, filter: "blur(4px)" }}

            transition={{ duration: 0.22, ease: [0.05, 0.7, 0.1, 1] }}

            onClick={(event) => event.stopPropagation()}

          >

            <span className="modal-icon">

              <ShieldAlert size={22} strokeWidth={1.9} />

            </span>

            <h2>Before installing on Windows</h2>

            <p className="modal-hint">

              Windows Defender may flag game setups and delete files mid install. Pause

              real-time protection while FIXED installs:

            </p>

            <ol className="modal-steps">

              {STEPS.map((step) => (

                <li key={step}>{step}</li>

              ))}

            </ol>

            <div className="modal-actions">

              <motion.button

                type="button"

                className="ghost"

                whileHover={liftOnHover}

                whileTap={pressDown}

                disabled={opening}

                onClick={() => {

                  setOpening(true);

                  openUrl("ms-settings:windowsdefender").catch(() => undefined).finally(() => setOpening(false));

                }}

              >

                {opening ? "Opening..." : "Open Windows Security"}

              </motion.button>

              <motion.button

                type="button"

                className="hero-cta"

                whileHover={liftOnHover}

                whileTap={pressDown}

                onClick={onClose}

              >

                Done

              </motion.button>

            </div>

          </motion.div>

        </motion.div>

      )}

    </AnimatePresence>

  );

}

export function shouldShowDefenderModal(): boolean {

  if (typeof navigator === "undefined" || !navigator.userAgent.includes("Windows")) {

    return false;

  }

  return window.localStorage.getItem("fixed-defender-seen") !== "1";

}
