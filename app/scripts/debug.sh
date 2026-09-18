#!/usr/bin/env bash

set -u

LABEL() { printf "\n=== %s ===\n" "$1"; }

LABEL "PROCESS"

APP_PID=$(pgrep -x app | head -1)

echo "app pid: ${APP_PID:-none}"

if [ -n "${APP_PID:-}" ]; then

  ps -p "$APP_PID" -o pid=,etime=,rss= --no-headers

  echo "webkit env: $(tr '\0' '\n' < /proc/$APP_PID/environ | grep -E 'WEBKIT' | tr '\n' ' ')"

fi

LABEL "WINDOWS"

if [ -n "${APP_PID:-}" ]; then

  for w in $(xdotool search --pid "$APP_PID" 2>/dev/null); do

    eval $(xdotool getwindowgeometry --shell "$w" 2>/dev/null)

    echo "win $w ${WIDTH}x${HEIGHT}+${X}+${Y}"

  done

fi

LABEL "VITE"

echo "root: $(curl -s -o /dev/null -w '%{http_code}' -m 4 http://localhost:1420/)"

echo "app.tsx: $(curl -s -o /dev/null -w '%{http_code}' -m 8 http://localhost:1420/src/App.tsx)"

echo "main.tsx: $(curl -s -o /dev/null -w '%{http_code}' -m 8 http://localhost:1420/src/main.tsx)"

LABEL "INSPECTOR"

INSPECT=$(curl -s -m 4 http://127.0.0.1:9222/json 2>/dev/null)

if [ -n "$INSPECT" ]; then

  echo "$INSPECT" | head -40

else

  echo "no inspector on :9222"

fi

LABEL "CAPTURE"

STAMP=$(date +%H%M%S)

if [ -n "${APP_PID:-}" ]; then

  MAINWIN=$(for w in $(xdotool search --pid "$APP_PID" 2>/dev/null); do eval $(xdotool getwindowgeometry --shell "$w" 2>/dev/null); echo "$((WIDTH*HEIGHT)) $w"; done | sort -rn | head -1 | cut -d' ' -f2)

  if [ -n "${MAINWIN:-}" ]; then

    xdotool windowactivate --sync "$MAINWIN" 2>/dev/null || xdotool windowactivate "$MAINWIN" 2>/dev/null

    sleep 2

    import -window "$MAINWIN" "png:/tmp/opencode/proof-A-$STAMP.png" 2>&1 | head -1

    eval $(xdotool getwindowgeometry --shell "$MAINWIN")

    ffmpeg -hide_banner -loglevel error -f x11grab -video_size ${WIDTH}x${HEIGHT} -i :0.0+${X},${Y} -frames:v 1 -y "/tmp/opencode/proof-B-$STAMP.png" 2>&1 | head -1

    file -b "/tmp/opencode/proof-A-$STAMP.png" 2>/dev/null | head -1

    file -b "/tmp/opencode/proof-B-$STAMP.png" 2>/dev/null | head -1

    echo "stamp: $STAMP"

  fi

fi

LABEL "DEV LOG TAIL"

tail -n 15 /tmp/opencode/fixed-tauri-dev4.log 2>/dev/null | grep -viE "^\s*$" | tail -n 10
