import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { open } from "@tauri-apps/plugin-dialog";

import { PackagePlus } from "lucide-react";

import { Play } from "lucide-react";

import { Puzzle } from "lucide-react";

import type { DownloadEntry } from "./types";

function megabytes(bytes: number): string {

  return `${(bytes / 1_000_000).toFixed(1)} MB`;

}

function EntryCard({ entry }: { entry: DownloadEntry }) {

  const [launching, setLaunching] = useState(false);

  const [pluginMsg, setPluginMsg] = useState("");

  async function addPlugin() {

    const selected = await open({

      multiple: false,

      filters: [{ name: "Plugin Package", extensions: ["zip", "rar", "dll"] }],

    });

    if (!selected || typeof selected !== "string") {

      return;

    }

    invoke<number>("install_plugin", { title: entry.game.title, archivePath: selected })

      .then((count) => setPluginMsg(`Plugin Installed. ${count} File${count === 1 ? "" : "s"}`))

      .catch((reason: unknown) => setPluginMsg(`Plugin Failed: ${String(reason)}`));

  }

  const total = entry.totalBytes ?? 0;

  const downloaded = entry.downloadedBytes ?? 0;

  const percent = total > 0 ? Math.floor((downloaded / total) * 100) : 0;

  const status =

    entry.state === "resolving"

      ? "Resolving Mirrors…"

      : entry.state === "torrenting"

        ? `${percent}% · ${megabytes(downloaded)} of ${megabytes(total)}`

        : entry.state === "extracting"

          ? "Extracting…"

          : entry.state === "ready"

            ? "Ready to Play"

            : entry.state === "stopped"

              ? "Stopped"

              : entry.state === "parts"

                ? `${entry.parts.length} File${entry.parts.length === 1 ? "" : "s"} Found. Manual Lane`

                : "Could Not Resolve This Lane";

  return (

    <article className="queue-card">

      <div className="poster small">

        <img src={entry.game.posterUrl} alt={`${entry.game.title} poster`} referrerPolicy="no-referrer" />

      </div>

      <div className="queue-copy">

        <h2>

          {entry.game.title}

          {entry.state === "torrenting" && (

            <span className="lane-badge">{entry.lane === "http" ? "HTTP · mirror" : "P2P · torrent"}</span>

          )}

        </h2>

        <p>{status}</p>

        {(entry.state === "torrenting" || entry.state === "extracting") && (

          <div className="progress-track">

            <div className="progress-fill" style={{ width: `${percent}%` }} />

          </div>

        )}

        {entry.state === "parts" && entry.parts.length > 0 && (

          <ul className="parts">

            {entry.parts.slice(0, 4).map((part) => (

              <li key={part}>{part.split("/").pop()}</li>

            ))}

          </ul>

        )}

        {entry.state === "ready" && (

          <div className="ready-actions">

            <button

              type="button"

              className="play"

              disabled={launching}

              onClick={() => {

                setLaunching(true);

                invoke<string>("launch_game", { title: entry.game.title })

                  .catch((reason: unknown) => console.error("launch failed:", reason))

                  .finally(() => {

                    setTimeout(() => setLaunching(false), 400);

                  });

              }}

            >

              <Play size={14} strokeWidth={2.2} />

              {launching ? "Launching…" : "Play"}

            </button>

            <button type="button" className="ghost" onClick={addPlugin}>

              <PackagePlus size={14} strokeWidth={2} />

              Add Plugin

            </button>

          </div>

        )}

        {pluginMsg && <p className="plugin-note">{pluginMsg}</p>}

      </div>

    </article>

  );

}

export function DownloadsView({

  entries,

  onBrowse,

  onStopAll,

}: {

  entries: DownloadEntry[];

  onBrowse: () => void;

  onStopAll: () => void;

}) {

  if (entries.length === 0) {

    return (

      <div className="empty">

        <Puzzle size={28} strokeWidth={1.5} />

        <h2>No Active Downloads</h2>

        <p>Pick a Game and FIXED Handles Mirrors, Extraction and Setup for You.</p>

        <button type="button" onClick={onBrowse}>

          Find a Game

        </button>

      </div>

    );

  }

  const hasActive = entries.some(

    (entry) => entry.state === "torrenting" || entry.state === "extracting",

  );

  return (

    <div>

      <header className="bar downloads-bar">

        <div className="bar-title">

          <p className="eyebrow">Active Queue</p>

          <h1>Downloads</h1>

        </div>

        {hasActive && (

          <button type="button" className="ghost" onClick={onStopAll}>

            Stop All

          </button>

        )}

      </header>

      <div className="queue">

        {entries.map((entry) => (

          <EntryCard entry={entry} key={entry.game.pageUrl} />

        ))}

      </div>

    </div>

  );

}
