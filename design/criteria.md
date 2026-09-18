# Concept Round — Judge Rubric v1 (ratchet: only hardens)

Judge scores each 0–100 with named justification. Lowest score drives next iteration.
A concept set ships only when every criterion ≥ 80.

## Composition

1. Home shows featured hero + ≥ 8 real covers (real online-fix.me poster URLs), zero placeholder gray boxes.
2. Cover-first composition: posters occupy ≥ 40% of Home visual area.
3. Every screen has exactly ONE primary CTA, accent-colored, dominant (grayscale test passes).
4. Pixel discipline: consistent spacing grid (8pt), aligned edges, no random offsets.
5. Typography: one family, ≤ 3 sizes per screen, clear hierarchy.

## Product truth

6. Download state machine visible: Queued → Downloading → Extracting → Configuring → Added to Steam → Ready/Play.
7. "Add to Steam" shown as DEFAULT-ON (install context).
8. Torrent option present + ISP warning appears when torrent selected.
9. No login UI anywhere. No onboarding maze: First-run = one card, one action.
10. Library shows status badges (Ready / Update / Plugins) + Play that implies direct launch.
11. Game Detail carries real meta: build version, release, play via, modes, trust votes.
12. Screens ≤ 6 core + brand sheet. No settings/management screen in concept set.
13. Empty state designed for Library (CTA to Home).
14. OS differences surface only where they matter (folder default, launch options) — no OS maze.

## Plugin story

15. Plugin flow visualizes real PEAK anatomy: BepInEx framework → dll → first-run cycle → Fix Repair when needed.
16. Linux launch option auto-applied (`WINEDLLOVERRIDES="winhttp=n,b"`), shown as automatic, not user homework.

## Visual identity

17. Dark cinematic theme, ≤ 2 accents + neutrals; accent reserved for primary action/progress.
18. Unique identity: not Steam-blue, not Epic, not admin-dashboard template.
19. Brand wordmark present, consistent across all screens, name ≠ "launcher".
20. Motion specs documented in design/motion.md (durations, easings, per-component rules). Product screens contain ZERO spec/debug/annotation text.

## Copy + data

21. All copy EN. Zero lorem, zero PT, zero Russian.
22. Game names match real online-fix.me titles; sizes/dates plausible; no invented genres.
23. No protocol tokens (scores, criteria refs, law names) inside artifacts.

## Accessibility + robustness

24. Text contrast ≥ 4.5:1 on backgrounds.
25. Focus/hover state visible on primary CTA.
26. Readable at 50% zoom (thumbnail test): primary action + state identifiable.

## Reusability

27. Concepts are self-contained HTML (implementation starting point), open directly in browser.
28. Would a cold viewer mistake it for a shipping AAA product? (judge gut + justification)

## Interaction v2 (ratchet — motion IMPLEMENTED, not annotated)

29. Every interactive element has designed hover, press (:active), and focus-visible states in real CSS (transform/opacity/color only).
30. Every screen has load choreography: staggered children, blur+rise+fade, primary first, secondary later.
31. State transitions animated: Download→Extracting→Ready cross-fade, badge pop-ins, toggle knobs, popover reveals.
32. Timing explicit everywhere: named duration + easing per animation; no naked `transition: all`, no default linear.
33. prefers-reduced-motion honored globally (all animation collapses).
34. Motion lives in CSS via the token custom properties; design/motion.md is the only place motion is described in words.
