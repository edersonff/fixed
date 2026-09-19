import { useEffect } from "react";

export function useMouseBackNavigation(closeDetail: (value: null) => void): void {

  useEffect(() => {

    function handleMouseButton(event: MouseEvent) {

      if (event.button !== 3 && event.button !== 4) {

        return;

      }

      event.preventDefault();

      if (event.button === 3) {

        closeDetail(null);

      }

    }

    window.addEventListener("mouseup", handleMouseButton);

    return () => {

      window.removeEventListener("mouseup", handleMouseButton);

    };

  }, [closeDetail]);

}
