import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { getVersion } from "@tauri-apps/api/app";

import { AnimatePresence, MotionConfig, motion } from "framer-motion";

import { DetailView } from "./Detail";

import { DownloadsView } from "./Downloads";

import { LibraryView } from "./Library";

import { DefenderModal, shouldShowDefenderModal } from "./components/DefenderModal";

import { Sidebar } from "./components/Sidebar";

import { HomeView } from "./components/HomeView";

import { useCatalog } from "./hooks/useCatalog";

import { useDownloads } from "./hooks/useDownloads";

import { useInstalledGames } from "./hooks/useInstalledGames";

import { useMouseBackNavigation } from "./hooks/useMouseBackNavigation";

import { ensureInstalledTitlesLoaded } from "./lib/installedGames";

import { viewVariants } from "./lib/motion";

import type { DownloadLane } from "./types";

import type { GameDetail } from "./types";

import type { GameEntry } from "./types";

import type { View } from "./types";

import "./App.css";

export default function App() {

  const [view, setView] = useState<View>("home");

  const [selected, setSelected] = useState<GameEntry | null>(null);

  const [detail, setDetail] = useState<GameDetail | null>(null);

  const [detailBusy, setDetailBusy] = useState(false);

  const [appVersion, setAppVersion] = useState("");

  const [showDefender, setShowDefender] = useState(false);

  const catalog = useCatalog();

  function openDetail(game: GameEntry) {

    setSelected(game);

    setDetail(null);

    setDetailBusy(true);

    catalog.fetchDetail(game.pageUrl)

      .then((result) => setDetail(result))

      .finally(() => setDetailBusy(false));

  }

  const [torrentAsk, setTorrentAsk] = useState<{ game: GameEntry; detail: GameDetail } | null>(null);

  const downloads = useDownloads({

    setView,

    setSelected,

    openDetail,

    onError: catalog.setError,

    onTorrentOnly: (game, gameDetail) => setTorrentAsk({ game, detail: gameDetail }),

  });

  const library = useInstalledGames(view === "library");

  useEffect(() => {

    if (shouldShowDefenderModal()) {

      setShowDefender(true);

    }

  }, []);

  useEffect(() => {

    getVersion()

      .then((version) => setAppVersion(version))

      .catch(() => setAppVersion(""));

  }, []);

  useEffect(() => {

    ensureInstalledTitlesLoaded();

  }, []);

  useMouseBackNavigation(setSelected);

  function pickLane(lane: DownloadLane) {

    if (!selected) {

      return;

    }

    if (lane.kind === "hosters") {

      downloads.startHttp(selected, lane.url);

      return;

    }

    if (lane.kind === "torrent") {

      downloads.startTorrent(selected, detail ?? { title: selected.title, build: "", steamExtUrl: "", lanes: [lane], mentionsFixRepair: false });

      return;

    }

    invoke("open_download_window", { url: lane.url }).catch((reason: unknown) => console.error(reason));

  }

  function switchView(target: View) {

    setSelected(null);

    catalog.setQuery("");

    catalog.setSearchTerm("");

    setView(target);

  }

  return (

    <MotionConfig reducedMotion="user">

      <main className="shell">

      <Sidebar view={view} hasSelection={!!selected} appVersion={appVersion} onSwitchView={switchView} />

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

            onDownload={() => downloads.requestDownload(selected, detail)}

            onLanePick={pickLane}

          />

        ) : view === "home" ? (

          <HomeView

            query={catalog.query}

            setQuery={catalog.setQuery}

            runSearch={catalog.runSearch}

            source={catalog.source}

            error={catalog.error}

            searchTerm={catalog.searchTerm}

            busy={catalog.busy}

            canLoadMore={catalog.canLoadMore}

            page={catalog.page}

            loadPage={catalog.loadPage}

            featured={catalog.featured}

            trendingRest={catalog.trendingRest}

            recent={catalog.recent}

            visible={catalog.visible}

            openDetail={openDetail}

            requestDownload={downloads.requestDownload}

            quickBusy={downloads.quickBusy}

          />

        ) : view === "library" ? (

          <LibraryView

            games={library.games}

            scanning={library.scanning}

            error={library.error}

            refresh={library.refresh}

            onBrowse={() => switchView("home")}

          />

        ) : (

          <DownloadsView

            entries={downloads.downloads}

            onBrowse={() => switchView("home")}

            onStopAll={downloads.stopAll}

            onCancel={downloads.cancelDownload}

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

      {torrentAsk && (

        <div className="modal-backdrop" onClick={() => setTorrentAsk(null)}>

          <div className="modal-card" role="dialog" aria-modal="true" onClick={(event) => event.stopPropagation()}>

            <h2>Torrent is the only lane</h2>

            <p className="modal-hint">

              {torrentAsk.game.title} has no HTTP mirror right now. The torrent lane works, but some

              ISPs monitor torrent swarms and may flag your connection.

            </p>

            <div className="modal-actions">

              <button type="button" className="ghost" onClick={() => setTorrentAsk(null)}>

                Cancel

              </button>

              <button

                type="button"

                className="hero-cta"

                onClick={() => {

                  downloads.startTorrent(torrentAsk.game, torrentAsk.detail);

                  setTorrentAsk(null);

                }}

              >

                Download via torrent

              </button>

            </div>

          </div>

        </div>

      )}

      </main>

    </MotionConfig>

  );

}
