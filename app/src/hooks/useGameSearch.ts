import { useEffect, useMemo, useState } from "react";

import { useFeaturedRails } from "./useFeaturedRails";

import type { GameEntry } from "../types";

export function useGameSearch({

  games,

  source,

  busy,

  page,

  loadPage,

}: {

  games: GameEntry[];

  source: string;

  busy: boolean;

  page: number;

  loadPage: (target: number) => void;

}) {

  const [query, setQuery] = useState("");

  const [searchTerm, setSearchTerm] = useState("");

  function runSearch() {

    setSearchTerm(query.trim().toLowerCase());

  }

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

  const { recent, featured, trendingRest } = useFeaturedRails(visible);

  const canLoadMore = source === "live" && page >= 1 && !searchTerm;

  return {

    query,

    setQuery,

    runSearch,

    searchTerm,

    setSearchTerm,

    visible,

    recent,

    featured,

    trendingRest,

    canLoadMore,

  };

}
