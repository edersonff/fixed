import { useEffect, useMemo, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { getVersion } from "@tauri-apps/api/app";

import { listen } from "@tauri-apps/api/event";

import { AnimatePresence, MotionConfig, motion } from "framer-motion";

import { Download } from "lucide-react";

import { Home } from "lucide-react";

import { LibraryBig } from "lucide-react";

import { Search } from "lucide-react";

import { DetailView } from "./Detail";

import { DefenderModal, shouldShowDefenderModal } from "./components/DefenderModal";

import { GameCard } from "./components/GameCard";

import { Rail } from "./components/Rail";

import { EmptyState } from "./components/EmptyState";

import { displayTitle } from "./lib/format";

import { formatDate } from "./lib/format";

import { prettyCategory } from "./lib/format";

import { heroVariants } from "./lib/motion";

import { viewVariants } from "./lib/motion";

import { fadeRiseVariants } from "./lib/motion";

import { DownloadsView } from "./Downloads";

import type { DownloadEntry } from "./types";

import type { DownloadState } from "./types";

import type { ProgressPayload } from "./types";

import type { GameDetail } from "./types";

import type { GameEntry } from "./types";

import type { GameAssets } from "./types";

import type { DownloadLane } from "./types";

import type { GamesPage } from "./types";

import type { View } from "./types";

import "./App.css";

const NAV: Array<{ id: View; label: string; Icon: typeof Home }> = [

  { id: "home", label: "Home", Icon: Home },

  { id: "library", label: "Library", Icon: LibraryBig },

  { id: "downloads", label: "Downloads", Icon: Download },

];

function Wordmark() {

  return (

    <span className="wordmark">

      FI<span className="x">X</span>ED

    </span>

  );

}

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

export default function App() {

  const [view, setView] = useState<View>("home");

  const [games, setGames] = useState<GameEntry[]>([]);

  const [source, setSource] = useState("");

  const [page, setPage] = useState(1);

  const [query, setQuery] = useState("");

  const [searchTerm, setSearchTerm] = useState("");

  const [busy, setBusy] = useState(false);

  const [error, setError] = useState<string | null>(null);

  const [selected, setSelected] = useState<GameEntry | null>(null);

  const [detail, setDetail] = useState<GameDetail | null>(null);

  const [detailBusy, setDetailBusy] = useState(false);

  const [downloads, setDownloads] = useState<DownloadEntry[]>([]);

  const [quickBusy, setQuickBusy] = useState(false);

  useEffect(() => {

    loadPage(1);

  }, []);

  useEffect(() => {

    const unlisten = listen<ProgressPayload>("download-progress", (event) => {

      const { title, downloadedBytes, totalBytes, state } = event.payload;

      const mapped: DownloadState =

        state === "extracting"

          ? "extracting"

          : state === "ready"

            ? "ready"

            : state === "error"

              ? "error"

              : "torrenting";

      setDownloads((previous) =>

        previous.map((entry) => {

          if (entry.game.title.replace(/\//g, "_") !== title) {

            return entry;

          }

          if (mapped === "torrenting" && (entry.state === "extracting" || entry.state === "ready")) {

            return entry;

          }

          if (mapped === "torrenting") {

            return { ...entry, state: mapped, downloadedBytes, totalBytes };

          }

          return { ...entry, state: mapped };

        }),

      );

    });

    return () => {

      unlisten.then((stop) => stop());

    };

  }, []);

  const visible = useMemo(

    () =>

      searchTerm

        ? games.filter((game) => {

            const title = game.title.toLowerCase().replace(/[^a-z0-9]/g, "");

            const term = searchTerm.toLowerCase().replace(/[^a-z0-9]/g, "");

            return title.includes(term) || term.includes(title);

          })

        : games,

    [games, searchTerm],

  );

  useEffect(() => {

    if (!searchTerm || busy || source !== "live") {

      return;

    }

    if (visible.length > 0 || page >= 8) {

      return;

    }

    loadPage(page + 1);

  }, [searchTerm, visible.length, busy, page, source]);

  const trending = useMemo(

    () => [...visible].sort((a, b) => b.views - a.views).slice(0, 7),

    [visible],

  );

  const recent = useMemo(

    () => [...visible].sort((a, b) => b.publishedAt.localeCompare(a.publishedAt)).slice(0, 6),

    [visible],

  );

  const featured = trending[0] ?? null;

  const trendingRest = featured ? trending.slice(1) : trending;

  const [appVersion, setAppVersion] = useState("");

  const [showDefender, setShowDefender] = useState(false);

  useEffect(() => {

    if (shouldShowDefenderModal()) {

      setShowDefender(true);

    }

  }, []);

  const [featuredAssets, setFeaturedAssets] = useState<GameAssets | null>(null);

  const [featuredLogoFailed, setFeaturedLogoFailed] = useState(false);

  useEffect(() => {

    getVersion()

      .then((version) => setAppVersion(version))

      .catch(() => setAppVersion(""));

  }, []);

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

  function mergeGames(previous: GameEntry[], incoming: GameEntry[]) {

    const seen = new Set(previous.map((game) => game.pageUrl));

    return [...previous, ...incoming.filter((game) => !seen.has(game.pageUrl))];

  }

  function loadPage(target: number) {

    setBusy(true);

    invoke<GamesPage>("list_games", { page: target })

      .then((result) => {

        setSource(result.source);

        setGames((previous) => (target === 1 ? result.games : mergeGames(previous, result.games)));

        setPage(target);

      })

      .catch((reason: unknown) => setError(String(reason)))

      .finally(() => setBusy(false));

  }

  function runSearch() {

    setSearchTerm(query.trim().toLowerCase());

  }

  function openDetail(game: GameEntry) {

    setSelected(game);

    setDetail(null);

    setDetailBusy(true);

    invoke<GameDetail>("game_detail", { url: game.pageUrl })

      .then((result) => setDetail(result))

      .catch(() => setDetail(null))

      .finally(() => setDetailBusy(false));

  }

  function startTorrent(game: GameEntry, gameDetail: GameDetail) {

    const torrentLane = gameDetail.lanes.find(

      (lane) => lane.kind === "torrent" && lane.url.startsWith("https://uploads.online-fix.me"),

    );

    if (!torrentLane) {

      openDetail(game);

      return;

    }

    const safeTitle = game.title.replace(/\//g, "_");

    setDownloads((previous) => [

      { game, state: "torrenting", lane: "torrent", parts: [], downloadedBytes: 0, totalBytes: 0 },

      ...previous.filter((entry) => entry.game.pageUrl !== game.pageUrl),

    ]);

    setSelected(null);

    setView("downloads");

    invoke<string>("start_torrent_download", { title: safeTitle, laneUrl: torrentLane.url })

      .catch((reason: unknown) => {

        console.error("start_torrent_download failed:", reason);

        setDownloads((previous) =>

          previous.map((entry) =>

            entry.game.pageUrl === game.pageUrl

              ? { ...entry, state: "error", errorMsg: String(reason).slice(0, 140) }

              : entry,

          ),

        );

      });

  }

  function startHttp(game: GameEntry, hostersUrl: string, gameDetail: GameDetail | null) {

    const safeTitle = game.title.replace(/\//g, "_");

    setDownloads((previous) => [

      { game, state: "torrenting", lane: "http", parts: [], downloadedBytes: 0, totalBytes: 0 },

      ...previous.filter((entry) => entry.game.pageUrl !== game.pageUrl),

    ]);

    setSelected(null);

    setView("downloads");

    invoke<string>("start_http_download", { title: safeTitle, laneUrl: hostersUrl })

      .catch((reason: unknown) => {

        console.error("start_http_download failed:", reason);

        if (gameDetail) {

          startTorrent(game, gameDetail);

          return;

        }

        setDownloads((previous) =>

          previous.map((entry) =>

            entry.game.pageUrl === game.pageUrl

              ? { ...entry, state: "error", errorMsg: String(reason).slice(0, 140) }

              : entry,

          ),

        );

      });

  }

  function downloadWithDetail(game: GameEntry, gameDetail: GameDetail) {

    const hostersLane = gameDetail.lanes.find((lane) => lane.kind === "hosters");

    if (hostersLane) {

      startHttp(game, hostersLane.url, gameDetail);

      return;

    }

    startTorrent(game, gameDetail);

  }

  function requestDownload(game: GameEntry, gameDetail: GameDetail | null) {

    if (gameDetail) {

      downloadWithDetail(game, gameDetail);

      return;

    }

    setQuickBusy(true);

    invoke<GameDetail>("game_detail", { url: game.pageUrl })

      .then((result) => downloadWithDetail(game, result))

      .catch((reason: unknown) => console.error("quick download failed:", reason))

      .finally(() => setQuickBusy(false));

  }

  function pickLane(lane: DownloadLane) {

    if (!selected) {

      return;

    }

    if (lane.kind === "hosters") {

      startHttp(selected, lane.url, detail);

      return;

    }

    if (lane.kind === "torrent") {

      startTorrent(selected, detail ?? { title: selected.title, build: "", steamExtUrl: "", lanes: [lane], mentionsFixRepair: false });

      return;

    }

    invoke("open_download_window", { url: lane.url }).catch((reason: unknown) => console.error(reason));

  }

  function stopAll() {

    invoke("cancel_all_downloads").catch((reason: unknown) => setError(String(reason)));

    setDownloads((previous) =>

      previous.map((entry) => (entry.state === "torrenting" ? { ...entry, state: "stopped" } : entry)),

    );

  }

  function cancelDownload(entry: DownloadEntry) {

    invoke("cancel_download", { title: entry.game.title }).catch(() => undefined);

    setDownloads((previous) =>

      previous.map((current) =>

        current.game.pageUrl === entry.game.pageUrl ? { ...current, state: "stopped" } : current,

      ),

    );

  }

  function switchView(target: View) {

    setSelected(null);

    setQuery("");

    setSearchTerm("");

    setView(target);

  }

  function scrollToCatalog() {

    document.getElementById("catalog")?.scrollIntoView({ behavior: "smooth" });

  }

  const canLoadMore = source === "live" && page >= 1 && !searchTerm;

  return (

    <MotionConfig reducedMotion="user">

      <main className="shell">

      <aside className="sidebar">

        <Wordmark />

        <nav>

          {NAV.map(({ id, label, Icon }) => (

            <motion.button

              key={id}

              type="button"

              title={label}

              className={view === id && !selected ? "nav-item active" : "nav-item"}

              whileHover={{ y: -2 }}

              whileTap={{ scale: 0.97 }}

              onClick={() => switchView(id)}

            >

              <Icon size={19} strokeWidth={1.8} />

              <span>{label}</span>

            </motion.button>

          ))}

        </nav>

        <span className="foot">v{appVersion || "…"} · every game, ready to play</span>

      </aside>

        <section className="content">

          <AnimatePresence mode="wait">

            <motion.div

              key={selected ? `detail-${selected.pageUrl}` : view}

              className="view-frame"

              variants={viewVariants}

              initial="hidden"

              animate="visible"

              exit="exit"

              custom={selected ? "detail" : "view"}

            >

        {selected ? (

          <DetailView

            game={selected}

            detail={detail}

            busy={detailBusy}

            onBack={() => setSelected(null)}

            onDownload={() => requestDownload(selected, detail)}

            onLanePick={pickLane}

          />

        ) : view === "home" ? (

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

        ) : view === "library" ? (

          <EmptyState

            title="Nothing Here Yet"

            hint="Games You Install Land on This Shelf, Ready to Play With Zero Setup."

            action="Browse Games"

            onAction={() => switchView("home")}

          />

        ) : (

          <DownloadsView

            entries={downloads}

            onBrowse={() => switchView("home")}

            onStopAll={stopAll}

            onCancel={cancelDownload}

          />

        )}

            </motion.div>

          </AnimatePresence>

        </section>

      <DefenderModal

        open={showDefender}

        onClose={() => {

          window.localStorage.setItem("fixed-defender-seen", "1");

          setShowDefender(false);

        }}

      />

      </main>

    </MotionConfig>

  );

}
