import { AnimatePresence, motion } from "framer-motion";

import { FolderOpen, Play, Puzzle, Wrench } from "lucide-react";

import { actionHover, actionPressDown, stateVariants } from "../lib/motion";

export function LibraryCardActions({

  label,

  busy,

  needsFix,

  onPress,

  onAddPlugin,

  onOpenFolder,

}: {

  label: string;

  busy: boolean;

  needsFix: boolean;

  onPress: (event: React.MouseEvent) => void;

  onAddPlugin: (event: React.MouseEvent) => void;

  onOpenFolder: (event: React.MouseEvent) => void;

}) {

  return (

    <div className="ready-actions">

      <motion.button

        type="button"

        className={needsFix ? "play action-button fix" : "play action-button"}

        whileHover={actionHover}

        whileTap={actionPressDown}

        disabled={busy}

        onClick={onPress}

      >

        <span className="action-icon">

          {needsFix ? <Wrench size={16} strokeWidth={2.4} /> : <Play size={16} strokeWidth={2.4} />}

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
