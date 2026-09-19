import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { open } from "@tauri-apps/plugin-dialog";

export function usePluginInstall(gameTitle: string) {

  const [pluginMsg, setPluginMsg] = useState("");

  async function addPlugin() {

    const selected = await open({

      multiple: false,

      filters: [{ name: "Plugin Package", extensions: ["zip", "rar", "dll"] }],

    });

    if (!selected || typeof selected !== "string") {

      return;

    }

    invoke<number>("install_plugin", { title: gameTitle, archivePath: selected })

      .then((count) => setPluginMsg(`Plugin Installed. ${count} File${count === 1 ? "" : "s"}`))

      .catch((reason: unknown) => setPluginMsg(`Plugin Failed: ${String(reason)}`));

  }

  return { pluginMsg, addPlugin };

}
