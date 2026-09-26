type Listener = () => void;

let repairTitle: string | null = null;

const listeners = new Set<Listener>();

export const FILES_REMOVED = "files-removed-by-antivirus";

export function getRepairTitle(): string | null {

  return repairTitle;

}

export function subscribeRepair(listener: Listener): () => void {

  listeners.add(listener);

  return () => {

    listeners.delete(listener);

  };

}

export function openRepair(title: string | null): void {

  repairTitle = title;

  for (const listener of listeners) {

    listener();

  }

}
