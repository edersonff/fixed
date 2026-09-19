import { needsBlurBackdrop } from "./artQuality";

function assertEqual(actual: boolean, expected: boolean, label: string): void {

  if (actual !== expected) {

    throw new Error(`${label}: expected ${expected}, got ${actual}`);

  }

  console.log(`ok - ${label}`);

}

assertEqual(

  needsBlurBackdrop(1920, 620, 340, 210),

  true,

  "a 3.1:1 hero forced into a 1.62:1 library card triggers the blur backdrop",

);

assertEqual(

  needsBlurBackdrop(600, 900, 300, 450),

  false,

  "a 2:3 Steam cover in a matching 2:3 poster box stays a plain cover, no blur",

);

assertEqual(

  needsBlurBackdrop(600, 900, 300, 210),

  true,

  "a 2:3 cover pushed into the old 10:7 landscape poster box would have triggered the blur",

);
