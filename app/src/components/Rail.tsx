import { memo } from "react";

import { AnimatePresence, motion } from "framer-motion";

import { ChevronRight } from "lucide-react";

import type { GameEntry } from "../types";

import { GameCard } from "./GameCard";


function RailBase({

  title,

  games,

  onSelect,

  onQuickDownload,

  quickBusy,

  onSeeAll,

  onPreview,

}: {

  title: string;

  games: GameEntry[];

  onSelect: (game: GameEntry) => void;

  onQuickDownload: (game: GameEntry) => void;

  quickBusy: boolean;

  onSeeAll: () => void;

  onPreview?: (game: GameEntry | null) => void;

}) {

  if (games.length === 0) {

    return null;

  }

  return (

    <section className="rail-block">

      <header className="rail-head">

        <h2>{title}</h2>

        <motion.button

          type="button"

          className="press-lift seeall"


          onClick={onSeeAll}

        >

          See all <ChevronRight size={14} strokeWidth={2} />

        </motion.button>

      </header>

      <div className="rail">

        <AnimatePresence mode="popLayout">

          {games.map((game, index) => (

          <GameCard

            game={game}

            index={index}

            onSelect={onSelect}

            onQuickDownload={onQuickDownload}

            quickBusy={quickBusy}

            onPreview={onPreview}

            key={game.pageUrl}

          />

          ))}

        </AnimatePresence>

      </div>

    </section>

  );

}

export const Rail = memo(RailBase);
