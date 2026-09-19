import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { ArrowLeft } from "lucide-react";

import { Cloud } from "lucide-react";

import { Download } from "lucide-react";

import { Globe } from "lucide-react";

import { HardDrive } from "lucide-react";

import { Link2 } from "lucide-react";

import { Play } from "lucide-react";

import { Wrench } from "lucide-react";

import { motion, type Variants } from "framer-motion";

import type { DownloadLane } from "./types";

import type { GameAssets } from "./types";

import type { GameDetail } from "./types";

import type { GameEntry } from "./types";

const detailItemVariants: Variants = {

  hidden: { opacity: 0, filter: "blur(8px)", y: 10 },

  visible: (index = 0) => ({

    opacity: 1,

    filter: "blur(0px)",

    y: 0,

    transition: { duration: 0.24, delay: index * 0.06, ease: [0.05, 0.7, 0.1, 1] },

  }),

};

function LoadingDots({ label = "Loading" }: { label?: string }) {

  return (

    <span className="loading-status" aria-label={label}>

      <span className="sr-only">{label}</span>

      <span className="loading-dots" aria-hidden="true">

        {[0, 1, 2].map((index) => (

          <motion.span

            key={index}

            animate={{ opacity: [0.25, 1, 0.25], y: [0, -3, 0] }}

            transition={{ duration: 0.4, delay: index * 0.12, repeat: Infinity, ease: "easeInOut" }}

          />

        ))}

      </span>

    </span>

  );

}

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

  onLanePick,

}: {

  game: GameEntry;

  detail: GameDetail | null;

  busy: boolean;

  onBack: () => void;

  onDownload: () => void;

  onLanePick: (lane: DownloadLane) => void;

}) {

  const published = game.publishedAt.slice(0, 10);

  const title = displayTitle(game.title);

  const [videoPlaying, setVideoPlaying] = useState(false);

  const [assets, setAssets] = useState<GameAssets | null>(null);

  const [logoFailed, setLogoFailed] = useState(false);

  useEffect(() => {

    setAssets(null);

    setLogoFailed(false);

    invoke<GameAssets | null>("game_assets", { title: game.title })

      .then((result) => setAssets(result))

      .catch(() => setAssets(null));

  }, [game.title]);

  return (

    <motion.div className="detail" initial="hidden" animate="visible" variants={detailItemVariants}>

      <header className="detail-bar">

        <motion.button

          type="button"

          className="back"

          whileHover={{ y: -2 }}

          whileTap={{ scale: 0.97 }}

          onClick={onBack}

        >

          <ArrowLeft size={16} strokeWidth={1.8} />

          Back to Home

        </motion.button>

        <span className="source">Game Detail</span>

      </header>

      <section className="detail-banner" aria-labelledby="detail-title">

        <div className="poster detail-art">

          <img

            src={assets?.heroUrl || game.posterUrl}

            alt={`${title} banner`}

            referrerPolicy="no-referrer"

            onError={(event) => {

              if (assets?.heroUrl) {

                event.currentTarget.src = game.posterUrl;

              }

            }}

          />

          {busy && (

            <motion.div

              className="poster-skeleton"

              animate={{ backgroundPosition: ["200% 0", "-100% 0"] }}

              transition={{ duration: 1.1, repeat: Infinity, ease: "linear" }}

            />

          )}

        </div>

        <div className="detail-banner-shade" />

        <div className="info">

          <p className="eyebrow">Game Detail</p>

          {assets?.logoUrl && !logoFailed ? (

            <img

              className="detail-logo"

              src={assets.logoUrl}

              alt={title}

              referrerPolicy="no-referrer"

              onError={() => setLogoFailed(true)}

            />

          ) : (

            <h1 id="detail-title">{title}</h1>

          )}

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

          <motion.button

            type="button"

            className="hero-cta"

            onClick={onDownload}

            disabled={!detail || detail.lanes.length === 0}

            whileHover={{ y: -2 }}

            whileTap={{ scale: 0.97 }}

          >

            <Download size={17} strokeWidth={2.2} />

            Download

          </motion.button>

        </div>

        {busy && (

          <motion.p className="state" initial="hidden" animate="visible" variants={detailItemVariants}>

            Loading Build Info <LoadingDots label="Loading build information" />

          </motion.p>

        )}

        {detail && detail.lanes.length > 0 && (

          <motion.div className="lanes" initial="hidden" animate="visible" variants={detailItemVariants}>

            {detail.lanes.map((lane, index) => (

              <motion.button

                type="button"

                key={lane.kind}

                className={lane.kind === "torrent" ? "lane-row torrent" : "lane-row"}

                variants={detailItemVariants}

                initial="hidden"

                animate="visible"

                custom={index}

                whileHover={{ y: -2 }}

                whileTap={{ scale: 0.99 }}

                onClick={() => onLanePick(lane)}

              >

                <LaneIcon kind={lane.kind} />

                <span className="name">{displayLane(lane.kind)}</span>

                <span className="kind">{LANE_NOTES[lane.kind] ?? "Available Source"}</span>

                <span className="spacer">Available</span>

                {lane.kind === "torrent" && (

                  <p className="warning">Some ISPs Monitor Torrent Swarms. Mirrors Are Safer Where P2P Is Watched.</p>

                )}

              </motion.button>

            ))}

          </motion.div>

        )}

        {detail && detail.mentionsFixRepair && (

          <p className="note">

            <Wrench size={13} strokeWidth={1.8} />

            Dead Links? Fix Repair Is Included Automatically.

          </p>

        )}

        {detail?.videoId && !videoPlaying && (

          <motion.button

            type="button"

            className="review-card"

            aria-label="Play the video review"

            whileHover={{ y: -2 }}

            whileTap={{ scale: 0.99 }}

            onClick={() => {

              setVideoPlaying(true);

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

          </motion.button>

        )}

        {detail?.videoId && videoPlaying && (

          <motion.div

            className="review-card review-player"

            initial={{ opacity: 0, filter: "blur(8px)" }}

            animate={{ opacity: 1, filter: "blur(0px)" }}

            transition={{ duration: 0.2, ease: [0.05, 0.7, 0.1, 1] }}

          >

            <iframe

              src={`https://www.youtube-nocookie.com/embed/${detail.videoId}?autoplay=1&rel=0`}

              title={`${game.title} video review`}

              allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"

              allowFullScreen

            />

          </motion.div>

        )}

        {detail && !busy && detail.lanes.length === 0 && (

          <p className="state">Could Not Load Download Lanes.</p>

        )}

      </section>

    </motion.div>

  );

}
