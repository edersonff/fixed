import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { AnimatePresence, motion } from "framer-motion";

import { Download } from "lucide-react";

import { Search } from "lucide-react";

import { GameCard } from "./GameCard";

import { LoadingDots } from "./LoadingDots";

import { Rail } from "./Rail";

import { displayTitle } from "../lib/format";

import { formatDate } from "../lib/format";

import { prettyCategory } from "../lib/format";

import { fadeRiseVariants } from "../lib/motion";

import { heroVariants } from "../lib/motion";

import type { GameAssets } from "../types";

import type { GameDetail } from "../types";

import type { GameEntry } from "../types";

export function HomeView({

  query,

  setQuery,

  runSearch,

  source,

  error,

  searchTerm,

  busy,

  canLoadMore,

  page,

  loadPage,

  featured,

  trendingRest,

  recent,

  visible,

  openDetail,

  requestDownload,

  quickBusy,

}: {

  query: string;

  setQuery: (value: string) => void;

  runSearch: () => void;

  source: string;

  error: string | null;

  searchTerm: string;

  busy: boolean;

  canLoadMore: boolean;

  page: number;

  loadPage: (target: number) => void;

  featured: GameEntry | null;

  trendingRest: GameEntry[];

  recent: GameEntry[];

  visible: GameEntry[];

  openDetail: (game: GameEntry) => void;

  requestDownload: (game: GameEntry, detail: GameDetail | null) => void;

  quickBusy: boolean;

}) {

  const [featuredAssets, setFeaturedAssets] = useState<GameAssets | null>(null);

  const [featuredLogoFailed, setFeaturedLogoFailed] = useState(false);

  useEffect(() => {

    setFeaturedAssets(null);

    setFeaturedLogoFailed(false);

    if (!featured) {

      return;

    }

    invoke<GameAssets | null>("game_assets", { title: featured.title })

      .then((result) => setFeaturedAssets(result))

      .catch(() => setFeaturedAssets(null));

  }, [featured?.title]);

  function scrollToCatalog() {

    document.getElementById("catalog")?.scrollIntoView({ behavior: "smooth" });

  }

  return (

    <>

      <header className="bar">

        <div className="bar-title">

          <p className="eyebrow">FIXED / Game Library</p>

          <h1>

            Home

            {source && <span className="source">{source}</span>}

          </h1>

        </div>

        <div className="bar-actions">

          <label className="searchbar">

            <Search size={15} strokeWidth={1.8} />

            <input

              value={query}

              placeholder="Search Games"

              onChange={(event) => setQuery(event.target.value)}

              onKeyDown={(event) => event.key === "Enter" && runSearch()}

            />

          </label>

        </div>

      </header>

      {featured && (

        <motion.section className="hero" initial="hidden" animate="visible" variants={fadeRiseVariants}>

          <img

            className="hero-bg"

            src={featuredAssets?.heroUrl || featured.posterUrl}

            alt=""

            referrerPolicy="no-referrer"

            onError={(event) => {

              if (featuredAssets?.heroUrl) {

                event.currentTarget.src = featured.posterUrl;

              }

            }}

          />

          <div className="hero-copy">

            <motion.p className="eyebrow" variants={heroVariants} initial="hidden" animate="visible" custom={0}>

              Featured

            </motion.p>

            <motion.h1 className="hero-title" variants={heroVariants} initial="hidden" animate="visible" custom={1}>

              {featuredAssets?.logoUrl && !featuredLogoFailed ? (

                <img

                  className="hero-logo-img"

                  src={featuredAssets.logoUrl}

                  alt={displayTitle(featured.title)}

                  referrerPolicy="no-referrer"

                  onError={() => setFeaturedLogoFailed(true)}

                />

              ) : (

                displayTitle(featured.title)

              )}

            </motion.h1>

            <motion.p className="hero-meta-line" variants={heroVariants} initial="hidden" animate="visible" custom={2}>

              {prettyCategory(featured.category)} · {formatDate(featured.publishedAt)}

            </motion.p>

            <motion.button

              type="button"

              className="hero-cta"

              variants={heroVariants}

              initial="hidden"

              animate="visible"

              custom={3}

              whileHover={{ y: -2 }}

              whileTap={{ scale: 0.97 }}

              onClick={() => openDetail(featured)}

            >

              <Download size={17} strokeWidth={2.2} />

              Download

            </motion.button>

          </div>

        </motion.section>

      )}

      {error && <p className="state">Failed to Load: {error}</p>}

      <Rail

        title="Trending Now"

        games={trendingRest}

        onSelect={openDetail}

        onQuickDownload={(game) => requestDownload(game, null)}

        quickBusy={quickBusy}

        onSeeAll={scrollToCatalog}

      />

      <Rail

        title="Recently Added"

        games={recent}

        onSelect={openDetail}

        onQuickDownload={(game) => requestDownload(game, null)}

        quickBusy={quickBusy}

        onSeeAll={scrollToCatalog}

      />

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

            whileHover={{ y: -2 }}

            whileTap={{ scale: 0.97 }}

            onClick={() => loadPage(page + 1)}

          >

            {busy ? <LoadingDots label="Loading more games" /> : "Load More"}

          </motion.button>

        )}

      </section>

    </>

  );

}
