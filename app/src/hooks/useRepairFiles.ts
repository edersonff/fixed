import { useCallback, useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import type { GameFilesStatus } from "../types";

const POLL_MS = 2000;

export function useRepairFiles(title: string | null) {

  const [status, setStatus] = useState<GameFilesStatus | null>(null);

  const [restoring, setRestoring] = useState(false);

  const [restored, setRestored] = useState(false);

  const [error, setError] = useState("");

  useEffect(() => {

    setStatus(null);

    setRestored(false);

    setError("");

    if (!title) {

      return;

    }

    let alive = true;

    const read = () => {

      invoke<GameFilesStatus>("game_files_status", { title })

        .then((next) => alive && setStatus(next))

        .catch((reason: unknown) => alive && setError(String(reason)));

    };

    read();

    const timer = window.setInterval(read, POLL_MS);

    return () => {

      alive = false;

      window.clearInterval(timer);

    };

  }, [title]);

  const restore = useCallback(() => {

    if (!title) {

      return;

    }

    setRestoring(true);

    setError("");

    invoke<GameFilesStatus>("restore_game_files", { title })

      .then((next) => {

        setStatus(next);

        setRestored(next.missing.length === 0);

      })

      .catch((reason: unknown) => setError(String(reason)))

      .finally(() => setRestoring(false));

  }, [title]);

  return { status, restoring, restored, error, restore };

}
