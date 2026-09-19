import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { motion } from "framer-motion";

import { FolderOpen, Play, Puzzle, Trash2 } from "lucide-react";

import { ArtImage } from "./ArtImage";

import { useGameAssets } from "../hooks/useGameAssets";

import { useGameLaunch } from "../hooks/useGameLaunch";

import { usePluginInstall } from "../hooks/usePluginInstall";

import { diskSize, displayTitle } from "../lib/format";

import { actionHover, actionPressDown, fadeRiseVariants, liftOnHover, pressDown } from "../lib/motion";

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

  const [confirming, setConfirming] = useState(false);

  const [uninstalling, setUninstalling] = useState(false);

  const [uninstallMsg, setUninstallMsg] = useState("");

  const { launching, launchMsg, launch } = useGameLaunch(game.title);

  const { pluginMsg, addPlugin } = usePluginInstall(game.title);

  function handleUninstall() {

    setUninstalling(true);

    setUninstallMsg("");

    invoke<string>("uninstall_game", { title: game.title, removeFromSteam: true })

      .then(() => onUninstalled())

      .catch((reason: unknown) => setUninstallMsg(`Could not uninstall: ${String(reason)}`))

      .finally(() => setUninstalling(false));

  }

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

              className="lib-uninstall"

              aria-label={`Uninstall ${displayTitle(game.title)}`}

              whileHover={liftOnHover}

              whileTap={pressDown}

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

            {game.hasPlugins && <span className="lane-badge">Plugins</span>}

          </p>

          <div className="ready-actions">

            <motion.button

              type="button"

              className="play action-button"

              whileHover={actionHover}

              whileTap={actionPressDown}

              disabled={launching}

              onClick={(event) => {

                event.stopPropagation();

                launch();

              }}

            >

              <span className="action-icon">

                <Play size={16} strokeWidth={2.4} />

              </span>

              {launching ? "Starting" : "Play"}

            </motion.button>

            <button type="button" className="ghost" onClick={(event) => { event.stopPropagation(); addPlugin(); }}>

              <Puzzle size={15} strokeWidth={2} />

              Add Plugin

            </button>

            <button

              type="button"

              className="ghost"

              onClick={(event) => {

                event.stopPropagation();

                invoke("open_game_folder", { folder: game.folder });

              }}

            >

              <FolderOpen size={15} strokeWidth={2} />

              Folder

            </button>

          </div>

          {(launchMsg || pluginMsg || uninstallMsg) && (

            <p

              className={

                uninstallMsg || launchMsg.startsWith("Launch Failed") ? "launch-note launch-error" : "launch-note"

              }

            >

              {uninstallMsg || launchMsg || pluginMsg}

            </p>

          )}

          {confirming && (

            <div className="uninstall-confirm" role="group" onClick={(event) => event.stopPropagation()}>

              <p>Remove this game from your library?</p>

              <div>

                <button type="button" className="ghost" onClick={() => setConfirming(false)} disabled={uninstalling}>

                  Keep

                </button>

                <button type="button" className="danger-button" onClick={handleUninstall} disabled={uninstalling}>

                  {uninstalling ? "Removing" : "Remove"}

                </button>

              </div>

            </div>

          )}

        </div>

      </article>

    </motion.div>

  );

}
