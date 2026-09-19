import { AnimatePresence, motion } from "framer-motion";

import { stateVariants } from "../lib/motion";

function noteClass(uninstallMsg: string, launchMsg: string, phase: string): string {

  if (uninstallMsg || phase === "failed") {

    return "launch-note launch-error";

  }

  if (launchMsg && phase !== "running") {

    return "launch-note launch-progress";

  }

  return "launch-note";

}

function noteKey(uninstallMsg: string, launchMsg: string, pluginMsg: string, phase: string): string {

  if (uninstallMsg) {

    return "uninstall";

  }

  if (launchMsg) {

    return `launch-${phase}`;

  }

  if (pluginMsg) {

    return "plugin";

  }

  return "none";

}

export function LibraryCardNote({

  launchMsg,

  pluginMsg,

  uninstallMsg,

  phase,

}: {

  launchMsg: string;

  pluginMsg: string;

  uninstallMsg: string;

  phase: string;

}) {

  const text = uninstallMsg || launchMsg || pluginMsg;

  return (

    <AnimatePresence mode="wait" initial={false}>

      {text && (

        <motion.p

          key={noteKey(uninstallMsg, launchMsg, pluginMsg, phase)}

          className={noteClass(uninstallMsg, launchMsg, phase)}

          variants={stateVariants}

          initial="hidden"

          animate="visible"

          exit="exit"

        >

          {text}

        </motion.p>

      )}

    </AnimatePresence>

  );

}
