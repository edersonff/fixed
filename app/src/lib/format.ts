export const MONTHS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];


export function formatDate(iso: string): string {

  const date = new Date(iso);

  if (Number.isNaN(date.getTime())) {

    return iso.slice(0, 10);

  }

  return `${String(date.getDate()).padStart(2, "0")} ${MONTHS[date.getMonth()]} ${date.getFullYear()}`;

}


export function prettyCategory(raw: string): string {

  return raw

    .replace("officialservers", "official servers")

    .replace(/(^|[- ])([a-z])/g, (_, prefix: string, letter: string) => `${prefix}${letter.toUpperCase()}`);

}


export function displayTitle(raw: string): string {

  return raw

    .trim()

    .split(/\s+/)

    .map((word) => {

      if (/^(?:[A-Za-z]\.){2,}[A-Za-z]?/.test(word)) {

        return word;

      }

      const normalized = word.toLocaleLowerCase();

      return normalized.replace(/^[a-zà-ÿ]/i, (letter) => letter.toLocaleUpperCase());

    })

    .join(" ");

}


export function displayLane(raw: string): string {

  return raw

    .replace("google-drive", "Google Drive")

    .replace(/(^|[- ])([a-z])/g, (_, prefix: string, letter: string) => `${prefix}${letter.toUpperCase()}`);

}


export function megabytes(bytes: number): string {

  return `${(bytes / 1_000_000).toFixed(1)} MB`;

}


export function diskSize(bytes: number): string {

  if (bytes >= 1_000_000_000) {

    return `${(bytes / 1_000_000_000).toFixed(1)} GB`;

  }

  return megabytes(bytes);

}
