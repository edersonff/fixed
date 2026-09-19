import type { CSSProperties } from "react";

import { motion } from "framer-motion";

import { Download } from "lucide-react";

import type { GameEntry } from "../types";

import { displayTitle } from "../lib/format";

import { formatDate } from "../lib/format";

import { prettyCategory } from "../lib/format";

import { fadeRiseVariants } from "../lib/motion";

export function GameCard({

  game,

  index,

  onSelect,

  onQuickDownload,

  quickBusy,

}: {

  game: GameEntry;

  index: number;

  onSelect: (game: GameEntry) => void;

  onQuickDownload: (game: GameEntry) => void;

  quickBusy: boolean;

}) {

  const cropPositions = ["center 30%", "center 22%", "center 36%", "left 32%", "right 28%"];

  const posterStyle = {

    "--poster-position": cropPositions[game.title.length % cropPositions.length],

    "--poster-scale": "1.03",

    "--poster-hover-scale": "1.09",

  } as CSSProperties;

  return (

    <motion.article

      className="card"

      variants={fadeRiseVariants}

      initial="hidden"

      animate="visible"

      exit="exit"

      custom={index}

      role="button"

      tabIndex={0}

      aria-label={`Open ${displayTitle(game.title)}`}

      onClick={() => onSelect(game)}

      onKeyDown={(event: React.KeyboardEvent) => {

        if (event.key === "Enter" || event.key === " ") {

          event.preventDefault();

          onSelect(game);

        }

      }}

    >

      <div className="poster" style={posterStyle}>

        <img src={game.posterUrl} alt={`${game.title} poster`} loading="lazy" referrerPolicy="no-referrer" />

        <div className="poster-overlay">

          <motion.button

            type="button"

            className="quick-dl"

            whileHover={{ y: -2 }}

            whileTap={{ scale: 0.97 }}

            disabled={quickBusy}

            onClick={(event: React.MouseEvent) => {

              event.stopPropagation();

              onQuickDownload(game);

            }}

          >

            <Download size={14} strokeWidth={2.4} />

            Get

          </motion.button>

        </div>

      </div>

      <div className="copy">

        <h2>{displayTitle(game.title)}</h2>

        <p className="meta">

          <span>{prettyCategory(game.category)}</span>

          <span>{formatDate(game.publishedAt)}</span>

        </p>

      </div>

    </motion.article>

  );

}
