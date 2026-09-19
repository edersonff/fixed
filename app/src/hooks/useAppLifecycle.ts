import { useEffect, useState } from "react";

import { getVersion } from "@tauri-apps/api/app";

import { shouldShowDefenderModal } from "../components/DefenderModal";

import { ensureInstalledTitlesLoaded } from "../lib/installedGames";

import { useMouseBackNavigation } from "./useMouseBackNavigation";

import type { GameEntry } from "../types";

export function useAppLifecycle(setSelected: (game: GameEntry | null) => void) {

  const [appVersion, setAppVersion] = useState("");

  const [showDefender, setShowDefender] = useState(false);

  useEffect(() => {

    if (!shouldShowDefenderModal()) {

      return;

    }

    const timer = window.setTimeout(() => setShowDefender(true), 1500);

    return () => window.clearTimeout(timer);

  }, []);

  useEffect(() => {

    getVersion()

      .then((version) => setAppVersion(version))

      .catch(() => setAppVersion(""));

  }, []);

  useEffect(() => {

    ensureInstalledTitlesLoaded();

  }, []);

  useMouseBackNavigation(setSelected);

  return { appVersion, showDefender, setShowDefender };

}
