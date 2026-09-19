import { AnimatePresence, motion } from "framer-motion";

import { FolderOpen, Play, Puzzle } from "lucide-react";

import { actionHover, actionPressDown, stateVariants } from "../lib/motion";

function playLabel(launching: boolean, phase: string): string {

  if (phase === "running") {

    return "Running";

  }

  return launching ? "Starting" : "Play";

}

export function LibraryCardActions({

  launching,

  phase,

  onLaunch,

  onAddPlugin,

  onOpenFolder,

}: {

  launching: boolean;

  phase: string;

  onLaunch: (event: React.MouseEvent) => void;

  onAddPlugin: (event: React.MouseEvent) => void;

  onOpenFolder: (event: React.MouseEvent) => void;

}) {

  const label = playLabel(launching, phase);

  return (

    <div className="ready-actions">

      <motion.button

        type="button"

        className="play action-button"

        whileHover={actionHover}

        whileTap={actionPressDown}

        disabled={launching || phase === "running"}

        onClick={onLaunch}

      >

        <span className="action-icon">

          <Play size={16} strokeWidth={2.4} />

        </span>

        <AnimatePresence mode="wait" initial={false}>

          <motion.span key={label} variants={stateVariants} initial="hidden" animate="visible" exit="exit">

            {label}

          </motion.span>

        </AnimatePresence>

      </motion.button>

      <button type="button" className="ghost" onClick={onAddPlugin}>

        <Puzzle size={15} strokeWidth={2} />

        Add Plugin

      </button>

      <button type="button" className="ghost" onClick={onOpenFolder}>

        <FolderOpen size={15} strokeWidth={2} />

        Folder

      </button>

    </div>

  );

}
