import { AnimatePresence, motion } from "framer-motion";

import { Search } from "lucide-react";

import { GameCard } from "./GameCard";

import { LoadingDots } from "./LoadingDots";

import { fadeRiseVariants } from "../lib/motion";

import type { GameDetail } from "../types";

import type { GameEntry } from "../types";

import { liftOnHover, pressDown } from "../lib/motion";

export function HomeCatalog({

  visible,

  searchTerm,

  busy,

  canLoadMore,

  page,

  loadPage,

  openDetail,

  requestDownload,

  quickBusy,

}: {

  visible: GameEntry[];

  searchTerm: string;

  busy: boolean;

  canLoadMore: boolean;

  page: number;

  loadPage: (target: number) => void;

  openDetail: (game: GameEntry) => void;

  requestDownload: (game: GameEntry, detail: GameDetail | null) => void;

  quickBusy: boolean;

}) {

  return (

    <section className="catalog" id="catalog">

      <header className="rail-head">

        <h2>All Games</h2>

      </header>

      <div className="grid">

        <AnimatePresence mode="popLayout">

          {visible.map((game, index) => (

            <GameCard

              game={game}

              index={index}

              onSelect={openDetail}

              onQuickDownload={(gameEntry) => requestDownload(gameEntry, null)}

              quickBusy={quickBusy}

              key={game.pageUrl}

            />

          ))}

        </AnimatePresence>

      </div>

      {searchTerm && visible.length === 0 && !busy && (

        <motion.div className="empty" initial="hidden" animate="visible" variants={fadeRiseVariants}>

          <motion.div

            className="empty-icon"

            animate={{ y: [0, -4, 0] }}

            transition={{ duration: 3, repeat: Infinity, ease: "easeInOut" }}

          >

            <Search size={26} strokeWidth={1.5} />

          </motion.div>

          <motion.h2 variants={fadeRiseVariants} custom={1}>

            No games found

          </motion.h2>

          <motion.p variants={fadeRiseVariants} custom={2}>

            Nothing matches "{searchTerm}" in the catalog loaded so far. Try another name.

          </motion.p>

        </motion.div>

      )}

      {canLoadMore && (

        <motion.button

          type="button"

          className="loadmore"

          disabled={busy}

          whileHover={liftOnHover}

          whileTap={pressDown}

          onClick={() => loadPage(page + 1)}

        >

          {busy ? <LoadingDots label="Loading more games" /> : "Load More"}

        </motion.button>

      )}

    </section>

  );

}
