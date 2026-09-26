import { useCallback, useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { listen } from "@tauri-apps/api/event";

import type { InstalledGame, ProgressPayload } from "../types";

export function useLiveInstalledGames() {

  const [games, setGames] = useState<InstalledGame[]>([]);

  const refresh = useCallback(() => {

    invoke<InstalledGame[]>("installed_games").then(setGames).catch(() => undefined);

  }, []);

  useEffect(() => {

    refresh();

    window.addEventListener("focus", refresh);

    return () => window.removeEventListener("focus", refresh);

  }, [refresh]);

  useEffect(() => {

    const unlisten = listen<ProgressPayload>("download-progress", (event) => {

      if (event.payload.state === "ready") {

        refresh();

      }

    });

    return () => {

      unlisten.then((stop) => stop());

    };

  }, [refresh]);

  return { games, refresh };

}
