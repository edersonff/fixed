import { listen } from "@tauri-apps/api/event";

import { refreshInstalledTitles } from "./installedGames";

import { titleKey } from "./titleKey";

import type { DownloadEntry } from "../types";

import type { DownloadState } from "../types";

import type { ProgressPayload } from "../types";

type Listener = () => void;

let entries: DownloadEntry[] = [];

const listeners = new Set<Listener>();

export function getDownloadEntries(): DownloadEntry[] {

  return entries;

}

export function subscribeDownloads(listener: Listener): () => void {

  listeners.add(listener);

  return () => {

    listeners.delete(listener);

  };

}

function notify(): void {

  for (const listener of listeners) {

    listener();

  }

}

export function updateDownloadEntries(updater: (previous: DownloadEntry[]) => DownloadEntry[]): void {

  entries = updater(entries);

  notify();

}

function mapPayloadState(state: string): DownloadState {

  if (state === "extracting") {

    return "extracting";

  }

  if (state === "ready") {

    return "ready";

  }

  if (state === "error") {

    return "error";

  }

  if (state === "stopped") {

    return "stopped";

  }

  return "torrenting";

}

void listen<ProgressPayload>("download-progress", (event) => {

  const { title, downloadedBytes, totalBytes, state } = event.payload;

  const mapped = mapPayloadState(state);

  if (mapped === "ready") {

    refreshInstalledTitles();

  }

  updateDownloadEntries((previous) =>

    previous.map((entry) => {

      if (titleKey(entry.game.title) !== title) {

        return entry;

      }

      if (mapped === "torrenting" && entry.state !== "torrenting" && entry.state !== "resolving") {

        return entry;

      }

      if (mapped === "torrenting") {

        return { ...entry, state: mapped, downloadedBytes, totalBytes };

      }

      return { ...entry, state: mapped };

    }),

  );

});
