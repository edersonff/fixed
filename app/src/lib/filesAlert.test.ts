import { restoreOutcome } from "./filesAlert.ts";


import type { InstalledGame } from "../types";

function assertEqual(actual: boolean, expected: boolean, label: string): void {

  if (actual !== expected) {

    throw new Error(`${label}: expected ${expected}, got ${actual}`);

  }

  console.log(`ok - ${label}`);

}

function game(title: string, missingFiles: number, canRestore: boolean): InstalledGame {

  return {
    title,
    folder: `/games/${title}`,
    exe: `${title}.exe`,
    bytes: 0,
    hasPlugins: false,
    missingFiles,
    canRestore,
    downloadBytes: 0,
  };

}

const allFixed = restoreOutcome([game("A", 0, true), game("B", 0, false)], false);

assertEqual(allFixed.success, true, "no installed game missing files counts as success");

assertEqual(allFixed.removedAgain, false, "a success carries no removedAgain flag");

const oneUnrecoverable = restoreOutcome([game("A", 0, true), game("B", 2, false)], false);

assertEqual(oneUnrecoverable.success, false, "one game still missing files with no archive is not success");

assertEqual(oneUnrecoverable.removedAgain, false, "an unrecoverable game with no watch hit reports no removedAgain");

const vanishedAgain = restoreOutcome([game("A", 1, true)], true);

assertEqual(vanishedAgain.success, false, "a game still missing files after the watch is not success");

assertEqual(vanishedAgain.removedAgain, true, "a restore whose watch caught files vanishing again reports removedAgain");
