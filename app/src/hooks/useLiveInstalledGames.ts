import { useCallback, useEffect, useRef, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { listen } from "@tauri-apps/api/event";

import type { InstalledGame, ProgressPayload } from "../types";

const VISIBLE_REFRESH_MS = 10000;

export function useLiveInstalledGames() {

  const [games, setGames] = useState<InstalledGame[]>([]);

  const gamesRef = useRef<InstalledGame[]>([]);

  gamesRef.current = games;

  const refresh = useCallback(async (): Promise<InstalledGame[]> => {

    try {

      const fresh = await invoke<InstalledGame[]>("installed_games");

      setGames(fresh);

      return fresh;

    } catch {

      return gamesRef.current;

    }

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

  useEffect(() => {

    const interval = window.setInterval(() => {

      if (document.visibilityState === "visible") {

        refresh();

      }

    }, VISIBLE_REFRESH_MS);

    return () => window.clearInterval(interval);

  }, [refresh]);

  return { games, refresh };

}
