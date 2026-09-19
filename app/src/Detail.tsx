import { useEffect, useState } from "react";

import { ArrowLeft } from "lucide-react";

import { motion } from "framer-motion";

import { DetailBanner } from "./components/DetailBanner";

import { DetailLanes } from "./components/DetailLanes";

import { DetailReview } from "./components/DetailReview";

import { useGameAssets } from "./hooks/useGameAssets";

import { useGameLaunch } from "./hooks/useGameLaunch";

import { useIsGameInstalled } from "./hooks/useIsGameInstalled";

import { displayTitle } from "./lib/format";

import { detailItemVariants } from "./lib/motion";

import type { DownloadLane } from "./types";

import type { GameDetail } from "./types";

import type { GameEntry } from "./types";

import { liftOnHover, pressDown } from "./lib/motion";

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

  const title = displayTitle(game.title);

  const [videoPlaying, setVideoPlaying] = useState(false);

  const rawAssets = useGameAssets(game.title);

  const loadingArt = rawAssets === undefined;

  const assets = rawAssets ?? null;

  const [logoFailed, setLogoFailed] = useState(false);

  const installed = useIsGameInstalled(game.title);

  const { launching, launch } = useGameLaunch(game.title);

  useEffect(() => {

    setLogoFailed(false);

  }, [game.title]);

  return (

    <motion.div className="detail" initial="hidden" animate="visible" variants={detailItemVariants}>

      <header className="detail-bar">

        <motion.button

          type="button"

          className="back"

          whileHover={liftOnHover}

          whileTap={pressDown}

          onClick={onBack}

        >

          <ArrowLeft size={16} strokeWidth={1.8} />

          Back to Home

        </motion.button>

        <span className="source">Game Detail</span>

      </header>

      <DetailBanner

        game={game}

        detail={detail}

        busy={busy}

        title={title}

        assets={assets}

        loadingArt={loadingArt}

        logoFailed={logoFailed}

        onLogoFailed={() => setLogoFailed(true)}

      />

      <section className="detail-body">

        <DetailLanes

          detail={detail}

          busy={busy}

          onDownload={onDownload}

          onLanePick={onLanePick}

          installed={installed}

          launching={launching}

          onPlay={launch}

        />

        <DetailReview

          videoId={detail?.videoId}

          gameTitle={game.title}

          playing={videoPlaying}

          onPlay={() => setVideoPlaying(true)}

        />

        {detail && !busy && detail.lanes.length === 0 && (

          <p className="state">Could Not Load Download Lanes.</p>

        )}

      </section>

    </motion.div>

  );

}
