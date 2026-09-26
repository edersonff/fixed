export type GameEntry = {

  title: string;

  pageUrl: string;

  posterUrl: string;

  category: string;

  publishedAt: string;

  views: number;

  comments: number;

};

export type GamesPage = {

  source: string;

  games: GameEntry[];

};

export type DownloadLane = {

  kind: string;

  url: string;

};

export type GameDetail = {

  title: string;

  build: string;

  steamExtUrl: string;

  lanes: DownloadLane[];

  mentionsFixRepair: boolean;

  videoId?: string;

};

export type GameAssets = {

  appid: number;

  heroUrl: string;

  logoUrl: string;

  coverUrl?: string;

};

export type DownloadState = "resolving" | "parts" | "error" | "torrenting" | "stopped" | "extracting" | "ready";

export type DownloadEntry = {

  game: GameEntry;

  state: DownloadState;

  lane?: string;

  parts: string[];

  downloadedBytes?: number;

  totalBytes?: number;

  errorMsg?: string;

};

export type ProgressPayload = {

  title: string;

  downloadedBytes: number;

  totalBytes: number;

  state: string;

};

export type LaunchProgressPayload = {

  title: string;

  phase: string;

  detail: string;

};

export type InstalledGame = {

  title: string;

  folder: string;

  exe: string;

  bytes: number;

  hasPlugins: boolean;

  build?: string | null;

  missingFiles: number;

  canRestore: boolean;

  downloadBytes: number;

};

export type View = "home" | "library" | "downloads";

export type GameFilesStatus = {

  missing: string[];

  canRestore: boolean;

};

export type RestoreOutcome = {

  missing: string[];

  removedAgain: boolean;

  canRestore: boolean;

};
