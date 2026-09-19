import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { motion } from "framer-motion";

import { FolderOpen, Play, Puzzle } from "lucide-react";

import { ArtImage } from "./ArtImage";

import { useGameAssets } from "../hooks/useGameAssets";

import { useGameLaunch } from "../hooks/useGameLaunch";

import { usePluginInstall } from "../hooks/usePluginInstall";

import { diskSize } from "../lib/format";

import { displayTitle } from "../lib/format";

import { fadeRiseVariants } from "../lib/motion";

import type { InstalledGame } from "../types";

import { liftOnHover, pressDown } from "../lib/motion";

export function LibraryCard({ game }: { game: InstalledGame }) {

  const rawAssets = useGameAssets(game.title);

  const loadingArt = rawAssets === undefined;

  const assets = rawAssets ?? null;

  const [logoFailed, setLogoFailed] = useState(false);

  const { launching, launchMsg, launch } = useGameLaunch(game.title);

  const { pluginMsg, addPlugin } = usePluginInstall(game.title);

  return (

    <motion.article className="lib-card" initial="hidden" animate="visible" variants={fadeRiseVariants}>

      <ArtImage

        title={displayTitle(game.title)}

        sources={[assets?.heroUrl]}

        loading={loadingArt}

        className="lib-art"

        alt=""

      />

      <div className="lib-body">

        <h2>

          {assets?.logoUrl && !logoFailed ? (

            <img

              className="lib-logo"

              src={assets.logoUrl}

              alt={displayTitle(game.title)}

              referrerPolicy="no-referrer"

              onError={() => setLogoFailed(true)}

            />

          ) : (

            displayTitle(game.title)

          )}

        </h2>

        <p className="lib-meta">

          {diskSize(game.bytes)}

          {game.hasPlugins && <span className="lane-badge">Plugins</span>}

        </p>

        <div className="ready-actions">

          <motion.button

            type="button"

            className="play"

            whileHover={liftOnHover}

            whileTap={pressDown}

            disabled={launching}

            onClick={launch}

          >

            <Play size={15} strokeWidth={2.4} />

            {launching ? "Starting" : "Play"}

          </motion.button>

          <button type="button" className="ghost" onClick={addPlugin}>

            <Puzzle size={15} strokeWidth={2} />

            Add Plugin

          </button>

          <button

            type="button"

            className="ghost"

            onClick={() => invoke("open_game_folder", { folder: game.folder })}

          >

            <FolderOpen size={15} strokeWidth={2} />

            Folder

          </button>

        </div>

        {launchMsg && <p className="plugin-note">{launchMsg}</p>}

        {pluginMsg && <p className="plugin-note">{pluginMsg}</p>}

      </div>

    </motion.article>

  );

}
