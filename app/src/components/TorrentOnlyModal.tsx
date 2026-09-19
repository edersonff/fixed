import type { GameDetail } from "../types";

import type { GameEntry } from "../types";

export function TorrentOnlyModal({

  torrentAsk,

  onCancel,

  onConfirm,

}: {

  torrentAsk: { game: GameEntry; detail: GameDetail } | null;

  onCancel: () => void;

  onConfirm: () => void;

}) {

  if (!torrentAsk) {

    return null;

  }

  return (

    <div className="modal-backdrop" onClick={onCancel}>

      <div className="modal-card" role="dialog" aria-modal="true" onClick={(event) => event.stopPropagation()}>

        <h2>Torrent is the only lane</h2>

        <p className="modal-hint">

          {torrentAsk.game.title} has no HTTP mirror right now. The torrent lane works, but some

          ISPs monitor torrent swarms and may flag your connection.

        </p>

        <div className="modal-actions">

          <button type="button" className="ghost" onClick={onCancel}>

            Cancel

          </button>

          <button type="button" className="hero-cta" onClick={onConfirm}>

            Download via torrent

          </button>

        </div>

      </div>

    </div>

  );

}
