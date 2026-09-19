import { useEffect, useMemo, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import type { GameDetail } from "../types";

import type { GameEntry } from "../types";

import type { GamesPage } from "../types";

export function useCatalog() {

  const [games, setGames] = useState<GameEntry[]>([]);

  const [source, setSource] = useState("");

  const [page, setPage] = useState(1);

  const [query, setQuery] = useState("");

  const [searchTerm, setSearchTerm] = useState("");

  const [busy, setBusy] = useState(false);

  const [error, setError] = useState<string | null>(null);

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

  function fetchDetail(url: string): Promise<GameDetail | null> {

    return invoke<GameDetail>("game_detail", { url }).catch(() => null);

  }

  useEffect(() => {

    loadPage(1);

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

  const canLoadMore = source === "live" && page >= 1 && !searchTerm;

  return {

    query,

    setQuery,

    runSearch,

    searchTerm,

    setSearchTerm,

    visible,

    trendingRest,

    recent,

    featured,

    busy,

    source,

    error,

    canLoadMore,

    loadPage,

    fetchDetail,

  };

}
