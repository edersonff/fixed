import { motion } from "framer-motion";

import { Download, Play } from "lucide-react";

import { Wrench } from "lucide-react";

import { LaneIcon } from "./LaneIcon";

import { LoadingDots } from "./LoadingDots";

import { LANE_NOTES } from "../lib/lanes";

import { displayLane } from "../lib/format";

import { detailItemVariants } from "../lib/motion";

import type { DownloadLane } from "../types";

import type { GameDetail } from "../types";

import { actionHover, actionPressDown, liftOnHover } from "../lib/motion";

export function DetailLanes({

  detail,

  busy,

  onDownload,

  onLanePick,

  installed,

  updateAvailable,

  launching,

  onPlay,

}: {

  detail: GameDetail | null;

  busy: boolean;

  onDownload: () => void;

  onLanePick: (lane: DownloadLane) => void;

  installed: boolean;

  updateAvailable: boolean;

  launching: boolean;

  onPlay: () => void;

}) {

  return (

    <>

      <div className="detail-section-head">

        <div>

          <p className="eyebrow">Download Lanes</p>

          <h2>Choose a Source</h2>

        </div>

        {updateAvailable && <span className="chip">Update available</span>}

        <motion.button

          type="button"

          className="hero-cta"

          onClick={installed ? onPlay : onDownload}

          disabled={installed ? launching : !detail || detail.lanes.length === 0}

          whileHover={actionHover}

          whileTap={actionPressDown}

        >

          <span className="action-icon">

            {installed ? <Play size={17} strokeWidth={2.2} /> : <Download size={17} strokeWidth={2.2} />}

          </span>

          {installed ? (launching ? "Starting" : "Play") : "Download"}

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

              whileHover={liftOnHover}

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

    </>

  );

}
