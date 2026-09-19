import type { Transition, Variants } from "framer-motion";

export const EASE_STANDARD = [0.2, 0, 0, 1] as const;

export const EASE_DECEL = [0.05, 0.7, 0.1, 1] as const;

export const EASE_ACCEL = [0.3, 0, 0.8, 0.15] as const;

export const EASE_POP = [0.2, 1.6, 0.4, 1] as const;

export const DUR_MICRO = 0.1;

export const DUR_SHORT = 0.2;

export const DUR_MED = 0.3;

export const DUR_LONG = 0.5;

export const ROW_STAGGER = 0.05;

export const microTransition: Transition = { duration: DUR_MICRO, ease: EASE_STANDARD };

export const liftOnHover = { y: -2, transition: microTransition } as const;

export const pressDown = { scale: 0.97, transition: { duration: 0.06, ease: EASE_STANDARD } } as const;

export const actionHover = {

  scale: 1.04,

  boxShadow: "0 0 0 1px rgba(198, 255, 74, .12), 0 12px 30px rgba(198, 255, 74, .2)",

  transition: microTransition,

} as const;

export const actionPressDown = { scale: 0.96, transition: { duration: 0.06, ease: EASE_STANDARD } } as const;

const enter = (delay: number): Transition => ({ duration: DUR_MED, delay, ease: EASE_DECEL });

const leave = (duration: number): Transition => ({ duration, ease: EASE_ACCEL });

export const fadeRiseVariants: Variants = {

  hidden: { opacity: 0, filter: "blur(8px)", y: 10 },

  visible: (index = 0) => ({

    opacity: 1,

    filter: "blur(0px)",

    y: 0,

    transition: enter(index * ROW_STAGGER),

  }),

  exit: {

    opacity: 0,

    filter: "blur(8px)",

    y: 10,

    transition: leave(DUR_SHORT),

  },

};

export const heroVariants: Variants = {

  ...fadeRiseVariants,

  visible: (index = 0) => ({

    opacity: 1,

    filter: "blur(0px)",

    y: 0,

    transition: enter(index * ROW_STAGGER),

  }),

};

export const HERO_DRIFT_SECONDS = 20;

export const HERO_DRIFT_SCALE = 1.07;

// Owner ruling 2026-09-19 ("zoom in tipo no banner lentamente"): the hero art drifts while it is
// on screen. It overrides the no-ambient-motion clause in design/motion.md for this surface only.
export const heroArtVariants: Variants = {

  hidden: { opacity: 0, scale: 1 },

  visible: {

    opacity: 1,

    scale: HERO_DRIFT_SCALE,

    transition: {

      opacity: { duration: DUR_LONG, ease: EASE_DECEL },

      scale: { duration: HERO_DRIFT_SECONDS, ease: "linear" },

    },

  },

  exit: { opacity: 0, transition: leave(DUR_MED) },

};

export const detailItemVariants: Variants = {

  hidden: { opacity: 0, filter: "blur(8px)", y: 10 },

  visible: (index = 0) => ({

    opacity: 1,

    filter: "blur(0px)",

    y: 0,

    transition: enter(index * ROW_STAGGER),

  }),

};

export const stateVariants: Variants = {

  hidden: { opacity: 0, filter: "blur(6px)", y: 6 },

  visible: { opacity: 1, filter: "blur(0px)", y: 0, transition: { duration: DUR_SHORT, ease: EASE_DECEL } },

  exit: { opacity: 0, filter: "blur(6px)", y: -4, transition: leave(DUR_MICRO) },

};

export const queueVariants: Variants = {

  hidden: { opacity: 0, filter: "blur(8px)", y: 10 },

  visible: (index = 0) => ({

    opacity: 1,

    filter: "blur(0px)",

    y: 0,

    transition: enter(index * ROW_STAGGER),

  }),

  exit: { opacity: 0, filter: "blur(8px)", y: 10, transition: leave(DUR_SHORT) },

};

export const viewVariants: Variants = {

  hidden: { opacity: 0, filter: "blur(6px)", scale: 0.995, y: 4 },

  visible: (mode = "view") => ({

    opacity: 1,

    filter: "blur(0px)",

    scale: 1,

    y: 0,

    transition: { duration: mode === "detail" ? DUR_MED : DUR_SHORT, ease: EASE_DECEL },

  }),

  exit: (mode = "view") => ({

    opacity: 0,

    filter: mode === "detail" ? "blur(8px)" : "blur(0px)",

    scale: 0.995,

    y: mode === "detail" ? 10 : 0,

    transition: leave(mode === "detail" ? DUR_SHORT : DUR_MICRO),

  }),

};
