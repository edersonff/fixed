import { invoke } from "@tauri-apps/api/core";

import type { GameAssets } from "../types";

const resolved = new Map<string, GameAssets | null>();

const inFlight = new Map<string, Promise<GameAssets | null>>();

export function cachedGameAssets(title: string): GameAssets | null | undefined {

  return resolved.get(title);

}

export function loadGameAssets(title: string): Promise<GameAssets | null> {

  const done = resolved.get(title);

  if (done !== undefined) {

    return Promise.resolve(done);

  }

  const pending = inFlight.get(title);

  if (pending) {

    return pending;

  }

  const request = invoke<GameAssets | null>("game_assets", { title })

    .catch(() => null)

    .then((result) => {

      resolved.set(title, result ?? null);

      inFlight.delete(title);

      return result ?? null;

    });

  inFlight.set(title, request);

  return request;

}

export function prefetchGameAssets(titles: string[]): void {

  for (const title of titles) {

    if (title) {

      void loadGameAssets(title);

    }

  }

}
