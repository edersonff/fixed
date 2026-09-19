import { useEffect, useState } from "react";

import {

  ensureInstalledTitlesLoaded,

  isGameInstalled,

  subscribeInstalledTitles,

} from "../lib/installedGames";

export function useIsGameInstalled(gameTitle: string): boolean {

  const [installed, setInstalled] = useState(() => isGameInstalled(gameTitle));

  useEffect(() => {

    ensureInstalledTitlesLoaded();

    setInstalled(isGameInstalled(gameTitle));

    return subscribeInstalledTitles(() => {

      setInstalled(isGameInstalled(gameTitle));

    });

  }, [gameTitle]);

  return installed;

}
