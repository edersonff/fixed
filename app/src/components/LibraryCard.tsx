import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { AnimatePresence, motion } from "framer-motion";

import { Trash2 } from "lucide-react";

import { ArtImage } from "./ArtImage";

import { LibraryCardActions } from "./LibraryCardActions";

import { LibraryCardNote } from "./LibraryCardNote";

import { LibraryCardUninstallConfirm } from "./LibraryCardUninstallConfirm";

import { useGameAssets } from "../hooks/useGameAssets";

import { useGameLaunch } from "../hooks/useGameLaunch";

import { useGameUninstall } from "../hooks/useGameUninstall";

import { usePluginInstall } from "../hooks/usePluginInstall";

import { diskSize, displayTitle } from "../lib/format";

import { fadeRiseVariants } from "../lib/motion";

import { openRepair } from "../lib/repairStore";

import type { InstalledGame } from "../types";

export function LibraryCard({

  game,

  index,

  onOpenDetail,

  onUninstalled,

}: {

  game: InstalledGame;

  index: number;

  onOpenDetail: (game: InstalledGame) => void;

  onUninstalled: () => void;

}) {

  const rawAssets = useGameAssets(game.title);

  const loadingArt = rawAssets === undefined;

  const assets = rawAssets ?? null;

  const [logoFailed, setLogoFailed] = useState(false);

  const { launching, launchMsg, phase, launch } = useGameLaunch(game.title);

  const { pluginMsg, addPlugin } = usePluginInstall(game.title);

  const { confirming, setConfirming, uninstalling, uninstallMsg, setUninstallMsg, handleUninstall } =

    useGameUninstall(game.title, onUninstalled);

  return (

    <motion.div

      className="lib-card-reveal"

      initial="hidden"

      animate="visible"

      variants={fadeRiseVariants}

      custom={index}

    >

      <article

        className="lib-card"

        role="button"

        tabIndex={0}

        aria-label={`Open details for ${displayTitle(game.title)}`}

        onClick={() => onOpenDetail(game)}

        onKeyDown={(event) => {

          if (event.key === "Enter" || event.key === " ") {

            event.preventDefault();

            onOpenDetail(game);

          }

        }}

      >

        <ArtImage

          title={displayTitle(game.title)}

          sources={[assets?.coverUrl, assets?.heroUrl]}

          loading={loadingArt}

          className="lib-art"

          alt={`${displayTitle(game.title)} cover`}

        />

        <div className="lib-body">

          <div className="lib-topline">

            <span className="lib-label">Installed</span>

            <motion.button

              type="button"

              className="press-lift lib-uninstall"

              aria-label={`Uninstall ${displayTitle(game.title)}`}


              onClick={(event) => {

                event.stopPropagation();

                setConfirming((value) => !value);

                setUninstallMsg("");

              }}

            >

              <Trash2 size={15} strokeWidth={2} />

            </motion.button>

          </div>

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

            {game.build && <span className="lane-badge">v{game.build}</span>}

            {game.hasPlugins && <span className="lane-badge">Plugins</span>}

          </p>

          <LibraryCardActions

            launching={launching}

            phase={phase}

            needsRepair={game.missingFiles > 0}

            onLaunch={(event) => {

              event.stopPropagation();

              if (game.missingFiles > 0) {

                openRepair(game.title);

                return;

              }

              launch();

            }}

            onAddPlugin={(event) => {

              event.stopPropagation();

              addPlugin();

            }}

            onOpenFolder={(event) => {

              event.stopPropagation();

              invoke("open_game_folder", { folder: game.folder });

            }}

          />

          <LibraryCardNote launchMsg={launchMsg} pluginMsg={pluginMsg} uninstallMsg={uninstallMsg} phase={phase} />

          <AnimatePresence>

            {confirming && (

              <LibraryCardUninstallConfirm

                key="uninstall-confirm"

                uninstalling={uninstalling}

                onKeep={() => setConfirming(false)}

                onRemove={handleUninstall}

              />

            )}

          </AnimatePresence>

        </div>

      </article>

    </motion.div>

  );

}
