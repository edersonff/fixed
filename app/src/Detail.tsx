import { ArrowLeft } from "lucide-react";

import { Cloud } from "lucide-react";

import { Download } from "lucide-react";

import { Globe } from "lucide-react";

import { HardDrive } from "lucide-react";

import { Link2 } from "lucide-react";

import { Play } from "lucide-react";

import { Wrench } from "lucide-react";

import { openUrl } from "@tauri-apps/plugin-opener";

import type { GameDetail } from "./types";

import type { GameEntry } from "./types";

const LANE_ICONS: Record<string, typeof Globe> = {

  hosters: Globe,

  drive: HardDrive,

  direct: Link2,

  torrent: Download,

  mega: Cloud,

  yandex: HardDrive,

  "google-drive": Cloud,

  mirror: Link2,

};

const LANE_NOTES: Record<string, string> = {

  hosters: "Recommended Mirror",

  drive: "Fast Mirror",

  direct: "Site Files",

  mega: "Mega Mirror",

  yandex: "Yandex Disk Mirror",

  "google-drive": "Google Drive Mirror",

  mirror: "Mirror",

  torrent: "P2P · Optional",

};

function displayTitle(raw: string): string {

  return raw

    .trim()

    .split(/\s+/)

    .map((word) => {

      if (/^(?:[A-Za-z]\.){2,}[A-Za-z]?/.test(word)) {

        return word;

      }

      const normalized = word.toLocaleLowerCase();

      return normalized.replace(/^[a-zà-ÿ]/i, (letter) => letter.toLocaleUpperCase());

    })

    .join(" ");

}

function displayLane(raw: string): string {

  return raw

    .replace("google-drive", "Google Drive")

    .replace(/(^|[- ])([a-z])/g, (_, prefix: string, letter: string) => `${prefix}${letter.toUpperCase()}`);

}

function prettyCategory(raw: string): string {

  return raw

    .replace("officialservers", "official servers")

    .replace(/(^|[- ])([a-z])/g, (_, prefix: string, letter: string) => `${prefix}${letter.toUpperCase()}`);

}

function LaneIcon({ kind }: { kind: string }) {

  const Icon = LANE_ICONS[kind] ?? Link2;

  return <Icon size={17} strokeWidth={1.8} />;

}

export function DetailView({

  game,

  detail,

  busy,

  onBack,

  onDownload,

}: {

  game: GameEntry;

  detail: GameDetail | null;

  busy: boolean;

  onBack: () => void;

  onDownload: () => void;

}) {

  const published = game.publishedAt.slice(0, 10);

  const title = displayTitle(game.title);

  return (

    <div className="detail">

      <header className="detail-bar">

        <button type="button" className="back" onClick={onBack}>

          <ArrowLeft size={16} strokeWidth={1.8} />

          Back to Home

        </button>

        <span className="source">Game Detail</span>

      </header>

      <section className="detail-banner" aria-labelledby="detail-title">

        <div className="poster detail-art">

          <img src={game.posterUrl} alt={`${title} poster`} referrerPolicy="no-referrer" />

        </div>

        <div className="detail-banner-shade" />

        <div className="info">

          <p className="eyebrow">Game Detail</p>

          <h1 id="detail-title">{title}</h1>

          <div className="meta">

            <span className="chip">{prettyCategory(game.category)}</span>

            <span className="chip">Published {published}</span>

            <span className="views-chip">{game.views.toLocaleString("en-US")} Views</span>

          </div>

          {detail && detail.build && <p className="build">Build {detail.build}</p>}

        </div>

      </section>

      <section className="detail-body">

        <div className="detail-section-head">

          <div>

            <p className="eyebrow">Download Lanes</p>

            <h2>Choose a Source</h2>

          </div>

          <button

            type="button"

            className="hero-cta"

            onClick={onDownload}

            disabled={!detail || detail.lanes.length === 0}

          >

            <Download size={17} strokeWidth={2.2} />

            Download

          </button>

        </div>

        {busy && <p className="state">Loading Build Info...</p>}

        {detail && detail.lanes.length > 0 && (

          <div className="lanes">

            {detail.lanes.map((lane) => (

              <div key={lane.kind} className={lane.kind === "torrent" ? "lane-row torrent" : "lane-row"}>

                <LaneIcon kind={lane.kind} />

                <span className="name">{displayLane(lane.kind)}</span>

                <span className="kind">{LANE_NOTES[lane.kind] ?? "Available Source"}</span>

                <span className="spacer">Available</span>

                {lane.kind === "torrent" && (

                  <p className="warning">Some ISPs Monitor Torrent Swarms. Mirrors Are Safer Where P2P Is Watched.</p>

                )}

              </div>

            ))}

          </div>

        )}

        {detail && detail.mentionsFixRepair && (

          <p className="note">

            <Wrench size={13} strokeWidth={1.8} />

            Dead Links? Fix Repair Is Included Automatically.

          </p>

        )}

        {detail?.videoId && (

          <button

            type="button"

            className="review-card"

            onClick={() => {

              openUrl(`https://www.youtube.com/watch?v=${detail.videoId}`).catch((reason: unknown) =>

                console.error("open review failed:", reason),

              );

            }}

          >

            <span className="review-thumb">

              <img

                src={`https://i.ytimg.com/vi/${detail.videoId}/hqdefault.jpg`}

                alt={`${game.title} review thumbnail`}

                referrerPolicy="no-referrer"

                loading="lazy"

              />

              <Play size={22} strokeWidth={2} />

            </span>

            <span className="review-copy">Watch the Video Review</span>

          </button>

        )}

        {detail && !busy && detail.lanes.length === 0 && (

          <p className="state">Could Not Load Download Lanes.</p>

        )}

      </section>

    </div>

  );

}
