import { useCallback, useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import type { InstalledGame } from "../types";

export function useInstalledGames(active: boolean) {

  const [games, setGames] = useState<InstalledGame[]>([]);

  const [scanning, setScanning] = useState(false);

  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(() => {

    setScanning(true);

    setError(null);

    invoke<InstalledGame[]>("installed_games")

      .then((result) => setGames(result))

      .catch((reason: unknown) => setError(String(reason)))

      .finally(() => setScanning(false));

  }, []);

  useEffect(() => {

    if (active) {

      refresh();

    }

  }, [active, refresh]);

  return { games, scanning, error, refresh };

}
