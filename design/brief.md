# FIXED — Concept Brief v1

Desktop client for online-fix.me. Browse → download → install → configure → add to Steam → play.
Zero login. Zero config. Minimal clicks. Unofficial project, EN first (RU optional later).

## Name (owner decision pending)

Mockups brand as **FIXED** (single-string rebrand).
Candidates: FIXED / Fixbox / Spacewar (inside joke: all fixes run as Spacewar appid 228980) / OnlineFix (site name).

## Pillars

1. Open = playable catalog. No login, no onboarding maze.
2. One primary action per screen: Download → becomes Install → becomes Play.
3. Defaults do the work: library `~/games` (Linux) / `C:\Games` (Windows), Add to Steam ON, parallel downloads ON, mirror HTTP default, torrent optional with ISP warning.
4. Plugins generalized: manifest of steps (framework → files → first-run cycle → repair if needed → launch options). Accepts any BepInEx-flavor Thunderstore zip.
5. Fewer screens. Total: Home, Game Detail, Downloads (inline cards), Library, Plugins (per game), First-run (once).

## Concept screens

- 01 Home — featured hero (latest big build) + Fresh fixes row + Most played row + search in top bar.
- 02 Game Detail — poster header, meta (build, release, play via, modes, size), screenshots, repacker trust votes, one Download CTA; source popover shows mirrors default + torrent with ISP warning; plugins strip.
- 03 Downloads — inline card state machine: Queued → Downloading (%, speed, ETA) → Extracting → Configuring → Added to Steam → Ready (Play). Parallel by default.
- 04 Library — installed grid, status badges (Ready / Update / Plugins), Play launches direct (Steam client not required).
- 05 Plugins (PEAK case) — BepInEx step, PEAKUnlimited.dll step, first-run cycle (start & close once), Fix Repair when needed, Linux launch options auto (`WINEDLLOVERRIDES="winhttp=n,b" %command%`).
- 06 First-run — single card: detected OS + library folder preview, Add to Steam ON, one button "Start browsing".
- 07 Brand — wordmark options sheet.

## Stack verdict

**Tauri v2 + React + motion (framer-motion)**.
Rust core: scraper, download manager (parallel HTTP + torrent via rqbit), unzip, Steam shortcuts.vdf writer, plugin step engine.
REJECTED: Electron (RAM + binary size), pure native (slower motion iteration).
Risk named: Linux webkit2gtk variance → CSS feature walls, graceful fallbacks.

## Real site data (scraped 2026-09-16)

Posters at `https://online-fix.me/uploads/posts/<ym>/<id>_poster.jpg`.
Game page fields: build version, updated date, views, votes, play-via, modes, 4 download lanes (Hosters / Drive / Direct / Torrent), Fix Repair note, manual install steps, comments.
Detail mockup uses Ready or Not: Build 10092026, Steam + Epic overlay, Coop/Multiplayer, 1.79M views, 250+ repacker votes.

## Release (later, not now)

README: unofficial disclaimer + link to online-fix.me + build link. Optional donation (buy me a coffee). GitHub Pages landing with download button. Publish only after polish.

## Standing quality gates (never lowered)

- Cold start < 2s. Open → first game downloading ≤ 3 clicks. Open → play installed ≤ 1 click.
- No settings maze: first run has ONE action.
- Every screen: primary action findable in 1s (thumbnail + grayscale tests).
- Code: EN only, no comments in shipped code, 300-line file cap.
