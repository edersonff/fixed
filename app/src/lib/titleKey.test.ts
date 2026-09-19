import { titleKey } from "./titleKey.ts";

function assertEqual(actual: string, expected: string, label: string): void {

  if (actual !== expected) {

    throw new Error(`${label}: expected "${expected}", got "${actual}"`);

  }

  console.log(`ok - ${label}`);

}

assertEqual(titleKey("BOMBANANA!"), "BOMBANANA!", "leaves a slash-free real installed title unchanged");

assertEqual(titleKey("Friendly Steps"), "Friendly Steps", "leaves the other real installed title unchanged");

assertEqual(titleKey("Foo / Bar"), "Foo _ Bar", "mirrors the backend's title.replace('/', '_') folder sanitisation");
