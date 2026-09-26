export type FixEntry = {

  missing: string[];

  fixing: boolean;

  fixPhase: string;

  downloadPercent: number | null;

  removedAgain: boolean;

};

const EMPTY_ENTRY: FixEntry = {

  missing: [],

  fixing: false,

  fixPhase: "",

  downloadPercent: null,

  removedAgain: false,

};

type Listener = () => void;

const entries = new Map<string, FixEntry>();

const listeners = new Map<string, Set<Listener>>();

// One entry per game title, read by every surface that renders a Play/Fix button for that game
// (library card, detail page, home catalog card) — a Fix started on one surface must be visible
// as fixing/removedAgain on every other surface showing the same game.
export function getFixEntry(title: string): FixEntry {

  return entries.get(title) ?? EMPTY_ENTRY;

}

export function subscribeFixEntry(title: string, listener: Listener): () => void {

  const set = listeners.get(title) ?? new Set<Listener>();

  set.add(listener);

  listeners.set(title, set);

  return () => {

    set.delete(listener);

  };

}

function notify(title: string): void {

  for (const listener of listeners.get(title) ?? []) {

    listener();

  }

}

export function updateFixEntry(title: string, updater: (previous: FixEntry) => FixEntry): void {

  const previous = entries.get(title) ?? EMPTY_ENTRY;

  entries.set(title, updater(previous));

  notify(title);

}
