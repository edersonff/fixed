import { AnimatePresence, motion } from "framer-motion";

import { stateVariants } from "../lib/motion";

function isError(uninstallMsg: string, launchMsg: string): boolean {

  return !!uninstallMsg || launchMsg.startsWith("Launch Failed") || launchMsg.startsWith("Fix Failed");

}

function noteClass(uninstallMsg: string, launchMsg: string): string {

  if (isError(uninstallMsg, launchMsg)) {

    return "launch-note launch-error";

  }

  if (launchMsg) {

    return "launch-note launch-progress";

  }

  return "launch-note";

}

export function LibraryCardNote({

  launchMsg,

  pluginMsg,

  uninstallMsg,

}: {

  launchMsg: string;

  pluginMsg: string;

  uninstallMsg: string;

}) {

  const text = uninstallMsg || launchMsg || pluginMsg;

  return (

    <AnimatePresence mode="wait" initial={false}>

      {text && (

        <motion.p

          key={text}

          className={noteClass(uninstallMsg, launchMsg)}

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
