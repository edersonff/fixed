import { useMemo, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { AnimatePresence, MotionConfig, motion } from "framer-motion";

import { DetailView } from "./Detail";

import { DownloadsView } from "./Downloads";

import { LibraryView } from "./Library";

import { FilesAlert } from "./components/FilesAlert";

import { Sidebar } from "./components/Sidebar";

import { HomeView } from "./components/HomeView";

import { TorrentOnlyModal } from "./components/TorrentOnlyModal";

import { useAppLifecycle } from "./hooks/useAppLifecycle";

import { useAppNavigation } from "./hooks/useAppNavigation";

import { useCatalog } from "./hooks/useCatalog";

import { useDownloads } from "./hooks/useDownloads";

import { useInstalledGames } from "./hooks/useInstalledGames";

import { viewVariants } from "./lib/motion";

import type { DownloadLane } from "./types";

import type { GameDetail } from "./types";

import type { GameEntry } from "./types";

import "./App.css";

export default function App() {

  const catalog = useCatalog();

  const { view, selected, setSelected, detail, detailBusy, openDetail, openInstalledDetail, switchView, setView } =

    useAppNavigation(catalog);

  const { appVersion } = useAppLifecycle(setSelected);

  const [torrentAsk, setTorrentAsk] = useState<{ game: GameEntry; detail: GameDetail } | null>(null);

  const downloads = useDownloads({

    setView,

    setSelected,

    openDetail,

    onError: catalog.setError,

    onTorrentOnly: (game, gameDetail) => setTorrentAsk({ game, detail: gameDetail }),

  });

  const library = useInstalledGames(view === "library" || selected !== null);

  const installedBuild = useMemo(() => {

    if (!selected) {

      return null;

    }

    const searchKey = selected.title.toLowerCase().replace(/[^a-z0-9]/g, "");

    return library.games.find(

      (game) => game.title.toLowerCase().replace(/[^a-z0-9]/g, "") === searchKey,

    )?.build ?? null;

  }, [library.games, selected]);

  function pickLane(lane: DownloadLane) {

    if (!selected) {

      return;

    }

    if (lane.kind === "hosters") {

      downloads.startHttp(selected, lane.url, detail?.build || null);

      return;

    }

    if (lane.kind === "torrent") {

      downloads.startTorrent(selected, detail ?? { title: selected.title, build: "", steamExtUrl: "", lanes: [lane], mentionsFixRepair: false });

      return;

    }

    invoke("open_download_window", { url: lane.url }).catch((reason: unknown) => console.error(reason));

  }

  return (

    <MotionConfig reducedMotion="user">

      <main className="shell">

      <Sidebar view={view} hasSelection={!!selected} appVersion={appVersion} onSwitchView={switchView} />

        <section className="content">

          <FilesAlert />

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

            installedBuild={installedBuild}

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

            onOpenDetail={openInstalledDetail}

            onUninstalled={library.refresh}

          />

        ) : (

          <DownloadsView

            onBrowse={() => switchView("home")}

            onStopAll={downloads.stopAll}

            onCancel={downloads.cancelDownload}

          />

        )}

            </motion.div>

          </AnimatePresence>

        </section>

      <TorrentOnlyModal

        torrentAsk={torrentAsk}

        onCancel={() => setTorrentAsk(null)}

        onConfirm={() => {

          if (torrentAsk) {

            downloads.startTorrent(torrentAsk.game, torrentAsk.detail);

            setTorrentAsk(null);

          }

        }}

      />

      </main>

    </MotionConfig>

  );

}
