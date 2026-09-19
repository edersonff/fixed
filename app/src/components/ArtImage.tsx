import { useEffect, useMemo, useState } from "react";

import { ArtFallback } from "./ArtFallback";

import { LoadingDots } from "./LoadingDots";

import { needsBlurBackdrop } from "../lib/artQuality";

type ArtFit = {
  blurred: boolean;
  width: number;
  height: number;
};

type ArtImageProps = {
  title: string;
  sources: Array<string | null | undefined>;
  loading: boolean;
  className: string;
  alt: string;
};

const COVER_FIT: ArtFit = { blurred: false, width: 0, height: 0 };

export function ArtImage({ title, sources, loading, className, alt }: ArtImageProps) {

  const candidates = useMemo(() => sources.filter((src): src is string => Boolean(src)), [sources]);

  const key = candidates.join("|");

  const [attempt, setAttempt] = useState(0);

  const [fit, setFit] = useState<ArtFit>(COVER_FIT);

  useEffect(() => {

    setAttempt(0);

    setFit(COVER_FIT);

  }, [key]);

  if (loading) {

    return (

      <div className="art-loading">

        <LoadingDots label={`Loading ${title} Art`} />

      </div>

    );

  }

  const src = candidates[attempt];

  if (!src) {

    return <ArtFallback title={title} />;

  }

  const measure = (event: React.SyntheticEvent<HTMLImageElement>) => {

    const image = event.currentTarget;

    setFit({

      blurred: needsBlurBackdrop(image.naturalWidth, image.naturalHeight, image.clientWidth, image.clientHeight),

      width: image.naturalWidth,

      height: image.naturalHeight,

    });

  };

  return (

    <div className={className}>

      {fit.blurred && <img className="art-backdrop" src={src} alt="" aria-hidden="true" referrerPolicy="no-referrer" />}

      <img

        className={fit.blurred ? "art-foreground" : "art-cover"}

        src={src}

        alt={alt}

        referrerPolicy="no-referrer"

        onLoad={measure}

        onError={() => setAttempt((index) => index + 1)}

        style={fit.blurred ? { maxWidth: fit.width, maxHeight: fit.height } : undefined}

      />

    </div>

  );

}
