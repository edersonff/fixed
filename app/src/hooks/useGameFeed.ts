import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import type { GameEntry } from "../types";

import type { GamesPage } from "../types";

function mergeGames(previous: GameEntry[], incoming: GameEntry[]) {

  const seen = new Set(previous.map((game) => game.pageUrl));

  return [...previous, ...incoming.filter((game) => !seen.has(game.pageUrl))];

}

export function useGameFeed() {

  const [games, setGames] = useState<GameEntry[]>([]);

  const [source, setSource] = useState("");

  const [page, setPage] = useState(1);

  const [busy, setBusy] = useState(false);

  const [error, setError] = useState<string | null>(null);

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

  useEffect(() => {

    loadPage(1);

  }, []);

  return { games, source, page, busy, error, setError, loadPage };

}
