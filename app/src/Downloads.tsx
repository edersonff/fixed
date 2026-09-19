import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { open } from "@tauri-apps/plugin-dialog";

import { AnimatePresence, motion, type Variants } from "framer-motion";

import { PackagePlus } from "lucide-react";

import { Play } from "lucide-react";

import { Puzzle } from "lucide-react";

import { X } from "lucide-react";

import type { DownloadEntry } from "./types";

const stateVariants: Variants = {

  hidden: { opacity: 0, filter: "blur(6px)", y: 6 },

  visible: { opacity: 1, filter: "blur(0px)", y: 0, transition: { duration: 0.18 } },

  exit: { opacity: 0, filter: "blur(6px)", y: -4, transition: { duration: 0.12 } },

};

const queueVariants: Variants = {

  hidden: { opacity: 0, filter: "blur(8px)", y: 10 },

  visible: (index = 0) => ({

    opacity: 1,

    filter: "blur(0px)",

    y: 0,

    transition: { duration: 0.3, delay: index * 0.04, ease: [0.05, 0.7, 0.1, 1] },

  }),

  exit: { opacity: 0, filter: "blur(8px)", y: 10, transition: { duration: 0.15 } },

};

function LoadingDots({ label = "Loading" }: { label?: string }) {

  return (

    <span className="loading-status" aria-label={label}>

      <span className="sr-only">{label}</span>

      <span className="loading-dots" aria-hidden="true">

        {[0, 1, 2].map((index) => (

          <motion.span

            key={index}

            animate={{ opacity: [0.25, 1, 0.25], y: [0, -3, 0] }}

            transition={{ duration: 0.4, delay: index * 0.12, repeat: Infinity, ease: "easeInOut" }}

          />

        ))}

      </span>

    </span>

  );

}

function megabytes(bytes: number): string {

  return `${(bytes / 1_000_000).toFixed(1)} MB`;

}

function EntryCard({ entry, index, onCancel }: { entry: DownloadEntry; index: number; onCancel: (entry: DownloadEntry) => void }) {

  const [launching, setLaunching] = useState(false);

  const [pluginMsg, setPluginMsg] = useState("");

  async function addPlugin() {

    const selected = await open({

      multiple: false,

      filters: [{ name: "Plugin Package", extensions: ["zip", "rar", "dll"] }],

    });

    if (!selected || typeof selected !== "string") {

      return;

    }

    invoke<number>("install_plugin", { title: entry.game.title, archivePath: selected })

      .then((count) => setPluginMsg(`Plugin Installed. ${count} File${count === 1 ? "" : "s"}`))

      .catch((reason: unknown) => setPluginMsg(`Plugin Failed: ${String(reason)}`));

  }

  const total = entry.totalBytes ?? 0;

  const downloaded = entry.downloadedBytes ?? 0;

  const percent = total > 0 ? Math.floor((downloaded / total) * 100) : 0;

  const status =

    entry.state === "resolving"

      ? "Resolving Mirrors"

      : entry.state === "torrenting"

        ? `${percent}% · ${megabytes(downloaded)} of ${megabytes(total)}`

        : entry.state === "extracting"

          ? "Extracting"

          : entry.state === "ready"

            ? "Ready to Play"

            : entry.state === "stopped"

              ? "Stopped"

              : entry.state === "parts"

                ? `${entry.parts.length} File${entry.parts.length === 1 ? "" : "s"} Found. Manual Lane`

                : (entry.errorMsg ?? "Could Not Resolve This Lane");

  return (

    <motion.article

      className="queue-card"

      layout

      variants={queueVariants}

      initial="hidden"

      animate="visible"

      exit="exit"

      custom={index}

    >

      <motion.button

        type="button"

        className="queue-cancel ghost"

        aria-label={`Cancel ${entry.game.title}`}

        whileHover={{ y: -2 }}

        whileTap={{ scale: 0.97 }}

        onClick={() => onCancel(entry)}

      >

        <X size={14} strokeWidth={2} />

      </motion.button>

      <div className="poster small">

        <img src={entry.game.posterUrl} alt={`${entry.game.title} poster`} referrerPolicy="no-referrer" />

      </div>

      <div className="queue-copy">

        <h2>

          {entry.game.title}

          {entry.state === "torrenting" && (

            <span className="lane-badge">{entry.lane === "http" ? "HTTP · mirror" : "P2P · torrent"}</span>

          )}

        </h2>

        <AnimatePresence mode="wait" initial={false}>

          <motion.p

            key={entry.state}

            className={entry.state === "extracting" ? "status-shimmer" : undefined}

            variants={stateVariants}

            initial="hidden"

            animate="visible"

            exit="exit"

          >

            {entry.state === "resolving" ? <LoadingDots label="Resolving mirrors" /> : status}

          </motion.p>

        </AnimatePresence>

        {(entry.state === "torrenting" || entry.state === "extracting") && (

          <div className="progress-track">

            <motion.div

              className="progress-fill"

              animate={{ width: `${percent}%` }}

              transition={{ type: "spring", stiffness: 115, damping: 22, mass: 0.45 }}

            />

            {entry.state === "torrenting" && (

              <motion.div

                className="progress-shimmer"

                animate={{ x: ["-100%", "100%"] }}

                transition={{ duration: 1.4, repeat: Infinity, ease: "linear" }}

              />

            )}

          </div>

        )}

        {entry.state === "parts" && entry.parts.length > 0 && (

          <ul className="parts">

            {entry.parts.slice(0, 4).map((part) => (

              <li key={part}>{part.split("/").pop()}</li>

            ))}

          </ul>

        )}

        <AnimatePresence initial={false}>

        {entry.state === "ready" && (

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

              transition={{ type: "spring", stiffness: 280, damping: 18 }}

              whileHover={{ y: -2 }}

              whileTap={{ scale: 0.97 }}

              disabled={launching}

              onClick={() => {

                setLaunching(true);

                invoke<string>("launch_game", { title: entry.game.title })

                  .catch((reason: unknown) => console.error("launch failed:", reason))

                  .finally(() => {

                    setTimeout(() => setLaunching(false), 400);

                  });

              }}

            >

              <Play size={14} strokeWidth={2.2} />

              {launching ? "Launching…" : "Play"}

            </motion.button>

            <motion.button

              type="button"

              className="ghost"

              whileHover={{ y: -2 }}

              whileTap={{ scale: 0.97 }}

              onClick={addPlugin}

            >

              <PackagePlus size={14} strokeWidth={2} />

              Add Plugin

            </motion.button>

          </motion.div>

        )}

        </AnimatePresence>

        {pluginMsg && <p className="plugin-note">{pluginMsg}</p>}

      </div>

    </motion.article>

  );

}

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

        <motion.button type="button" whileHover={{ y: -2 }} whileTap={{ scale: 0.97 }} onClick={onBrowse}>

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

            whileHover={{ y: -2 }}

            whileTap={{ scale: 0.97 }}

            onClick={onStopAll}

          >

            Stop All

          </motion.button>

        )}

      </header>

      <div className="queue">

        <AnimatePresence mode="popLayout">

        {entries.map((entry, index) => (

          <EntryCard entry={entry} index={index} onCancel={onCancel} key={entry.game.pageUrl} />

        ))}

        </AnimatePresence>

      </div>

    </div>

  );

}
