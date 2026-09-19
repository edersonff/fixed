#!/usr/bin/env bash
# Prints every precondition the Steam launch chain depends on, for one game title.
# Read-only: never starts, stops or mutates anything. Usage: tools/steam-doctor.sh "BOMBANANA!"
set -u

TITLE="${1:-}"
[ -z "$TITLE" ] && { echo "usage: $0 \"<game title>\""; exit 2; }

ok()   { printf '  \033[32mOK\033[0m   %s\n' "$1"; }
bad()  { printf '  \033[31mFAIL\033[0m %s\n' "$1"; FAILED=$((FAILED+1)); }
info() { printf '       %s\n' "$1"; }
FAILED=0

STEAM_ROOT=""
for candidate in "$HOME/.local/share/Steam" "$HOME/.steam/steam"; do
  [ -f "$candidate/config/config.vdf" ] && { STEAM_ROOT="$candidate"; break; }
done

echo "== steam client =="
if [ -z "$STEAM_ROOT" ]; then
  bad "no steam install found"
  exit 1
fi
ok "root $STEAM_ROOT"
pgrep -x steam        >/dev/null && ok "steam process up"        || info "steam process down"
pgrep -x steamwebhelper >/dev/null && ok "steamwebhelper up (accepts steam:// urls)" \
                                   || info "steamwebhelper down (steam:// urls are swallowed)"
UPDATING=$(grep -c "Downloading update" "$STEAM_ROOT/logs/console-linux.txt" 2>/dev/null | tail -1)
info "client-update lines in today's log: ${UPDATING:-0}"

echo
echo "== game on disk =="
FOLDER="$HOME/games/$TITLE"
if [ -d "$FOLDER" ]; then
  ok "folder $FOLDER"
  EXE=$(find "$FOLDER" -maxdepth 4 -iname '*.exe' ! -iname '*crashhandler*' ! -iname '*vcredist*' \
        ! -iname '*dxsetup*' ! -iname 'unins*' 2>/dev/null | head -1)
  [ -n "$EXE" ] && ok "exe $EXE" || bad "no launchable .exe under the folder"
else
  bad "no folder at $FOLDER"
  EXE=""
fi

echo
echo "== steam shortcut =="
VDF=$(find "$STEAM_ROOT/userdata" -maxdepth 3 -name shortcuts.vdf 2>/dev/null | head -1)
if [ -z "$VDF" ]; then
  bad "no shortcuts.vdf"
  exit 1
fi
ok "vdf $VDF"
info "$(stat -c '%s bytes, modified %y' "$VDF")"

APPID=$(TITLE="$TITLE" python3 - "$VDF" <<'PY'
import os, re, struct, sys
data = open(sys.argv[1], 'rb').read()
want = os.environ['TITLE'].encode()
for m in re.finditer(rb'\x02appid\x00(....)\x01AppName\x00([^\x00]*)\x00', data, re.S):
    if m.group(2) == want:
        print(struct.unpack('<I', m.group(1))[0])
        break
PY
)
if [ -n "$APPID" ]; then
  ok "shortcut present, appid $APPID"
  GAMEID=$(python3 -c "print((int($APPID) << 32) | 0x02000000)")
  info "launch url steam://rungameid/$GAMEID"
else
  bad "no shortcut entry named '$TITLE' — steam cannot launch what it does not list"
  GAMEID=""
fi

echo
echo "== compat tool =="
if [ -n "$APPID" ]; then
  if APPID="$APPID" python3 - "$STEAM_ROOT/config/config.vdf" <<'PY'
import os, re, sys
body = open(sys.argv[1], encoding='utf8', errors='replace').read()
start = body.find('CompatToolMapping')
sys.exit(0 if start >= 0 and re.search(r'"%s"\s*\{' % os.environ['APPID'], body[start:]) else 1)
PY
  then
    ok "CompatToolMapping has $APPID"
  else
    bad "no CompatToolMapping for $APPID — steam reports 'Game configuration unavailable'"
  fi
fi

echo
echo "== proton =="
find "$STEAM_ROOT/steamapps/common" -maxdepth 2 -name proton -type f 2>/dev/null | head -3 \
  | while read -r p; do info "$p"; done

echo
echo "== last launch attempt in steam's own log =="
grep -h "GameAction \[AppID ${APPID:-none}" "$STEAM_ROOT/logs/console_log.txt" 2>/dev/null | tail -6 \
  || info "no GameAction lines for this appid"

echo
echo "== live game session =="
PID=$(pgrep -f "SteamLaunch AppId=${APPID:-none}" | head -1)
if [ -n "$PID" ]; then
  ok "steam launched it, reaper pid $PID"
  GAMEPID=$(pgrep -f "$(basename "${EXE:-none}")" | head -1)
  if [ -n "$GAMEPID" ]; then
    VARS=$(tr '\0' '\n' < "/proc/$GAMEPID/environ" 2>/dev/null | grep -c '^Steam')
    [ "${VARS:-0}" -gt 0 ] && ok "game pid $GAMEPID has $VARS Steam* vars — real session, networking alive" \
                           || bad "game pid $GAMEPID has zero Steam* vars — no session, online-fix networking is dead"
  fi
else
  info "not running right now"
fi

echo
[ "$FAILED" -eq 0 ] && echo "ALL CHECKS PASSED" || echo "$FAILED CHECK(S) FAILED"
exit "$FAILED"
