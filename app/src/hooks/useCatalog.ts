import { invoke } from "@tauri-apps/api/core";

import { useGameFeed } from "./useGameFeed";

import { useGameSearch } from "./useGameSearch";

import type { GameDetail } from "../types";

function fetchDetail(url: string): Promise<GameDetail | null> {

  return invoke<GameDetail>("game_detail", { url }).catch(() => null);

}

export function useCatalog() {

  const feed = useGameFeed();

  const search = useGameSearch({

    games: feed.games,

    source: feed.source,

    busy: feed.busy,

    page: feed.page,

    loadPage: feed.loadPage,

  });

  return {

    query: search.query,

    setQuery: search.setQuery,

    runSearch: search.runSearch,

    searchTerm: search.searchTerm,

    setSearchTerm: search.setSearchTerm,

    page: feed.page,

    visible: search.visible,

    trendingRest: search.trendingRest,

    recent: search.recent,

    featured: search.featured,

    busy: feed.busy,

    source: feed.source,

    error: feed.error,

    setError: feed.setError,

    canLoadMore: search.canLoadMore,

    loadPage: feed.loadPage,

    fetchDetail,

  };

}
