import { motion } from "framer-motion";

import { Puzzle } from "lucide-react";

import { fadeRiseVariants } from "../lib/motion";

export function EmptyState({ title, hint, action, onAction }: { title: string; hint: string; action: string; onAction: () => void }) {

  return (

    <div className="empty">

      <motion.div

        className="empty-icon"

        animate={{ y: [0, -4, 0] }}

        transition={{ duration: 3, repeat: Infinity, ease: "easeInOut" }}

      >

        <Puzzle size={28} strokeWidth={1.5} />

      </motion.div>

      <motion.h2 variants={fadeRiseVariants} initial="hidden" animate="visible" custom={0}>

        {title}

      </motion.h2>

      <motion.p variants={fadeRiseVariants} initial="hidden" animate="visible" custom={1}>

        {hint}

      </motion.p>

      <motion.button type="button" whileHover={{ y: -2 }} whileTap={{ scale: 0.97 }} onClick={onAction}>

        {action}

      </motion.button>

    </div>

  );

}
