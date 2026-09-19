import { invoke } from "@tauri-apps/api/core";

import { titleKey } from "./titleKey";

import type { InstalledGame } from "../types";

type Listener = (titles: Set<string>) => void;

let titles = new Set<string>();

let loaded = false;

const listeners = new Set<Listener>();

function notify(): void {

  for (const listener of listeners) {

    listener(titles);

  }

}

export function currentInstalledTitles(): Set<string> {

  return titles;

}

export function isGameInstalled(gameTitle: string): boolean {

  return titles.has(titleKey(gameTitle));

}

export function refreshInstalledTitles(): void {

  invoke<InstalledGame[]>("installed_games")

    .then((games) => {

      titles = new Set(games.map((game) => game.title));

      loaded = true;

      notify();

    })

    .catch(() => {

      titles = new Set();

      loaded = true;

      notify();

    });

}

export function ensureInstalledTitlesLoaded(): void {

  if (!loaded) {

    refreshInstalledTitles();

  }

}

export function subscribeInstalledTitles(listener: Listener): () => void {

  listeners.add(listener);

  return () => {

    listeners.delete(listener);

  };

}
