import { useEffect, useState } from "react";

import { AnimatePresence, motion } from "framer-motion";

import { Download, Play } from "lucide-react";

import { ArtImage } from "./ArtImage";

import { useGameAssets } from "../hooks/useGameAssets";

import { useGameLaunch } from "../hooks/useGameLaunch";

import { useIsGameInstalled } from "../hooks/useIsGameInstalled";

import { currentHeroPreview, subscribeHeroPreview } from "../lib/heroPreview";

import { displayTitle } from "../lib/format";

import { formatDate } from "../lib/format";

import { prettyCategory } from "../lib/format";

import { fadeRiseVariants } from "../lib/motion";

import { heroArtVariants } from "../lib/motion";

import { liftOnHover, pressDown } from "../lib/motion";

import { heroVariants } from "../lib/motion";

import type { GameEntry } from "../types";

export function HomeHero({

  featured: defaultFeatured,

  onSelect,

}: {

  featured: GameEntry | null;

  onSelect: (game: GameEntry) => void;

}) {

  const [preview, setPreview] = useState<GameEntry | null>(currentHeroPreview);

  useEffect(() => subscribeHeroPreview(setPreview), []);

  const featured = preview ?? defaultFeatured;

  const rawAssets = useGameAssets(featured?.title ?? "");

  const loadingArt = featured !== null && rawAssets === undefined;

  const assets = rawAssets ?? null;

  const [logoFailed, setLogoFailed] = useState(false);

  const installed = useIsGameInstalled(featured?.title ?? "");

  const { launching, launch } = useGameLaunch(featured?.title ?? "");

  useEffect(() => {

    setLogoFailed(false);

  }, [featured?.title]);

  if (!featured) {

    return null;

  }

  return (

    <motion.section className="hero" initial="hidden" animate="visible" variants={fadeRiseVariants}>

      <AnimatePresence initial={false}>

        <motion.div

          className="hero-bg"

          key={featured.pageUrl}

          variants={heroArtVariants}

          initial="hidden"

          animate="visible"

          exit="exit"

        >

          <ArtImage

            title={displayTitle(featured.title)}

            sources={[assets?.heroUrl, featured.posterUrl]}

            loading={loadingArt}

            className="hero-bg-layer"

            alt=""

          />

        </motion.div>

      </AnimatePresence>

      <div className="hero-copy">

        <motion.p className="eyebrow" variants={heroVariants} initial="hidden" animate="visible" custom={0}>

          Featured

        </motion.p>

        <motion.h1 className="hero-title" variants={heroVariants} initial="hidden" animate="visible" custom={1}>

          {assets?.logoUrl && !logoFailed ? (

            <img

              className="hero-logo-img"

              src={assets.logoUrl}

              alt={displayTitle(featured.title)}

              referrerPolicy="no-referrer"

              onError={() => setLogoFailed(true)}

            />

          ) : (

            displayTitle(featured.title)

          )}

        </motion.h1>

        <motion.p className="hero-meta-line" variants={heroVariants} initial="hidden" animate="visible" custom={2}>

          {prettyCategory(featured.category)} · {formatDate(featured.publishedAt)}

        </motion.p>

        <motion.button

          type="button"

          className="hero-cta"

          variants={heroVariants}

          initial="hidden"

          animate="visible"

          custom={3}

          whileHover={liftOnHover}

          whileTap={pressDown}

          disabled={installed && launching}

          onClick={() => (installed ? launch() : onSelect(featured))}

        >

          {installed ? <Play size={17} strokeWidth={2.2} /> : <Download size={17} strokeWidth={2.2} />}

          {installed ? (launching ? "Starting" : "Play") : "Download"}

        </motion.button>

      </div>

    </motion.section>

  );

}
