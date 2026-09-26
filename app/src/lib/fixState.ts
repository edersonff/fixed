export type FixState = "intact" | "missing" | "removedAgain";

// Pure decision the button reads from: files are fine, files need a Fix, or a Fix just watched
// them vanish again and the button locks until the player turns real-time protection off.
export function deriveFixState(missingCount: number, removedAgain: boolean): FixState {

  if (removedAgain) {

    return "removedAgain";

  }

  return missingCount > 0 ? "missing" : "intact";

}

export function fixLabelFor(state: FixState): string {

  return state === "intact" ? "Play" : "Fix";

}

export function fixPressEnabled(state: FixState): boolean {

  return state !== "removedAgain";

}
