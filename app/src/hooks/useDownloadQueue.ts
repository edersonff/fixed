import { updateDownloadEntries } from "../lib/downloadsStore";

export function useDownloadQueue() {

  return { setDownloads: updateDownloadEntries };

}
