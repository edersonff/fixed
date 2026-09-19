import { useMemo } from "react";

import type { CSSProperties } from "react";

import { artIdentity } from "../lib/artIdentity";

export function ArtFallback({ title }: { title: string }) {

  const identity = useMemo(() => artIdentity(title), [title]);

  const style = { "--art-hue": identity.hue } as CSSProperties;

  return (

    <div className="art-fallback" data-art-pattern={identity.pattern} style={style} aria-hidden="true">

      <span className="art-fallback-mark">{identity.initials}</span>

    </div>

  );

}
