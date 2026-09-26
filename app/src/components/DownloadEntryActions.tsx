import { AnimatePresence, motion } from "framer-motion";

import { PackagePlus, Play, Wrench } from "lucide-react";

import { FixBlockedNote } from "./FixBlockedNote";

import { useGameLaunch } from "../hooks/useGameLaunch";

import { usePluginInstall } from "../hooks/usePluginInstall";

import { EASE_POP, DUR_SHORT, actionHover, actionPressDown } from '../lib/motion';

export function DownloadEntryActions({ gameTitle, ready }: { gameTitle: string; ready: boolean }) {

  const { label, press, busy, needsFix, removedAgain, launchMsg } = useGameLaunch(gameTitle);

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

              whileHover={actionHover}

              whileTap={actionPressDown}

              disabled={busy || removedAgain}

              onClick={press}

            >

              <span className="action-icon">

                {needsFix ? <Wrench size={15} strokeWidth={2.2} /> : <Play size={15} strokeWidth={2.2} />}

              </span>

              {label}

            </motion.button>

            <motion.button

              type="button"

              className="press-lift ghost"


              onClick={addPlugin}

            >

              <PackagePlus size={14} strokeWidth={2} />

              Add Plugin

            </motion.button>

          </motion.div>

        )}

      </AnimatePresence>

      {removedAgain && <FixBlockedNote />}

      {launchMsg && (

        <p className={launchMsg.startsWith("Launch Failed") || launchMsg.startsWith("Fix Failed") ? "launch-note launch-error" : "launch-note"}>

          {launchMsg}

        </p>

      )}

      {pluginMsg && <p className="plugin-note">{pluginMsg}</p>}

    </>

  );

}
