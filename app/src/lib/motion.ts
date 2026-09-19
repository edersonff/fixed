import type { Variants } from "framer-motion";

export const EASE_DECEL = [0.05, 0.7, 0.1, 1] as const;

export const fadeRiseVariants: Variants = {

  hidden: { opacity: 0, filter: "blur(8px)", y: 10 },

  visible: (index = 0) => ({

    opacity: 1,

    filter: "blur(0px)",

    y: 0,

    transition: { duration: 0.3, delay: index * 0.04, ease: EASE_DECEL },

  }),

  exit: {

    opacity: 0,

    filter: "blur(8px)",

    y: 10,

    transition: { duration: 0.15, ease: EASE_DECEL },

  },

};

export const heroVariants: Variants = {

  ...fadeRiseVariants,

  visible: (index = 0) => ({

    opacity: 1,

    filter: "blur(0px)",

    y: 0,

    transition: { duration: 0.3, delay: index * 0.06, ease: EASE_DECEL },

  }),

};

export const viewVariants: Variants = {

  hidden: { opacity: 0, filter: "blur(6px)", scale: 0.995, y: 4 },

  visible: (mode = "view") => ({

    opacity: 1,

    filter: "blur(0px)",

    scale: 1,

    y: 0,

    transition: {

      duration: mode === "detail" ? 0.24 : 0.2,

      ease: EASE_DECEL,

    },

  }),

  exit: (mode = "view") => ({

    opacity: 0,

    filter: mode === "detail" ? "blur(8px)" : "blur(0px)",

    scale: 0.995,

    y: mode === "detail" ? 10 : 0,

    transition: {

      duration: mode === "detail" ? 0.15 : 0.2,

      ease: EASE_DECEL,

    },

  }),

};
