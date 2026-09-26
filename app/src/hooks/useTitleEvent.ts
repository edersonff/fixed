import { useEffect } from "react";

import { listen } from "@tauri-apps/api/event";

// Shared by launch-progress and fix-progress: both are Tauri events carrying a `title` field, and
// every listener here only cares about the one game it was rendered for.
export function useTitleEvent<T extends { title: string }>(event: string, title: string, onMatch: (payload: T) => void) {

  useEffect(() => {

    const unlisten = listen<T>(event, (received) => {

      if (received.payload.title !== title) {

        return;

      }

      onMatch(received.payload);

    });

    return () => {

      unlisten.then((stop) => stop());

    };

  }, [event, title, onMatch]);

}
