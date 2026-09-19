import { useMemo } from "react";

import type { GameEntry } from "../types";

export function useFeaturedRails(visible: GameEntry[]) {

  const trending = useMemo(

    () => [...visible].sort((a, b) => b.views - a.views).slice(0, 7),

    [visible],

  );

  const recent = useMemo(

    () => [...visible].sort((a, b) => b.publishedAt.localeCompare(a.publishedAt)).slice(0, 6),

    [visible],

  );

  const featured = recent[0] ?? trending[0] ?? null;

  const trendingRest = trending.filter((game) => featured?.pageUrl !== game.pageUrl);

  return { trending, recent, featured, trendingRest };

}
