import { downloadStartArgs } from "./downloads.ts";

function assertEqual<T>(actual: T, expected: T, label: string): void {

  if (actual !== expected) {

    throw new Error(`${label}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`);

  }

  console.log(`ok - ${label}`);

}

const args = downloadStartArgs("https://online-fix.me/bombanana.html", "BOMBANANA_", "https://mirror.example/x", "1.0");

assertEqual(args.pageUrl, "https://online-fix.me/bombanana.html", "pageUrl carries the exact site page the download started from");

assertEqual(args.title, "BOMBANANA_", "title is the safe title, unchanged");

assertEqual(args.laneUrl, "https://mirror.example/x", "laneUrl passes through untouched");

assertEqual(args.build, "1.0", "build passes through untouched");

let threwOnEmptyPageUrl = false;

try {

  downloadStartArgs("", "BOMBANANA_", "https://mirror.example/x", null);

} catch {

  threwOnEmptyPageUrl = true;

}

assertEqual(threwOnEmptyPageUrl, true, "an empty page url throws instead of starting a download with no .fixed-source marker");

let threwOnUndefinedPageUrl = false;

try {

  downloadStartArgs(undefined as unknown as string, "BOMBANANA_", "https://mirror.example/x", null);

} catch {

  threwOnUndefinedPageUrl = true;

}

assertEqual(threwOnUndefinedPageUrl, true, "an undefined page url throws the same way an empty one does");
