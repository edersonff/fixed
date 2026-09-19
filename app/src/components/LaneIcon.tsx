import { Link2 } from "lucide-react";

import { LANE_ICONS } from "../lib/lanes";

export function LaneIcon({ kind }: { kind: string }) {

  const Icon = LANE_ICONS[kind] ?? Link2;

  return <Icon size={17} strokeWidth={1.8} />;

}
