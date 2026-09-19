import { useCallback, useEffect } from "react";

import { Search } from "lucide-react";

import { HomeCatalog } from "./HomeCatalog";

import { HomeHero } from "./HomeHero";

import { Rail } from "./Rail";

import { prefetchGameAssets } from "../lib/gameAssets";

import { setHeroPreview } from "../lib/heroPreview";

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

  const quickDownload = useCallback((game: GameEntry) => requestDownload(game, null), [requestDownload]);

  useEffect(() => {

    prefetchGameAssets([...trendingRest, ...recent].map((game) => game.title));

  }, [trendingRest, recent]);

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

      <HomeHero featured={featured} onSelect={openDetail} />

      {error && <p className="state">Failed to Load: {error}</p>}

      <Rail

        title="Trending Now"

        games={trendingRest}

        onSelect={openDetail}

        onQuickDownload={quickDownload}

        quickBusy={quickBusy}

        onSeeAll={scrollToCatalog}

        onPreview={setHeroPreview}

      />

      <Rail

        title="Recently Added"

        games={recent}

        onSelect={openDetail}

        onQuickDownload={quickDownload}

        quickBusy={quickBusy}

        onSeeAll={scrollToCatalog}

        onPreview={setHeroPreview}

      />

      <HomeCatalog

        visible={visible}

        searchTerm={searchTerm}

        busy={busy}

        canLoadMore={canLoadMore}

        page={page}

        loadPage={loadPage}

        openDetail={openDetail}

        requestDownload={requestDownload}

        quickBusy={quickBusy}

      />

    </>

  );

}
