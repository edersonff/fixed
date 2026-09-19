import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

export function useGameUninstall(title: string, onUninstalled: () => void) {

  const [confirming, setConfirming] = useState(false);

  const [uninstalling, setUninstalling] = useState(false);

  const [uninstallMsg, setUninstallMsg] = useState("");

  function handleUninstall() {

    setUninstalling(true);

    setUninstallMsg("");

    invoke<string>("uninstall_game", { title, removeFromSteam: true })

      .then(() => onUninstalled())

      .catch((reason: unknown) => setUninstallMsg(`Could not uninstall: ${String(reason)}`))

      .finally(() => setUninstalling(false));

  }

  return { confirming, setConfirming, uninstalling, uninstallMsg, setUninstallMsg, handleUninstall };

}
