import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

export function useFreeDownload(title: string, onFreed: () => void) {

  const [freeing, setFreeing] = useState(false);

  function freeDownload() {

    setFreeing(true);

    invoke<number>("delete_game_download", { title })

      .then(() => onFreed())

      .finally(() => setFreeing(false));

  }

  return { freeing, freeDownload };

}
