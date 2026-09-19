import { memo } from "react";

import { motion } from "framer-motion";

import { Download, Play } from "lucide-react";

import { ArtImage } from "./ArtImage";

import { useGameAssets } from "../hooks/useGameAssets";

import { useGameLaunch } from "../hooks/useGameLaunch";

import { useIsGameInstalled } from "../hooks/useIsGameInstalled";

import type { GameEntry } from "../types";

import { displayTitle } from "../lib/format";

import { formatDate } from "../lib/format";

import { prettyCategory } from "../lib/format";

import { fadeRiseVariants } from "../lib/motion";

import { liftOnHover, pressDown } from "../lib/motion";

function GameCardBase({

  game,

  index,

  onSelect,

  onQuickDownload,

  quickBusy,

  onPreview,

}: {

  game: GameEntry;

  index: number;

  onSelect: (game: GameEntry) => void;

  onQuickDownload: (game: GameEntry) => void;

  quickBusy: boolean;

  onPreview?: (game: GameEntry | null) => void;

}) {

  const rawAssets = useGameAssets(game.title);

  const loadingArt = rawAssets === undefined;

  const assets = rawAssets ?? null;

  const installed = useIsGameInstalled(game.title);

  const { launching, launch } = useGameLaunch(game.title);

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

      onMouseEnter={() => onPreview?.(game)}

      onMouseLeave={() => onPreview?.(null)}

      onFocus={() => onPreview?.(game)}

      onBlur={() => onPreview?.(null)}

      onKeyDown={(event: React.KeyboardEvent) => {

        if (event.key === "Enter" || event.key === " ") {

          event.preventDefault();

          onSelect(game);

        }

      }}

    >

      <div className="poster">

        <ArtImage

          title={displayTitle(game.title)}

          sources={[assets?.coverUrl, game.posterUrl]}

          loading={loadingArt}

          className="poster-art"

          alt={`${game.title} cover`}

        />

        <div className="poster-overlay">

          <motion.button

            type="button"

            className="quick-dl"

            whileHover={liftOnHover}

            whileTap={pressDown}

            disabled={installed ? launching : quickBusy}

            onClick={(event: React.MouseEvent) => {

              event.stopPropagation();

              if (installed) {

                launch();

                return;

              }

              onQuickDownload(game);

            }}

          >

            {installed ? <Play size={14} strokeWidth={2.4} /> : <Download size={14} strokeWidth={2.4} />}

            {installed ? (launching ? "Starting" : "Play") : "Get"}

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

export const GameCard = memo(GameCardBase);
