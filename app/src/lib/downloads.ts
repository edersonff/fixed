import { megabytes } from "./format.ts";

import type { DownloadEntry } from "../types";

export type DownloadStartArgs = {

  title: string;

  pageUrl: string;

  laneUrl: string;

  build: string | null;

};

// Both download pipelines (http and torrent) write a `.fixed-source` marker keyed on this exact
// page url once extraction finishes, so a later Fix can redownload from the same page instead of
// guessing by title. A markerless download is a silent future defect, so this throws instead of
// letting an empty pageUrl travel to the backend.
export function downloadStartArgs(gamePageUrl: string, safeTitle: string, laneUrl: string, build: string | null): DownloadStartArgs {

  if (!gamePageUrl) {

    throw new Error(`download started with no page url for ${safeTitle}`);

  }

  return { title: safeTitle, pageUrl: gamePageUrl, laneUrl, build };

}

export function downloadPercent(entry: DownloadEntry): number {

  const total = entry.totalBytes ?? 0;

  const downloaded = entry.downloadedBytes ?? 0;

  return total > 0 ? Math.floor((downloaded / total) * 100) : 0;

}

export function downloadStatusLabel(entry: DownloadEntry): string {

  const downloaded = entry.downloadedBytes ?? 0;

  const total = entry.totalBytes ?? 0;

  const percent = downloadPercent(entry);

  if (entry.state === "resolving") {

    return "Resolving Mirrors";

  }

  if (entry.state === "torrenting") {

    return `${percent}% · ${megabytes(downloaded)} of ${megabytes(total)}`;

  }

  if (entry.state === "extracting") {

    return "Extracting";

  }

  if (entry.state === "ready") {

    return "Ready to Play";

  }

  if (entry.state === "stopped") {

    return "Stopped";

  }

  if (entry.state === "parts") {

    return `${entry.parts.length} File${entry.parts.length === 1 ? "" : "s"} Found. Manual Lane`;

  }

  return entry.errorMsg ?? "Could Not Resolve This Lane";

}
