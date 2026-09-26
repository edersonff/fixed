import { deriveFixState, fixLabelFor, fixPressEnabled } from "./fixState.ts";

function assertEqual<T>(actual: T, expected: T, label: string): void {

  if (actual !== expected) {

    throw new Error(`${label}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`);

  }

  console.log(`ok - ${label}`);

}

assertEqual(deriveFixState(0, false), "intact", "no missing files and no protection block is intact");

assertEqual(deriveFixState(3, false), "missing", "missing files with no protection block is missing");

assertEqual(deriveFixState(3, true), "removedAgain", "protection block wins even with missing files");

assertEqual(deriveFixState(0, true), "removedAgain", "protection block wins even with no missing files");

assertEqual(fixLabelFor("intact"), "Play", "intact files label Play");

assertEqual(fixLabelFor("missing"), "Fix", "missing files label Fix");

assertEqual(fixLabelFor("removedAgain"), "Fix", "blocked files still label Fix");

assertEqual(fixPressEnabled("intact"), true, "intact press is enabled");

assertEqual(fixPressEnabled("missing"), true, "missing press is enabled");

assertEqual(fixPressEnabled("removedAgain"), false, "removedAgain press is disabled");
