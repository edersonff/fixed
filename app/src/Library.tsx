import { RefreshCw } from "lucide-react";

import { EmptyState } from "./components/EmptyState";

import { LibraryCard } from "./components/LibraryCard";

import { LoadingDots } from "./components/LoadingDots";

import type { InstalledGame } from "./types";

export function LibraryView({

  games,

  scanning,

  error,

  refresh,

  onBrowse,

  onOpenDetail,

  onUninstalled,

}: {

  games: InstalledGame[];

  scanning: boolean;

  error: string | null;

  refresh: () => void;

  onBrowse: () => void;

  onOpenDetail: (game: InstalledGame) => void;

  onUninstalled: () => void;

}) {

  if (error) {

    return <p className="state">Failed to Read Your Games Folder: {error}</p>;

  }

  if (!games.length) {

    return scanning ? (

      <p className="state">

        Scanning Your Games Folder <LoadingDots />

      </p>

    ) : (

      <EmptyState

        title="Nothing Here Yet"

        hint="Games You Install Land on This Shelf, Ready to Play With Zero Setup."

        action="Browse Games"

        onAction={onBrowse}

      />

    );

  }

  return (

    <section className="library-shelf">

      <header className="library-head">

        <div>

          <p className="eyebrow">Your collection</p>

          <h1>Library</h1>

        </div>

        <div className="library-head-actions">

          <span className="library-count">{games.length} {games.length === 1 ? "game" : "games"}</span>

          <button type="button" className="seeall" onClick={refresh} disabled={scanning}>

            <RefreshCw size={14} strokeWidth={2} />

            Rescan

          </button>

        </div>

      </header>

      <div className="lib-grid">

        {games.map((game, index) => (

          <LibraryCard

            key={game.folder}

            game={game}

            index={index}

            onOpenDetail={onOpenDetail}

            onUninstalled={onUninstalled}

          />

        ))}

      </div>

    </section>

  );

}
