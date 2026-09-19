import { AnimatePresence, motion } from "framer-motion";

import { PackagePlus } from "lucide-react";

import { Play } from "lucide-react";

import { useGameLaunch } from "../hooks/useGameLaunch";

import { usePluginInstall } from "../hooks/usePluginInstall";

import { EASE_POP, DUR_SHORT, liftOnHover, pressDown } from "../lib/motion";

export function DownloadEntryActions({ gameTitle, ready }: { gameTitle: string; ready: boolean }) {

  const { launching, launchMsg, launch } = useGameLaunch(gameTitle);

  const { pluginMsg, addPlugin } = usePluginInstall(gameTitle);

  return (

    <>

      <AnimatePresence initial={false}>

        {ready && (

          <motion.div

            className="ready-actions"

            key="ready-actions"

            initial={{ opacity: 0, y: 6 }}

            animate={{ opacity: 1, y: 0 }}

            exit={{ opacity: 0, y: 6 }}

          >

            <motion.button

              type="button"

              className="play"

              initial={{ opacity: 0, scale: 0.9 }}

              animate={{ opacity: 1, scale: 1 }}

              transition={{ duration: DUR_SHORT, ease: EASE_POP }}

              whileHover={liftOnHover}

              whileTap={pressDown}

              disabled={launching}

              onClick={launch}

            >

              <Play size={14} strokeWidth={2.2} />

              {launching ? "Starting…" : "Play"}

            </motion.button>

            <motion.button

              type="button"

              className="ghost"

              whileHover={liftOnHover}

              whileTap={pressDown}

              onClick={addPlugin}

            >

              <PackagePlus size={14} strokeWidth={2} />

              Add Plugin

            </motion.button>

          </motion.div>

        )}

      </AnimatePresence>

      {launchMsg && <p className="plugin-note">{launchMsg}</p>}

      {pluginMsg && <p className="plugin-note">{pluginMsg}</p>}

    </>

  );

}
