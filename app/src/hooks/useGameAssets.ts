import { useEffect, useState } from "react";

import { cachedGameAssets, loadGameAssets } from "../lib/gameAssets";

import type { GameAssets } from "../types";

export function useGameAssets(title: string): GameAssets | null | undefined {

  const [assets, setAssets] = useState<GameAssets | null | undefined>(() => cachedGameAssets(title));

  useEffect(() => {

    let cancelled = false;

    if (!title) {

      setAssets(null);

      return;

    }

    const known = cachedGameAssets(title);

    setAssets(known);

    if (known !== undefined) {

      return;

    }

    loadGameAssets(title).then((result) => {

      if (!cancelled) {

        setAssets(result);

      }

    });

    return () => {

      cancelled = true;

    };

  }, [title]);

  return assets;

}
