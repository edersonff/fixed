import { motion } from "framer-motion";

import { stateVariants } from "../lib/motion";

export function LibraryCardUninstallConfirm({

  uninstalling,

  onKeep,

  onRemove,

}: {

  uninstalling: boolean;

  onKeep: () => void;

  onRemove: () => void;

}) {

  return (

    <motion.div

      className="uninstall-confirm"

      role="group"

      variants={stateVariants}

      initial="hidden"

      animate="visible"

      exit="exit"

      onClick={(event) => event.stopPropagation()}

    >

      <p>Remove this game from your library?</p>

      <div>

        <button type="button" className="ghost" onClick={onKeep} disabled={uninstalling}>

          Keep

        </button>

        <button type="button" className="danger-button" onClick={onRemove} disabled={uninstalling}>

          {uninstalling ? "Removing" : "Remove"}

        </button>

      </div>

    </motion.div>

  );

}
