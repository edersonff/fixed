import { useCallback } from "react";

import { useDownloadActions } from "./useDownloadActions";

import { useDownloadQueue } from "./useDownloadQueue";

import type { GameDetail } from "../types";

import type { GameEntry } from "../types";

import type { View } from "../types";

export function useDownloads({

  setView,

  setSelected,

  openDetail,

  onError,

  onTorrentOnly,

}: {

  setView: (view: View) => void;

  setSelected: (game: GameEntry | null) => void;

  openDetail: (game: GameEntry) => void;

  onError: (message: string) => void;

  onTorrentOnly: (game: GameEntry, gameDetail: GameDetail) => void;

}) {

  const { downloads, setDownloads } = useDownloadQueue();

  const isBusy = useCallback(

    (pageUrl: string) =>

      downloads.some(

        (entry) =>

          entry.game.pageUrl === pageUrl &&

          (entry.state === "torrenting" || entry.state === "extracting" || entry.state === "resolving"),

      ),

    [downloads],

  );

  const actions = useDownloadActions({ setDownloads, setView, setSelected, openDetail, onError, onTorrentOnly, isBusy });

  return { downloads, ...actions };

}
