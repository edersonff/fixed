import { Cloud } from "lucide-react";

import { Download } from "lucide-react";

import { Globe } from "lucide-react";

import { HardDrive } from "lucide-react";

import { Link2 } from "lucide-react";

export const LANE_ICONS: Record<string, typeof Globe> = {

  hosters: Globe,

  drive: HardDrive,

  direct: Link2,

  torrent: Download,

  mega: Cloud,

  yandex: HardDrive,

  "google-drive": Cloud,

  mirror: Link2,

};

export const LANE_NOTES: Record<string, string> = {

  hosters: "Recommended Mirror",

  drive: "Fast Mirror",

  direct: "Site Files",

  mega: "Mega Mirror",

  yandex: "Yandex Disk Mirror",

  "google-drive": "Google Drive Mirror",

  mirror: "Mirror",

  torrent: "P2P · Optional",

};
