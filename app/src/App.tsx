import { useEffect, useMemo, useState } from "react";

import type { CSSProperties } from "react";

import { invoke } from "@tauri-apps/api/core";

import { listen } from "@tauri-apps/api/event";

import { ChevronRight } from "lucide-react";

import { Download } from "lucide-react";

import { Home } from "lucide-react";

import { LibraryBig } from "lucide-react";

import { Puzzle } from "lucide-react";

import { Search } from "lucide-react";

import { DetailView } from "./Detail";

import { DownloadsView } from "./Downloads";

import type { DownloadEntry } from "./types";

import type { DownloadState } from "./types";

import type { ProgressPayload } from "./types";

import type { GameDetail } from "./types";

import type { GameEntry } from "./types";

import type { GamesPage } from "./types";

import type { View } from "./types";

import "./App.css";

const NAV: Array<{ id: View; label: string; Icon: typeof Home }> = [

  { id: "home", label: "Home", Icon: Home },

  { id: "library", label: "Library", Icon: LibraryBig },

  { id: "downloads", label: "Downloads", Icon: Download },

];

const PREF_KEY = "fixed-dl-pref";

const MONTHS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

function formatDate(iso: string): string {

  const date = new Date(iso);

  if (Number.isNaN(date.getTime())) {

    return iso.slice(0, 10);

  }

  return `${String(date.getDate()).padStart(2, "0")} ${MONTHS[date.getMonth()]} ${date.getFullYear()}`;

}

function prettyCategory(raw: string): string {

  return raw.replace("officialservers", "official servers");

}

function displayTitle(raw: string): string {

  return raw

    .trim()

    .split(/\s+/)

    .map((word) => {

      if (/^(?:[A-Za-z]\.){2,}[A-Za-z]?/.test(word)) {

        return word;

      }

      const normalized = word.toLocaleLowerCase();

      return normalized.replace(/^[a-zà-ÿ]/i, (letter) => letter.toLocaleUpperCase());

    })

    .join(" ");

}

function Wordmark() {

  return (

    <span className="wordmark">

      FI<span className="x">X</span>ED

    </span>

  );

}

function GameCard({

  game,

  onSelect,

  onQuickDownload,

  quickBusy,

}: {

  game: GameEntry;

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

    <article

      className="card"

      role="button"

      tabIndex={0}

      aria-label={`Open ${displayTitle(game.title)}`}

      onClick={() => onSelect(game)}

      onKeyDown={(event) => {

        if (event.key === "Enter" || event.key === " ") {

          event.preventDefault();

          onSelect(game);

        }

      }}

    >

      <div className="poster" style={posterStyle}>

        <img src={game.posterUrl} alt={`${game.title} poster`} loading="lazy" referrerPolicy="no-referrer" />

        <div className="poster-overlay">

          <button

            type="button"

            className="quick-dl"

            disabled={quickBusy}

            onClick={(event) => {

              event.stopPropagation();

              onQuickDownload(game);

            }}

          >

            <Download size={14} strokeWidth={2.4} />

            Get

          </button>

        </div>

      </div>

      <div className="copy">

        <h2>{displayTitle(game.title)}</h2>

        <p className="meta">

          <span>{prettyCategory(game.category)}</span>

          <span>{formatDate(game.publishedAt)}</span>

        </p>

      </div>

    </article>

  );

}

function ChoiceModal({

  game,

  onPick,

  onDismiss,

}: {

  game: GameEntry;

  onPick: (mode: "torrent" | "manual", remember: boolean) => void;

  onDismiss: () => void;

}) {

  const [remember, setRemember] = useState(false);

  return (

    <div className="choice-backdrop" onClick={onDismiss}>

      <div className="choice" onClick={(event) => event.stopPropagation()}>

        <h3>How do you want to download {displayTitle(game.title)}?</h3>

        <p className="hint">

          Torrent is the only lane FIXED can automate end to end. Some ISPs monitor P2P swarms, so the choice is yours.

        </p>

        <button type="button" className="choice-lane" onClick={() => onPick("torrent", remember)}>

          <Download size={17} strokeWidth={2} />

          <span className="name">Torrent · P2P</span>

          <span className="kind">automatic download, extract and Steam setup</span>

        </button>

        <button type="button" className="choice-lane" onClick={() => onPick("manual", remember)}>

          <Puzzle size={17} strokeWidth={2} />

          <span className="name">Pick manually</span>

          <span className="kind">open the game page and choose a mirror yourself</span>

        </button>

        <label className="remember">

          <input

            type="checkbox"

            checked={remember}

            onChange={(event) => setRemember(event.target.checked)}

          />

          Remember my choice

        </label>

        <button type="button" className="dismiss" onClick={onDismiss}>

          Not now

        </button>

      </div>

    </div>

  );

}

function EmptyState({ title, hint, action, onAction }: { title: string; hint: string; action: string; onAction: () => void }) {

  return (

    <div className="empty">

      <Puzzle size={28} strokeWidth={1.5} />

      <h2>{title}</h2>

      <p>{hint}</p>

      <button type="button" onClick={onAction}>

        {action}

      </button>

    </div>

  );

}

function Rail({

  title,

  games,

  onSelect,

  onQuickDownload,

  quickBusy,

  onSeeAll,

}: {

  title: string;

  games: GameEntry[];

  onSelect: (game: GameEntry) => void;

  onQuickDownload: (game: GameEntry) => void;

  quickBusy: boolean;

  onSeeAll: () => void;

}) {

  if (games.length === 0) {

    return null;

  }

  return (

    <section className="rail-block">

      <header className="rail-head">

        <h2>{title}</h2>

        <button type="button" className="seeall" onClick={onSeeAll}>

          See all <ChevronRight size={14} strokeWidth={2} />

        </button>

      </header>

      <div className="rail">

        {games.map((game) => (

          <GameCard

            game={game}

            onSelect={onSelect}

            onQuickDownload={onQuickDownload}

            quickBusy={quickBusy}

            key={game.pageUrl}

          />

        ))}

      </div>

    </section>

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

  const [choiceGame, setChoiceGame] = useState<GameEntry | null>(null);

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

        ? games.filter((game) => game.title.toLowerCase().includes(searchTerm))

        : games,

    [games, searchTerm],

  );

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

    setChoiceGame(null);

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

    setChoiceGame(null);

    setView("downloads");

    invoke<string>("start_torrent_download", { title: safeTitle, laneUrl: torrentLane.url })

      .catch((reason: unknown) => {

        console.error("start_torrent_download failed:", reason);

        setDownloads((previous) =>

          previous.map((entry) =>

            entry.game.pageUrl === game.pageUrl ? { ...entry, state: "error" } : entry,

          ),

        );

      });

  }

  function downloadPreference(): string | null {

    return window.localStorage.getItem(PREF_KEY);

  }

  function requestDownload(game: GameEntry, gameDetail: GameDetail | null) {

    const pref = downloadPreference();

    if (!pref) {

      if (gameDetail) {

        setChoiceGame(game);

        return;

      }

      openDetail(game);

      return;

    }

    if (pref === "manual") {

      if (!gameDetail) {

        openDetail(game);

        return;

      }

      openDetail(game);

      return;

    }

    if (gameDetail) {

      startTorrent(game, gameDetail);

      return;

    }

    setQuickBusy(true);

    invoke<GameDetail>("game_detail", { url: game.pageUrl })

      .then((result) => startTorrent(game, result))

      .catch((reason: unknown) => console.error("quick download failed:", reason))

      .finally(() => setQuickBusy(false));

  }

  function pickChoice(mode: "torrent" | "manual", remember: boolean) {

    if (!choiceGame) {

      return;

    }

    if (remember) {

      window.localStorage.setItem(PREF_KEY, mode);

    }

    if (mode === "manual") {

      openDetail(choiceGame);

      return;

    }

    setQuickBusy(true);

    const game = choiceGame;

    invoke<GameDetail>("game_detail", { url: game.pageUrl })

      .then((result) => startTorrent(game, result))

      .catch((reason: unknown) => console.error("choice download failed:", reason))

      .finally(() => setQuickBusy(false));

  }

  function stopAll() {

    invoke("cancel_all_downloads").catch((reason: unknown) => setError(String(reason)));

    setDownloads((previous) =>

      previous.map((entry) => (entry.state === "torrenting" ? { ...entry, state: "stopped" } : entry)),

    );

  }

  function switchView(target: View) {

    setSelected(null);

    setView(target);

  }

  function scrollToCatalog() {

    document.getElementById("catalog")?.scrollIntoView({ behavior: "smooth" });

  }

  const canLoadMore = source === "live" && page >= 1;

  return (

    <main className="shell">

      <aside className="sidebar">

        <Wordmark />

        <nav>

          {NAV.map(({ id, label, Icon }) => (

            <button

              key={id}

              type="button"

              className={view === id && !selected ? "nav-item active" : "nav-item"}

              onClick={() => switchView(id)}

            >

              <Icon size={19} strokeWidth={1.8} />

              <span>{label}</span>

            </button>

          ))}

        </nav>

        <span className="foot">v0.1</span>

      </aside>

      <section className="content">

        {selected ? (

          <DetailView

            game={selected}

            detail={detail}

            busy={detailBusy}

            onBack={() => setSelected(null)}

            onDownload={() => requestDownload(selected, detail)}

          />

        ) : view === "home" ? (

          <>

            <header className="bar">

              <div className="bar-title">

                <p className="eyebrow">FIXED / game library</p>

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

                    placeholder="Search games"

                    onChange={(event) => setQuery(event.target.value)}

                    onKeyDown={(event) => event.key === "Enter" && runSearch()}

                  />

                </label>

              </div>

            </header>

            {featured && (

              <section className="hero">

                <img className="hero-bg" src={featured.posterUrl} alt="" referrerPolicy="no-referrer" />

                <div className="hero-copy">

                  <p className="eyebrow">Featured</p>

                  <h1 className="hero-title">{displayTitle(featured.title)}</h1>

                  <div className="meta-line">

                    <span className="meta-chip">{prettyCategory(featured.category)}</span>

                    <span className="meta-chip">{formatDate(featured.publishedAt)}</span>

                  </div>

                </div>

                <button type="button" className="hero-cta" onClick={() => openDetail(featured)}>

                  <Download size={17} strokeWidth={2.2} />

                  Download

                </button>

              </section>

            )}

            {error && <p className="state">Failed to load: {error}</p>}

            <Rail

              title="Trending now"

              games={trendingRest}

              onSelect={openDetail}

              onQuickDownload={(game) => requestDownload(game, null)}

              quickBusy={quickBusy}

              onSeeAll={scrollToCatalog}

            />

            <Rail

              title="Recently added"

              games={recent}

              onSelect={openDetail}

              onQuickDownload={(game) => requestDownload(game, null)}

              quickBusy={quickBusy}

              onSeeAll={scrollToCatalog}

            />

            <section className="catalog" id="catalog">

              <header className="rail-head">

                <h2>All games</h2>

              </header>

              <div className="grid">

                {visible.map((game) => (

                  <GameCard

                    game={game}

                    onSelect={openDetail}

                    onQuickDownload={(gameEntry) => requestDownload(gameEntry, null)}

                    quickBusy={quickBusy}

                    key={game.pageUrl}

                  />

                ))}

              </div>

              {canLoadMore && (

                <button type="button" className="loadmore" disabled={busy} onClick={() => loadPage(page + 1)}>

                  {busy ? "Loading…" : "Load more"}

                </button>

              )}

            </section>

          </>

        ) : view === "library" ? (

          <EmptyState

            title="Nothing here yet"

            hint="Games you install land on this shelf, ready to play with zero setup."

            action="Browse games"

            onAction={() => switchView("home")}

          />

        ) : (

          <DownloadsView entries={downloads} onBrowse={() => switchView("home")} onStopAll={stopAll} />

        )}

      </section>

      {choiceGame && (

        <ChoiceModal

          game={choiceGame}

          onPick={pickChoice}

          onDismiss={() => setChoiceGame(null)}

        />

      )}

    </main>

  );

}
