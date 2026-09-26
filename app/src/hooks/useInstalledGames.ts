import { useCallback, useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { refreshInstalledTitles } from "../lib/installedGames";

import { getRepairTitle, subscribeRepair } from "../lib/repairStore";

import type { InstalledGame } from "../types";

export function useInstalledGames(active: boolean) {

  const [games, setGames] = useState<InstalledGame[]>([]);

  const [scanning, setScanning] = useState(false);

  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(() => {

    setScanning(true);

    setError(null);

    invoke<InstalledGame[]>("installed_games")

      .then((result) => {

        setGames(result);

        refreshInstalledTitles();

      })

      .catch((reason: unknown) => setError(String(reason)))

      .finally(() => setScanning(false));

  }, []);

  useEffect(() => {

    if (active) {

      refresh();

    }

  }, [active, refresh]);

  useEffect(() => {

    if (!active) {

      return;

    }

    return subscribeRepair(() => {

      if (getRepairTitle() === null) {

        refresh();

      }

    });

  }, [active, refresh]);

  return { games, scanning, error, refresh };

}
