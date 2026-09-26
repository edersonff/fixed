import { useMemo } from "react";

import { affectedGames, affectedKey, isAlertVisible } from "../lib/filesAlert";

import { useFilesRestore } from "./useFilesRestore";

import { useHiddenAlertKey } from "./useHiddenAlertKey";

import { useLiveInstalledGames } from "./useLiveInstalledGames";

export function useFilesAlert() {

  const { games, refresh } = useLiveInstalledGames();

  const liveAffected = useMemo(() => affectedGames(games), [games]);

  const currentKey = affectedKey(liveAffected);

  const { hiddenKey, hide } = useHiddenAlertKey(currentKey);

  const { status, removedAgain, snapshot, restore } = useFilesRestore(liveAffected, refresh);

  const hidden = status === "idle" && hiddenKey === currentKey && currentKey !== "";

  const affected = status === "idle" ? liveAffected : snapshot;

  const visible = isAlertVisible(status, affected, hidden);

  return { visible, affected, status, removedAgain, hide, restore };

}
