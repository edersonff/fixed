import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { getCurrentWindow } from "@tauri-apps/api/window";

import { listen } from "@tauri-apps/api/event";

import { FILES_REMOVED, openRepair } from "../lib/repairStore";

import type { LaunchProgressPayload } from "../types";

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

const RESOLVED_PHASES = new Set(["idle", "running", "done", "exited", "failed"]);

function phaseMessage(phase: string, detail: string): string {

  if (phase === "failed") {

    return `Launch Failed: ${detail}`;

  }

  const label = PHASE_LABELS[phase] ?? "Starting";

  return detail ? `${label} · ${detail}` : label;

}

export function useGameLaunch(gameTitle: string) {

  const [phase, setPhase] = useState(IDLE_PHASE);

  const [launchMsg, setLaunchMsg] = useState("");

  useEffect(() => {

    const unlisten = listen<LaunchProgressPayload>("launch-progress", (event) => {

      const progress = event.payload;

      if (progress.title !== gameTitle) {

        return;

      }

      setPhase(progress.phase);

      setLaunchMsg(phaseMessage(progress.phase, progress.detail));

    });

    return () => {

      unlisten.then((stop) => stop());

    };

  }, [gameTitle]);

  function launch() {

    setPhase("checking");

    setLaunchMsg("Checking game");

    invoke<string>("launch_game", { title: gameTitle })

      .then((result) => {

        if (result.startsWith("launched:")) {

          getCurrentWindow().hide().catch(() => undefined);

        }

      })

      .catch((reason: unknown) => {

        if (String(reason) === FILES_REMOVED) {

          setPhase(IDLE_PHASE);

          setLaunchMsg("");

          openRepair(gameTitle);

          return;

        }

        setPhase("failed");

        setLaunchMsg(`Launch Failed: ${String(reason)}`);

      });

  }

  const launching = !RESOLVED_PHASES.has(phase);

  return { launching, launchMsg, phase, launch };

}
