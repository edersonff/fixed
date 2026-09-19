import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { listen } from "@tauri-apps/api/event";

import type { DownloadEntry } from "../types";

import type { DownloadState } from "../types";

import type { GameDetail } from "../types";

import type { GameEntry } from "../types";

import type { ProgressPayload } from "../types";

import type { View } from "../types";

export function useDownloads({

  setView,

  setSelected,

  openDetail,

}: {

  setView: (view: View) => void;

  setSelected: (game: GameEntry | null) => void;

  openDetail: (game: GameEntry) => void;

}) {

  const [downloads, setDownloads] = useState<DownloadEntry[]>([]);

  const [quickBusy, setQuickBusy] = useState(false);

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

              : state === "stopped"

                ? "stopped"

                : "torrenting";

      setDownloads((previous) =>

        previous.map((entry) => {

          if (entry.game.title.replace(/\//g, "_") !== title) {

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

  function startTorrent(game: GameEntry, gameDetail: GameDetail) {

    const torrentLane = gameDetail.lanes.find(

      (lane) => lane.kind === "torrent" && lane.url.startsWith("https://uploads.online-fix.me"),

    );

    if (!torrentLane) {

      openDetail(game);

      return;

    }

    const safeTitle = game.title.replace(/\//g, "_");

    setDownloads((previous) => [

      { game, state: "torrenting", lane: "torrent", parts: [], downloadedBytes: 0, totalBytes: 0 },

      ...previous.filter((entry) => entry.game.pageUrl !== game.pageUrl),

    ]);

    setSelected(null);

    setView("downloads");

    invoke<string>("start_torrent_download", { title: safeTitle, laneUrl: torrentLane.url })

      .catch((reason: unknown) => {

        console.error("start_torrent_download failed:", reason);

        setDownloads((previous) =>

          previous.map((entry) =>

            entry.game.pageUrl === game.pageUrl

              ? { ...entry, state: "error", errorMsg: String(reason).slice(0, 140) }

              : entry,

          ),

        );

      });

  }

  function startHttp(game: GameEntry, hostersUrl: string, gameDetail: GameDetail | null) {

    const safeTitle = game.title.replace(/\//g, "_");

    setDownloads((previous) => [

      { game, state: "torrenting", lane: "http", parts: [], downloadedBytes: 0, totalBytes: 0 },

      ...previous.filter((entry) => entry.game.pageUrl !== game.pageUrl),

    ]);

    setSelected(null);

    setView("downloads");

    invoke<string>("start_http_download", { title: safeTitle, laneUrl: hostersUrl })

      .catch((reason: unknown) => {

        console.error("start_http_download failed:", reason);

        if (gameDetail) {

          startTorrent(game, gameDetail);

          return;

        }

        setDownloads((previous) =>

          previous.map((entry) =>

            entry.game.pageUrl === game.pageUrl

              ? { ...entry, state: "error", errorMsg: String(reason).slice(0, 140) }

              : entry,

          ),

        );

      });

  }

  function downloadWithDetail(game: GameEntry, gameDetail: GameDetail) {

    const hostersLane = gameDetail.lanes.find((lane) => lane.kind === "hosters");

    if (hostersLane) {

      startHttp(game, hostersLane.url, gameDetail);

      return;

    }

    startTorrent(game, gameDetail);

  }

  function requestDownload(game: GameEntry, gameDetail: GameDetail | null) {

    if (gameDetail) {

      downloadWithDetail(game, gameDetail);

      return;

    }

    setQuickBusy(true);

    invoke<GameDetail>("game_detail", { url: game.pageUrl })

      .then((result) => downloadWithDetail(game, result))

      .catch((reason: unknown) => console.error("quick download failed:", reason))

      .finally(() => setQuickBusy(false));

  }

  function stopAll() {

    invoke("cancel_all_downloads").catch((reason: unknown) => console.error(reason));

    setDownloads((previous) =>

      previous.map((entry) => (entry.state === "torrenting" ? { ...entry, state: "stopped" } : entry)),

    );

  }

  function cancelDownload(entry: DownloadEntry) {

    invoke("cancel_download", { title: entry.game.title }).catch(() => undefined);

    setDownloads((previous) =>

      previous.map((current) =>

        current.game.pageUrl === entry.game.pageUrl ? { ...current, state: "stopped" } : current,

      ),

    );

  }

  return {

    downloads,

    quickBusy,

    startTorrent,

    startHttp,

    requestDownload,

    stopAll,

    cancelDownload,

  };

}
