import { displayTitle } from "./format";

import type { InstalledGame } from "../types";

export type AffectedGame = {
  title: string;
  canRestore: boolean;
};

export type RestoreStatus = "idle" | "restoring" | "watching" | "success";

export function affectedGames(games: InstalledGame[]): AffectedGame[] {

  return games
    .filter((game) => game.missingFiles > 0)
    .map((game) => ({ title: game.title, canRestore: game.canRestore }));

}

export function affectedKey(affected: AffectedGame[]): string {

  return affected
    .map((game) => game.title)
    .sort()
    .join("|");

}

export function anyCanRestore(affected: AffectedGame[]): boolean {

  return affected.some((game) => game.canRestore);

}

function gameLabel(affected: AffectedGame[]): string {

  if (affected.length === 1) {

    return displayTitle(affected[0].title);

  }

  return `${affected.length} games`;

}

export function alertHeadline(affected: AffectedGame[], removedAgain: boolean): string {

  if (removedAgain) {

    return "Windows removed the files again";

  }

  return `Windows security removed files from ${gameLabel(affected)}`;

}

export function alertLine(affected: AffectedGame[], removedAgain: boolean): string {

  if (removedAgain) {

    return "Turn off Real-time protection, then press Restore files.";

  }

  if (!anyCanRestore(affected)) {

    return "The download file was deleted. Download the game again to get the files back.";

  }

  return "The game opens, but online play will not work until the files are back.";

}

export function restoreButtonLabel(status: RestoreStatus): string {

  if (status === "restoring") {

    return "Restoring…";

  }

  if (status === "watching") {

    return "Checking that Windows keeps them…";

  }

  return "Restore files";

}

export function isAlertVisible(status: RestoreStatus, affected: AffectedGame[], hidden: boolean): boolean {

  if (status !== "idle") {

    return true;

  }

  return affected.length > 0 && !hidden;

}
