export type ArtIdentity = {
  hue: number;
  pattern: number;
  initials: string;
};


const PATTERN_COUNT = 4;


function hashTitle(title: string): number {

  let hash = 0;

  for (let index = 0; index < title.length; index += 1) {

    hash = (hash << 5) - hash + title.charCodeAt(index);

    hash |= 0;

  }

  return Math.abs(hash);

}


function initialsOf(title: string): string {

  const words = title.trim().split(/\s+/).filter(Boolean);

  if (words.length === 0) {

    return "?";

  }

  if (words.length === 1) {

    return words[0].slice(0, 2).toUpperCase();

  }

  return `${words[0][0]}${words[1][0]}`.toUpperCase();

}


export function artIdentity(title: string): ArtIdentity {

  const hash = hashTitle(title);

  return {
    hue: hash % 360,
    pattern: hash % PATTERN_COUNT,
    initials: initialsOf(title),
  };

}
