import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import type { GameDetail } from "../types";

import type { GameEntry } from "../types";

import type { GamesPage } from "../types";

import type { InstalledGame } from "../types";

import type { View } from "../types";

import type { useCatalog } from "./useCatalog";

export function useAppNavigation(catalog: ReturnType<typeof useCatalog>) {

  const [view, setView] = useState<View>("home");

  const [selected, setSelected] = useState<GameEntry | null>(null);

  const [detail, setDetail] = useState<GameDetail | null>(null);

  const [detailBusy, setDetailBusy] = useState(false);

  function openDetail(game: GameEntry) {

    setSelected(game);

    setDetail(null);

    setDetailBusy(true);

    catalog.fetchDetail(game.pageUrl)

      .then((result) => setDetail(result))

      .finally(() => setDetailBusy(false));

  }

  function openInstalledDetail(game: InstalledGame) {

    const searchKey = game.title.toLowerCase().replace(/[^a-z0-9]/g, "");

    const loadedGame = catalog.visible.find(

      (entry) => entry.title.toLowerCase().replace(/[^a-z0-9]/g, "") === searchKey,

    );

    if (loadedGame) {

      openDetail(loadedGame);

      return;

    }

    invoke<GamesPage>("find_games", { query: game.title })

      .then((result) => {

        const found = result.games.find(

          (entry) => entry.title.toLowerCase().replace(/[^a-z0-9]/g, "") === searchKey,

        ) ?? result.games[0];

        if (found) {

          openDetail(found);

          return;

        }

        catalog.setError(`Could not find details for ${game.title}`);

      })

      .catch((reason: unknown) => catalog.setError(String(reason)));

  }

  function switchView(target: View) {

    setSelected(null);

    catalog.setQuery("");

    catalog.setSearchTerm("");

    setView(target);

  }

  return {

    view,

    selected,

    setSelected,

    detail,

    detailBusy,

    openDetail,

    openInstalledDetail,

    switchView,

    setView,

  };

}
