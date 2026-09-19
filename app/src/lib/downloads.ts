import { megabytes } from "./format";

import type { DownloadEntry } from "../types";

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
