import { useState } from "react";

import { motion } from "framer-motion";

import { X } from "lucide-react";

import { ArtFallback } from "./ArtFallback";

import { DownloadEntryActions } from "./DownloadEntryActions";

import { DownloadEntryProgress } from "./DownloadEntryProgress";

import { downloadPercent, downloadStatusLabel } from "../lib/downloads";

import { queueVariants } from "../lib/motion";

import type { DownloadEntry } from "../types";


export function DownloadEntryCard({

  entry,

  index,

  onCancel,

}: {

  entry: DownloadEntry;

  index: number;

  onCancel: (entry: DownloadEntry) => void;

}) {

  const [posterFailed, setPosterFailed] = useState(false);

  const percent = downloadPercent(entry);

  const status = downloadStatusLabel(entry);

  return (

    <motion.div

      className="queue-reveal"

      layout

      variants={queueVariants}

      initial="hidden"

      animate="visible"

      exit="exit"

      custom={index}

    >

      <article

      className="queue-card"

    >

      <div className="poster small">

        {posterFailed ? (

          <ArtFallback title={entry.game.title} />

        ) : (

          <img

            src={entry.game.posterUrl}

            alt={`${entry.game.title} poster`}

            referrerPolicy="no-referrer"

            onError={() => setPosterFailed(true)}

          />

        )}

      </div>

      <div className="queue-copy">

        <h2>

          {entry.game.title}

          {entry.state === "torrenting" && (

            <span className="lane-badge">{entry.lane === "http" ? "HTTP · mirror" : "P2P · torrent"}</span>

          )}

        </h2>

        <DownloadEntryProgress entry={entry} percent={percent} status={status} />

        <DownloadEntryActions gameTitle={entry.game.title} ready={entry.state === "ready"} />

      </div>

      <motion.button

        type="button"

        className="press-lift queue-cancel"

        aria-label={`Cancel ${entry.game.title}`}


        onClick={() => onCancel(entry)}

      >

        <X size={14} strokeWidth={2} />

      </motion.button>

      </article>

    </motion.div>

  );

}
