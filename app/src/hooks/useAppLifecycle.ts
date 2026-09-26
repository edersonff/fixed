import { useEffect, useState } from "react";

import { getVersion } from "@tauri-apps/api/app";

import { ensureInstalledTitlesLoaded } from "../lib/installedGames";

import { useMouseBackNavigation } from "./useMouseBackNavigation";

import type { GameEntry } from "../types";

export function useAppLifecycle(setSelected: (game: GameEntry | null) => void) {

  const [appVersion, setAppVersion] = useState("");

  useEffect(() => {

    getVersion()

      .then((version) => setAppVersion(version))

      .catch(() => setAppVersion(""));

  }, []);

  useEffect(() => {

    ensureInstalledTitlesLoaded();

  }, []);

  useMouseBackNavigation(setSelected);

  return { appVersion };

}
