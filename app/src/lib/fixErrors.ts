export const FILES_REMOVED_AGAIN = "files-removed-again";

export const GAME_PAGE_NEEDED = "game-page-needed";

const MESSAGES: Record<string, string> = {

  [FILES_REMOVED_AGAIN]: "Windows keeps removing this game's online files. Allow the games folder once and FIXED fixes it.",

  [GAME_PAGE_NEEDED]: "Open this game's page and press Download to get its files again.",

};

// The only place that turns a backend fix-error code into words: useGameLaunch and FixBlockedNote
// both read through here so the sentence never drifts between the two screens that show it.
export function fixErrorMessage(code: string): string | null {

  return MESSAGES[code] ?? null;

}
