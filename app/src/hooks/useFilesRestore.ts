import { useCallback, useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import type { AffectedGame, RestoreStatus } from "../lib/filesAlert";

import type { RestoreOutcome } from "../types";

const SUCCESS_MS = 4000;

const WATCHING_LABEL_DELAY_MS = 1500;

async function restoreAll(titles: string[]): Promise<boolean> {

  let again = false;

  for (const title of titles) {

    const outcome = await invoke<RestoreOutcome>("restore_game_files", { title }).catch(() => null);

    if (outcome?.removedAgain) {

      again = true;

    }

  }

  return again;

}

export function useFilesRestore(liveAffected: AffectedGame[], onSettled: () => void) {

  const [status, setStatus] = useState<RestoreStatus>("idle");

  const [removedAgain, setRemovedAgain] = useState(false);

  const [snapshot, setSnapshot] = useState<AffectedGame[]>([]);

  useEffect(() => {

    if (status !== "success") {

      return;

    }

    const timer = window.setTimeout(() => setStatus("idle"), SUCCESS_MS);

    return () => window.clearTimeout(timer);

  }, [status]);

  const restore = useCallback(() => {

    if (liveAffected.length === 0) {

      return;

    }

    setSnapshot(liveAffected);

    setStatus("restoring");

    setRemovedAgain(false);

    const titles = liveAffected.map((game) => game.title);

    const watchingTimer = window.setTimeout(() => setStatus("watching"), WATCHING_LABEL_DELAY_MS);

    restoreAll(titles).then((again) => {

      window.clearTimeout(watchingTimer);

      onSettled();

      setRemovedAgain(again);

      setStatus(again ? "idle" : "success");

    });

  }, [liveAffected, onSettled]);

  return { status, removedAgain, snapshot, restore };

}
