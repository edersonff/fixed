import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { listen } from "@tauri-apps/api/event";

import type { LaunchProgressPayload } from "../types";

const PHASE_LABELS: Record<string, string> = {

  checking: "Checking game",

  "stopping-steam": "Restarting Steam",

  registering: "Registering game",

  "starting-steam": "Starting Steam",

  "waiting-steam": "Waiting for Steam",

  launching: "Launching",

};

function phaseMessage(phase: string, detail: string): string {

  if (phase === "failed") {

    return `Launch Failed: ${detail}`;

  }

  if (phase === "done") {

    return "Started";

  }

  const label = PHASE_LABELS[phase] ?? "Starting";

  return detail ? `${label} · ${detail}` : label;

}

export function useGameLaunch(gameTitle: string) {

  const [launching, setLaunching] = useState(false);

  const [launchMsg, setLaunchMsg] = useState("");

  useEffect(() => {

    const unlisten = listen<LaunchProgressPayload>("launch-progress", (event) => {

      const progress = event.payload;

      if (progress.title !== gameTitle) {

        return;

      }

      setLaunchMsg(phaseMessage(progress.phase, progress.detail));

      if (progress.phase === "failed" || progress.phase === "done") {

        setLaunching(false);

      } else {

        setLaunching(true);

      }

    });

    return () => {

      unlisten.then((stop) => stop());

    };

  }, [gameTitle]);

  function launch() {

    setLaunching(true);

    setLaunchMsg("Checking game");

    invoke<string>("launch_game", { title: gameTitle })

      .catch((reason: unknown) => {

        setLaunchMsg(`Launch Failed: ${String(reason)}`);

      })

      .finally(() => setLaunching(false));

  }

  return { launching, launchMsg, launch };

}
