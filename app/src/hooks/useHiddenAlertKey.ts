import { useCallback, useState } from "react";

const HIDE_KEY = "files-alert-hidden";

function readHiddenKey(): string {

  try {

    return window.localStorage.getItem(HIDE_KEY) ?? "";

  } catch {

    return "";

  }

}

function writeHiddenKey(key: string): void {

  try {

    window.localStorage.setItem(HIDE_KEY, key);

  } catch {

    return;

  }

}

export function useHiddenAlertKey(currentKey: string) {

  const [hiddenKey, setHiddenKey] = useState(readHiddenKey);

  const hide = useCallback(() => {

    writeHiddenKey(currentKey);

    setHiddenKey(currentKey);

  }, [currentKey]);

  return { hiddenKey, hide };

}
