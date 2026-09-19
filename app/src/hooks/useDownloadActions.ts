import { invoke } from "@tauri-apps/api/core";

import { useDownloadStart } from "./useDownloadStart";

import type { DownloadEntry } from "../types";

import type { GameDetail } from "../types";

import type { GameEntry } from "../types";

import type { View } from "../types";

export function useDownloadActions({

  setDownloads,

  setView,

  setSelected,

  openDetail,

  onError,

  onTorrentOnly,

}: {

  setDownloads: (updater: (previous: DownloadEntry[]) => DownloadEntry[]) => void;

  setView: (view: View) => void;

  setSelected: (game: GameEntry | null) => void;

  openDetail: (game: GameEntry) => void;

  onError: (message: string) => void;

  onTorrentOnly: (game: GameEntry, gameDetail: GameDetail) => void;

}) {

  const start = useDownloadStart({ setDownloads, setView, setSelected, openDetail, onTorrentOnly });

  function stopAll() {

    invoke("cancel_all_downloads").catch((reason: unknown) => onError(String(reason)));

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

  return { ...start, stopAll, cancelDownload };

}
