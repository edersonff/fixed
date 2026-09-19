import { AnimatePresence, motion } from "framer-motion";

import { LoadingDots } from "./LoadingDots";

import { stateVariants } from "../lib/motion";

import type { DownloadEntry } from "../types";

export function DownloadEntryProgress({

  entry,

  percent,

  status,

}: {

  entry: DownloadEntry;

  percent: number;

  status: string;

}) {

  return (

    <>

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

    </>

  );

}
