import { useState } from "react";

import { motion } from "framer-motion";

import { Play } from "lucide-react";

import { liftOnHover } from "../lib/motion";

export function DetailReview({

  videoId,

  gameTitle,

  playing,

  onPlay,

}: {

  videoId: string | undefined;

  gameTitle: string;

  playing: boolean;

  onPlay: () => void;

}) {

  const [thumbFailed, setThumbFailed] = useState(false);

  if (!videoId) {

    return null;

  }

  if (!playing) {

    return (

      <motion.button

        type="button"

        className="review-card"

        aria-label="Play the video review"

        whileHover={liftOnHover}

        whileTap={{ scale: 0.99 }}

        onClick={onPlay}

      >

        <span className="review-thumb">

          {!thumbFailed && (

          <img

            src={`https://i.ytimg.com/vi/${videoId}/hqdefault.jpg`}

            alt={`${gameTitle} review thumbnail`}

            referrerPolicy="no-referrer"

            onError={() => setThumbFailed(true)}

            loading="lazy"

          />

          )}

          <Play size={22} strokeWidth={2} />

        </span>

        <span className="review-copy">Watch the Video Review</span>

      </motion.button>

    );

  }

  return (

    <motion.div

      className="review-card review-player"

      initial={{ opacity: 0, filter: "blur(8px)" }}

      animate={{ opacity: 1, filter: "blur(0px)" }}

      transition={{ duration: 0.2, ease: [0.05, 0.7, 0.1, 1] }}

    >

      <iframe

        src={`https://www.youtube-nocookie.com/embed/${videoId}?autoplay=1&rel=0`}

        title={`${gameTitle} video review`}

        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"

        allowFullScreen

      />

    </motion.div>

  );

}
