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
