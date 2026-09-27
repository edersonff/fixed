import { useCallback, useEffect, useState, useSyncExternalStore } from "react";

import { invoke } from "@tauri-apps/api/core";

import { getCurrentWindow } from "@tauri-apps/api/window";

import { listen } from "@tauri-apps/api/event";

import { useTitleEvent } from "./useTitleEvent";

import { FILES_REMOVED_AGAIN, fixErrorMessage } from "../lib/fixErrors";

import { deriveFixState, fixLabelFor, fixPressEnabled } from "../lib/fixState";

import { getFixEntry, subscribeFixEntry, updateFixEntry } from "../lib/fixStore";

import type { FixProgressPayload } from "../types";

import type { GameFilesStatus } from "../types";

import type { LaunchProgressPayload } from "../types";

import type { ProgressPayload } from "../types";

const PHASE_LABELS: Record<string, string> = {

  checking: "Checking game",

  "stopping-steam": "Restarting Steam",

  registering: "Registering game",

  "starting-steam": "Starting Steam",

  "waiting-steam": "Waiting for Steam",

  launching: "Launching",

  done: "Running",

  running: "Running",

  exited: "Exited",

};

const IDLE_PHASE = "idle";

const RESOLVED_LAUNCH_PHASES = new Set(["idle", "running", "done", "exited", "failed"]);

function launchPhaseMessage(phase: string, detail: string): string {

  if (phase === "failed") {

    return `Launch Failed: ${detail}`;

  }

  const label = PHASE_LABELS[phase] ?? "Starting";

  return detail ? `${label} · ${detail}` : label;

}

function fixLabel(fixPhase: string, downloadPercent: number | null): string {

  if (downloadPercent !== null) {

    return `Downloading ${downloadPercent}%`;

  }

  if (fixPhase === "extracting") {

    return "Extracting…";

  }

  if (fixPhase === "checking") {

    return "Checking…";

  }

  return "Fixing…";

}

// One button, one flow: intact files show Play, missing files show Fix, and the same press()
// decides which action fires — never two buttons offering the player a choice mid-problem.
// The fix state itself (missing/fixing/progress/removedAgain) lives in fixStore, keyed by title,
// so the library card, the detail page and the home catalog card all agree on the same game.
export function useGameLaunch(gameTitle: string, installed: boolean) {

  const [phase, setPhase] = useState(IDLE_PHASE);

  const [launchMsg, setLaunchMsg] = useState("");

  const entry = useSyncExternalStore(

    useCallback((listener) => subscribeFixEntry(gameTitle, listener), [gameTitle]),

    () => getFixEntry(gameTitle),

  );

  const refreshStatus = useCallback(() => {

    if (!installed) {

      return;

    }

    invoke<GameFilesStatus>("game_files_status", { title: gameTitle })
      .then((status) => updateFixEntry(gameTitle, (previous) => ({ ...previous, missing: status.missing })))
      .catch(() => undefined);

  }, [gameTitle, installed]);

  useEffect(() => {

    refreshStatus();

  }, [refreshStatus]);

  useEffect(() => {

    if (!installed) {

      return;

    }

    function onFocus() {

      refreshStatus();

      updateFixEntry(gameTitle, (previous) => ({ ...previous, removedAgain: false }));

    }

    window.addEventListener("focus", onFocus);

    return () => window.removeEventListener("focus", onFocus);

  }, [refreshStatus, installed, gameTitle]);

  const onLaunchProgress = useCallback((payload: LaunchProgressPayload) => {

    setPhase(payload.phase);

    setLaunchMsg(launchPhaseMessage(payload.phase, payload.detail));

  }, []);

  useTitleEvent<LaunchProgressPayload>("launch-progress", gameTitle, onLaunchProgress);

  const onFixProgress = useCallback((payload: FixProgressPayload) => {

    updateFixEntry(gameTitle, (previous) => ({

      ...previous,

      fixPhase: payload.phase,

      removedAgain: payload.phase === "removed-again" ? true : previous.removedAgain,

    }));

  }, [gameTitle]);

  useTitleEvent<FixProgressPayload>("fix-progress", gameTitle, onFixProgress);

  useEffect(() => {

    const unlisten = listen<ProgressPayload>("download-progress", (event) => {

      if (event.payload.title !== gameTitle || !getFixEntry(gameTitle).fixing) {

        return;

      }

      if (event.payload.state === "downloading" && event.payload.totalBytes > 0) {

        const percent = Math.round((event.payload.downloadedBytes / event.payload.totalBytes) * 100);

        updateFixEntry(gameTitle, (previous) => ({ ...previous, downloadPercent: percent }));

        return;

      }

      if (event.payload.state === "extracting") {

        updateFixEntry(gameTitle, (previous) => ({ ...previous, downloadPercent: null, fixPhase: "extracting" }));

      }

    });

    return () => {

      unlisten.then((stop) => stop());

    };

  }, [gameTitle]);

  function play() {

    setPhase("checking");

    setLaunchMsg("Checking game");

    invoke<string>("launch_game", { title: gameTitle })

      .then((result) => {

        if (result.startsWith("launched:")) {

          getCurrentWindow().hide().catch(() => undefined);

        }

      })

      .catch((reason: unknown) => {

        setPhase("failed");

        setLaunchMsg(`Launch Failed: ${String(reason)}`);

      });

  }

  function fix() {

    updateFixEntry(gameTitle, (previous) => ({ ...previous, fixing: true, removedAgain: false, fixPhase: "checking", downloadPercent: null }));

    invoke<string>("fix_game", { title: gameTitle })

      .then(() => {

        refreshStatus();

      })

      .catch((reason: unknown) => {

        const code = String(reason);

        if (code === FILES_REMOVED_AGAIN) {

          updateFixEntry(gameTitle, (previous) => ({ ...previous, removedAgain: true }));

          return;

        }

        setLaunchMsg(`Fix Failed: ${fixErrorMessage(code) ?? code}`);

      })

      .finally(() => {

        updateFixEntry(gameTitle, (previous) => ({ ...previous, fixing: false, fixPhase: "", downloadPercent: null }));

      });

  }

  const fixState = deriveFixState(entry.missing.length, entry.removedAgain);

  const needsFix = fixState !== "intact";

  const launching = !RESOLVED_LAUNCH_PHASES.has(phase);

  function press() {

    if (needsFix) {

      if (!entry.fixing && fixPressEnabled(fixState)) {

        fix();

      }

      return;

    }

    play();

  }

  const label = needsFix ? (entry.fixing ? fixLabel(entry.fixPhase, entry.downloadPercent) : fixLabelFor(fixState)) : phase === "running" ? "Running" : launching ? "Starting" : fixLabelFor(fixState);

  const busy = launching || entry.fixing || phase === "running";

  return { label, press, busy, needsFix, removedAgain: entry.removedAgain, launchMsg, retryFix: fix };

}
