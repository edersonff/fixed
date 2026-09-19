import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import type { DownloadEntry } from "../types";

import type { GameDetail } from "../types";

import type { GameEntry } from "../types";

import type { View } from "../types";

export function useDownloadStart({

  setDownloads,

  setView,

  setSelected,

  openDetail,

  onTorrentOnly,

  isBusy,

}: {

  setDownloads: (updater: (previous: DownloadEntry[]) => DownloadEntry[]) => void;

  setView: (view: View) => void;

  setSelected: (game: GameEntry | null) => void;

  openDetail: (game: GameEntry) => void;

  onTorrentOnly: (game: GameEntry, gameDetail: GameDetail) => void;

  isBusy: (pageUrl: string) => boolean;

}) {

  const [quickBusy, setQuickBusy] = useState(false);

  function startTorrent(game: GameEntry, gameDetail: GameDetail) {

    if (isBusy(game.pageUrl)) {

      setView("downloads");

      return;

    }

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

  function startHttp(game: GameEntry, hostersUrl: string) {

    if (isBusy(game.pageUrl)) {

      setView("downloads");

      return;

    }

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

        setDownloads((previous) =>

          previous.map((entry) =>

            entry.game.pageUrl === game.pageUrl

              ? { ...entry, state: "error", errorMsg: `${String(reason).slice(0, 110)} · torrent available in details` }

              : entry,

          ),

        );

      });

  }

  function downloadWithDetail(game: GameEntry, gameDetail: GameDetail) {

    const hostersLane = gameDetail.lanes.find((lane) => lane.kind === "hosters");

    if (hostersLane) {

      startHttp(game, hostersLane.url);

      return;

    }

    onTorrentOnly(game, gameDetail);

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

  return { quickBusy, startTorrent, startHttp, requestDownload };

}
