import { motion } from "framer-motion";

import { ArtImage } from "./ArtImage";

import { prettyCategory } from "../lib/format";

import type { GameAssets } from "../types";

import type { GameDetail } from "../types";

import type { GameEntry } from "../types";

export function DetailBanner({

  game,

  detail,

  busy,

  title,

  assets,

  loadingArt,

  logoFailed,

  onLogoFailed,

}: {

  game: GameEntry;

  detail: GameDetail | null;

  busy: boolean;

  title: string;

  assets: GameAssets | null;

  loadingArt: boolean;

  logoFailed: boolean;

  onLogoFailed: () => void;

}) {

  const published = game.publishedAt.slice(0, 10);

  return (

    <section className="detail-banner" aria-labelledby="detail-title">

      <ArtImage

        title={title}

        sources={[assets?.heroUrl, game.posterUrl]}

        loading={loadingArt}

        className="detail-backdrop"

        alt=""

      />

      <div className="detail-backdrop-shade" />

      <div className="detail-cover">

        <ArtImage

          title={title}

          sources={[assets?.coverUrl, game.posterUrl]}

          loading={loadingArt}

          className="detail-cover-art"

          alt={`${title} cover`}

        />

        {busy && (

          <motion.div

            className="poster-skeleton"

            animate={{ backgroundPosition: ["200% 0", "-100% 0"] }}

            transition={{ duration: 1.1, repeat: Infinity, ease: "linear" }}

          />

        )}

      </div>

      <div className="info">

        <p className="eyebrow">Game Detail</p>

        {assets?.logoUrl && !logoFailed ? (

          <img

            className="detail-logo"

            src={assets.logoUrl}

            alt={title}

            referrerPolicy="no-referrer"

            onError={onLogoFailed}

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

  );

}
