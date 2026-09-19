import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

export function useGameLaunch(gameTitle: string) {

  const [launching, setLaunching] = useState(false);

  const [launchMsg, setLaunchMsg] = useState("");

  function launch() {

    setLaunching(true);

    setLaunchMsg("");

    invoke<string>("launch_game", { title: gameTitle })

      .catch((reason: unknown) => setLaunchMsg(`Launch Failed: ${String(reason)}`))

      .finally(() => setLaunching(false));

  }

  return { launching, launchMsg, launch };

}
