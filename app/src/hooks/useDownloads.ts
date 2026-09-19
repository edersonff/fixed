import { useCallback } from "react";

import { useDownloadActions } from "./useDownloadActions";

import { useDownloadQueue } from "./useDownloadQueue";

import { getDownloadEntries } from "../lib/downloadsStore";

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

  const { setDownloads } = useDownloadQueue();

  const isBusy = useCallback(

    (pageUrl: string) =>

      getDownloadEntries().some(

        (entry) =>

          entry.game.pageUrl === pageUrl &&

          (entry.state === "torrenting" || entry.state === "extracting" || entry.state === "resolving"),

      ),

    [],

  );

  const actions = useDownloadActions({ setDownloads, setView, setSelected, openDetail, onError, onTorrentOnly, isBusy });

  return actions;

}
