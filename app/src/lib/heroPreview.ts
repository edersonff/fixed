import type { GameEntry } from "../types";

type Listener = (game: GameEntry | null) => void;

let hovered: GameEntry | null = null;

const listeners = new Set<Listener>();

export function setHeroPreview(game: GameEntry | null): void {

  if (hovered === game) {

    return;

  }

  hovered = game;

  for (const listener of listeners) {

    listener(hovered);

  }

}

export function subscribeHeroPreview(listener: Listener): () => void {

  listeners.add(listener);

  return () => {

    listeners.delete(listener);

  };

}

export function currentHeroPreview(): GameEntry | null {

  return hovered;

}
