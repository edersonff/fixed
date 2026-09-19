export function isLowResArt(naturalWidth: number, renderedWidth: number, threshold = 1.15): boolean {

  if (naturalWidth <= 0 || renderedWidth <= 0) {

    return false;

  }

  return renderedWidth > naturalWidth * threshold;

}


export function isAspectMismatch(

  naturalWidth: number,

  naturalHeight: number,

  renderedWidth: number,

  renderedHeight: number,

  tolerance = 1.5,

): boolean {

  if (naturalWidth <= 0 || naturalHeight <= 0 || renderedWidth <= 0 || renderedHeight <= 0) {

    return false;

  }

  const source = naturalWidth / naturalHeight;

  const frame = renderedWidth / renderedHeight;

  return frame / source > tolerance || source / frame > tolerance;

}


export function needsBlurBackdrop(

  naturalWidth: number,

  _naturalHeight: number,

  renderedWidth: number,

  _renderedHeight: number,

): boolean {

  return isLowResArt(naturalWidth, renderedWidth);

}
