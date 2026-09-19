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

}: {

  games: InstalledGame[];

  scanning: boolean;

  error: string | null;

  refresh: () => void;

  onBrowse: () => void;

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

    <section className="rail-block">

      <div className="rail-head">

        <h2>Your Games</h2>

        <button type="button" className="seeall" onClick={refresh} disabled={scanning}>

          <RefreshCw size={14} strokeWidth={2} />

          Rescan

        </button>

      </div>

      <div className="lib-grid">

        {games.map((game) => (

          <LibraryCard key={game.folder} game={game} />

        ))}

      </div>

    </section>

  );

}
