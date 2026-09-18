# REVIEW GATE — law of this project

No artifact reaches the owner without passing this gate. Builder never scores own work.

## Stages (strict order, no skipping)

1. **STABLE** — all writers exited (codex PID gone). Zero file mtime newer than 60s. Moving targets are judged never.
2. **RESHOOT** — fresh screenshots of every screen at 1280x800 after stability check. Stale shots = automatic gate failure (measured: 22:23 shots vs 22:28-35 edits = FAIL).
3. **REFERENCE PASS** — side-by-side vs design/reference/v2-product/ per screen. Fidelity scored 0-100 by reviewer with eyes on pixels (multimodal-looker), not greps.
4. **STATE PASS** — Playwright captures: default + hover + focus on primary interactive elements per screen. Micro-interactions verified as IMAGES, never as CSS text counts.
5. **JUDGE PASS** — hostile agent scores every criterion in design/criteria.md 0-100 with named evidence. Lowest score drives the next iteration until all >= 80.
6. **OWNER** — only gate-passed artifacts reach the owner. If the owner finds a defect any stage should have caught, the stage gets fixed, not just the artifact.

## Data truth rules

- Game titles, posters, builds, dates come ONLY from verified scraped data (design/screens contract table). Reference AI images are layout authority, NEVER data authority. Measured failure: AI-fabricated names (The Finals, Helldivers 2, Deep Rock Galactic, FH5) leaked onto real posters — c22 scored 15.
- One poster URL maps to exactly one title across ALL screens.
- Cross-screen data story must agree (same game cannot be "Ready to install" on Home while Installed in Library while 62% in Downloads).

## Verification bans (measured failures)

- Grep counts are NOT interaction proof (keyframes=6 while 99% hovers missing — owner report 2026-09-16).
- Proxy-layer checks (fits, 0 broken, closed tags) are hygiene, never quality verdicts.
- Sub-agent reports are leads; re-run the core receipt before any claim.
