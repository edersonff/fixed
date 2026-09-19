import { AnimatePresence, motion } from "framer-motion";

import { Puzzle } from "lucide-react";

import { DownloadEntryCard } from "./components/DownloadEntryCard";

import { stateVariants } from "./lib/motion";

import { queueVariants } from "./lib/motion";

import type { DownloadEntry } from "./types";

import { liftOnHover, pressDown } from "./lib/motion";

export function DownloadsView({

  entries,

  onBrowse,

  onStopAll,

  onCancel,

}: {

  entries: DownloadEntry[];

  onBrowse: () => void;

  onStopAll: () => void;

  onCancel: (entry: DownloadEntry) => void;

}) {

  if (entries.length === 0) {

    return (

      <motion.div className="empty" initial="hidden" animate="visible" variants={queueVariants}>

        <motion.div

          className="empty-icon"

          animate={{ y: [0, -4, 0] }}

          transition={{ duration: 3, repeat: Infinity, ease: "easeInOut" }}

        >

          <Puzzle size={28} strokeWidth={1.5} />

        </motion.div>

        <motion.h2 variants={stateVariants}>

          No Active Downloads

        </motion.h2>

        <motion.p variants={stateVariants}>

          Pick a Game and FIXED Handles Mirrors, Extraction and Setup for You.

        </motion.p>

        <motion.button type="button" whileHover={liftOnHover} whileTap={pressDown} onClick={onBrowse}>

          Find a Game

        </motion.button>

      </motion.div>

    );

  }

  const hasActive = entries.some(

    (entry) => entry.state === "torrenting" || entry.state === "extracting",

  );

  return (

    <div>

      <header className="bar downloads-bar">

        <div className="bar-title">

          <p className="eyebrow">Active Queue</p>

          <h1>Downloads</h1>

        </div>

        {hasActive && (

          <motion.button

            type="button"

            className="ghost"

            whileHover={liftOnHover}

            whileTap={pressDown}

            onClick={onStopAll}

          >

            Stop All

          </motion.button>

        )}

      </header>

      <div className="queue">

        <AnimatePresence mode="popLayout">

        {entries.map((entry, index) => (

          <DownloadEntryCard entry={entry} index={index} onCancel={onCancel} key={entry.game.pageUrl} />

        ))}

        </AnimatePresence>

      </div>

    </div>

  );

}
