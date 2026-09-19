import { motion } from "framer-motion";

import { Download } from "lucide-react";

import { Home } from "lucide-react";

import { LibraryBig } from "lucide-react";

import type { View } from "../types";


const NAV: Array<{ id: View; label: string; Icon: typeof Home }> = [

  { id: "home", label: "Home", Icon: Home },

  { id: "library", label: "Library", Icon: LibraryBig },

  { id: "downloads", label: "Downloads", Icon: Download },

];

function Wordmark() {

  return (

    <span className="wordmark">

      FI<span className="x">X</span>ED

    </span>

  );

}

export function Sidebar({

  view,

  hasSelection,

  appVersion,

  onSwitchView,

}: {

  view: View;

  hasSelection: boolean;

  appVersion: string;

  onSwitchView: (target: View) => void;

}) {

  return (

    <aside className="sidebar">

      <Wordmark />

      <nav>

        {NAV.map(({ id, label, Icon }) => (

          <motion.button

            key={id}

            type="button"

            title={label}

            className={view === id && !hasSelection ? "nav-item active press-lift" : "nav-item press-lift"}

            onClick={() => onSwitchView(id)}

          >

            <Icon size={19} strokeWidth={1.8} />

            <span>{label}</span>

          </motion.button>

        ))}

      </nav>

      <span className="foot">v{appVersion || "…"} · every game, ready to play</span>

    </aside>

  );

}
