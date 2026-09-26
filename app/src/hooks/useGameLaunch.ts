import { useCallback, useEffect, useRef, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { getCurrentWindow } from "@tauri-apps/api/window";

import { listen } from "@tauri-apps/api/event";

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

const PROTECTION_MESSAGE = "Windows keeps deleting this game's files. Turn protection off, then press Fix.";

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
export function useGameLaunch(gameTitle: string) {

  const [phase, setPhase] = useState(IDLE_PHASE);

  const [launchMsg, setLaunchMsg] = useState("");

  const [missing, setMissing] = useState<string[]>([]);

  const [fixing, setFixing] = useState(false);

  const [fixPhase, setFixPhase] = useState("");

  const [downloadPercent, setDownloadPercent] = useState<number | null>(null);

  const [removedAgain, setRemovedAgain] = useState(false);

  const fixingRef = useRef(false);

  fixingRef.current = fixing;

  const refreshStatus = useCallback(() => {

    invoke<GameFilesStatus>("game_files_status", { title: gameTitle })
      .then((status) => setMissing(status.missing))
      .catch(() => undefined);

  }, [gameTitle]);

  useEffect(() => {

    refreshStatus();

  }, [refreshStatus]);

  useEffect(() => {

    function onFocus() {

      refreshStatus();

      setRemovedAgain(false);

    }

    window.addEventListener("focus", onFocus);

    return () => window.removeEventListener("focus", onFocus);

  }, [refreshStatus]);

  useEffect(() => {

    const unlisten = listen<LaunchProgressPayload>("launch-progress", (event) => {

      if (event.payload.title !== gameTitle) {

        return;

      }

      setPhase(event.payload.phase);

      setLaunchMsg(launchPhaseMessage(event.payload.phase, event.payload.detail));

    });

    return () => {

      unlisten.then((stop) => stop());

    };

  }, [gameTitle]);

  useEffect(() => {

    const unlisten = listen<FixProgressPayload>("fix-progress", (event) => {

      if (event.payload.title !== gameTitle) {

        return;

      }

      setFixPhase(event.payload.phase);

      if (event.payload.phase === "removed-again") {

        setRemovedAgain(true);

      }

    });

    return () => {

      unlisten.then((stop) => stop());

    };

  }, [gameTitle]);

  useEffect(() => {

    const unlisten = listen<ProgressPayload>("download-progress", (event) => {

      if (event.payload.title !== gameTitle || !fixingRef.current) {

        return;

      }

      if (event.payload.state === "downloading" && event.payload.totalBytes > 0) {

        setDownloadPercent(Math.round((event.payload.downloadedBytes / event.payload.totalBytes) * 100));

        return;

      }

      if (event.payload.state === "extracting") {

        setDownloadPercent(null);

        setFixPhase("extracting");

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

    setFixing(true);

    setRemovedAgain(false);

    setFixPhase("checking");

    setDownloadPercent(null);

    invoke<string>("fix_game", { title: gameTitle })

      .then(() => {

        refreshStatus();

      })

      .catch((reason: unknown) => {

        const message = String(reason);

        if (message === PROTECTION_MESSAGE) {

          setRemovedAgain(true);

          return;

        }

        setLaunchMsg(`Fix Failed: ${message}`);

      })

      .finally(() => {

        setFixing(false);

        setFixPhase("");

        setDownloadPercent(null);

      });

  }

  const needsFix = missing.length > 0;

  const launching = !RESOLVED_LAUNCH_PHASES.has(phase);

  function press() {

    if (needsFix) {

      if (!fixing && !removedAgain) {

        fix();

      }

      return;

    }

    play();

  }

  const label = needsFix ? (fixing ? fixLabel(fixPhase, downloadPercent) : "Fix") : phase === "running" ? "Running" : launching ? "Starting" : "Play";

  const busy = launching || fixing || phase === "running";

  return { label, press, busy, needsFix, removedAgain, launchMsg };

}
