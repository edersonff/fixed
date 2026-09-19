import { useEffect, useState } from "react";

import { listen } from "@tauri-apps/api/event";

import { refreshInstalledTitles } from "../lib/installedGames";

import { titleKey } from "../lib/titleKey";

import type { DownloadEntry } from "../types";

import type { DownloadState } from "../types";

import type { ProgressPayload } from "../types";

export function useDownloadQueue() {

  const [downloads, setDownloads] = useState<DownloadEntry[]>([]);

  useEffect(() => {

    const unlisten = listen<ProgressPayload>("download-progress", (event) => {

      const { title, downloadedBytes, totalBytes, state } = event.payload;

      const mapped: DownloadState =

        state === "extracting"

          ? "extracting"

          : state === "ready"

            ? "ready"

            : state === "error"

              ? "error"

              : "torrenting";

      if (mapped === "ready") {

        refreshInstalledTitles();

      }

      setDownloads((previous) =>

        previous.map((entry) => {

          if (titleKey(entry.game.title) !== title) {

            return entry;

          }

          if (mapped === "torrenting" && (entry.state === "extracting" || entry.state === "ready")) {

            return entry;

          }

          if (mapped === "torrenting") {

            return { ...entry, state: mapped, downloadedBytes, totalBytes };

          }

          return { ...entry, state: mapped };

        }),

      );

    });

    return () => {

      unlisten.then((stop) => stop());

    };

  }, []);

  return { downloads, setDownloads };

}
