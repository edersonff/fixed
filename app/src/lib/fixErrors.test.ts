import { FILES_REMOVED_AGAIN, GAME_PAGE_NEEDED, fixErrorMessage } from "./fixErrors.ts";

function assertEqual<T>(actual: T, expected: T, label: string): void {

  if (actual !== expected) {

    throw new Error(`${label}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`);

  }

  console.log(`ok - ${label}`);

}

assertEqual(

  fixErrorMessage(FILES_REMOVED_AGAIN),

  "Windows keeps removing this game's online files. Allow the games folder once and FIXED fixes it.",

  "files-removed-again maps to the protection sentence",

);

assertEqual(

  fixErrorMessage(GAME_PAGE_NEEDED),

  "Open this game's page and press Download to get its files again.",

  "game-page-needed maps to the redownload sentence",

);

assertEqual(fixErrorMessage("unknown-code"), null, "an unrecognised code has no fabricated sentence");
