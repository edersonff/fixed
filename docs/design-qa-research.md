# Design QA Research — review gate for HTML concept screens

Date: 2026-09-16. Goal: concrete mechanics behind a mandatory review gate that lifts concept screens to "better than Steam/Epic" polish with full micro-interaction coverage.

Evidence labels: **[V]** = VERIFIED (page fetched / canonical text quoted this session). **[R]** = RECALLED from memory, URL given, NOT re-fetched today.

## 0. Source register (part 1)

- Material 3 easing+duration tokens **[V]** https://m3.material.io/styles/motion/easing-and-duration (also `/tokens-specs`, `/overview/how-it-works`)
- M3 duration scale (16 attrs, 50–1000ms) **[V]** https://github.com/material-components/material-components-android/blob/master/docs/theming/Motion.md
- Primer motion tokens + MUST/NEVER rules **[V]** https://github.com/primer/primitives/blob/main/DESIGN_TOKENS_GUIDE.md — base values in PR #1080 / #1350
- Linear redesign mechanics **[V]** https://linear.app/now/how-we-redesigned-the-linear-ui
- Linear process (no formal reviews; async Slack crit; feature-flag dogfood) **[V]** https://www.lennysnewsletter.com/p/how-linear-builds-product
- Karri Saarinen's 10 craft rules **[V]** https://www.figma.com/blog/karri-saarinens-10-rules-for-crafting-products-that-stand-out/
- Playwright visual comparisons **[V]** https://playwright.dev/docs/test-snapshots

## 0. Source register (part 2)

- WCAG 2.2 + Understanding SC 2.5.8 (24×24px) **[V]** https://www.w3.org/TR/WCAG22 ; https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html
- prefers-reduced-motion (vestibular disorder, CSS/JS patterns) **[V]** https://web.dev/articles/prefers-reduced-motion
- Design critique protocol (facilitator, round-robin, quotas) **[V]** https://www.nngroup.com/articles/design-critiques/
- Chromatic visual-test flow (pixel diff, UI Review, TurboSnap) **[V]** https://www.chromatic.com/docs
- Motion (Framer) gestures/variants/orchestration **[V]** https://motion.dev/docs/react-animation
- Apple HIG Motion **[R]** https://developer.apple.com/design/human-interface-guidelines/motion — page exists, title verified; body is JS-gated, content recalled: purposeful/contextual motion, use system animation APIs, honor Reduce Motion
- Apple 44×44pt tap targets **[R]** https://developer.apple.com/design/human-interface-guidelines/buttons
- Vercel Web Interface Guidelines **[R]** https://github.com/vercel/web-interface-guidelines — 404 on fetch today; recalled checklist: :focus-visible on interactives, aria-label on icon-only buttons, explicit img dimensions, no user-scalable=no

## 1. What elite teams actually do (verified mechanics)

**Linear** (linear.app/now/how-we-redesigned-the-linear-ui [V]): designer produced a *complete set of screens per day*, hundreds total, linked into prototypes; direction validated with "stress tests" against the 8 main views BEFORE implementation; behaviors documented per component (sidebar, tabs, headers); feature-flag rollout to the whole company for dogfooding ("everyone can try and flag what feels wrong"); 6 weeks, ~5-person team. In their process interview [V]: "We don't have formal product or design reviews. It's more ad hoc and iterative — designers share early design in a project Slack group and get asynchronous feedback." Karri's rule [V]: the spec is the *baseline*, not the finish line.

**NN/g critique protocol** [V]: standalone crit ≠ design review; roles presenter/critiquer/facilitator; facilitator pre-distributes scope+agenda, converts opinions ("too red!") into goal-referenced questions ("does this help the user register faster?"), documents decisions publicly, follows up with action items; formats: round-robin, quotas (2 strengths + 1 improvement each).

**Chromatic flow** [V]: every push → per-state snapshots (visual), simulated interactions (hover/click/type), axe a11y per component; humans review *pixel diffs*, not whole screens; TurboSnap tests only changed subtree.

## 2. Recommended review loop (opinionated adoption)

5 stages; machine gates run before any human looks.

**S0 Reference lock.** Before building: pick 2–3 reference screens (Steam/Epic/Linear), screenshot them, store beside the concept. Every later stage compares side-by-side against these. (Linear stress-tested direction against all main views first [V].)

**S1 Builder self-sweep.** Builder runs the Playwright state-capture matrix (§4) on his own screen BEFORE claiming done. Output: a `/states/` dir with one PNG per state. No states dir = nothing to review.

**S2 Machine gate (Playwright, fully local).** (a) Visual: `expect(page).toHaveScreenshot()` per state; golden files in repo; regenerate deliberately with `--update-snapshots` [V]. (b) Motion lint: grep CSS/TS for raw `ms`/`cubic-bezier` outside `design/motion.md` tokens → fail. (c) Targets: JS sweep `getBoundingClientRect` ≥24×24 (WCAG 2.5.8 AA floor [V]), flag <44px. (d) Reduced-motion: `page.emulateMedia({reducedMotion:'reduce'})` re-capture; motion must not regress content [V web.dev].

**S3 Blind judge crit.** Reviewer who did NOT build scores 20–30 written criteria 0–100 against the S0 reference side-by-side; lowest score drives next iteration (project judge rule). NN/g rules apply: scope fixed, feedback references the goal, no directives [V].

**S4 Accept.** Pass = all states captured, 0 untokenized motion values, 0 unaccepted diffs, judge ≥ bar. Then ratchet criteria harder next screen.

## 3. Motion token system (drop-in `design/motion.md`)

Published systems agree on 5 things: closed duration scale; small named easing set; duration+easing picked by *transition role* (enter/exit/on-screen) — M3's pairs table [V]; composite transition tokens (Primer `--motion-transition-*`) [V]; hard caps as rules — Primer: UI ≤300ms MUST, hover/focus 100ms SHOULD, never >500ms, never decoration-only, MUST respect reduced-motion [V].

```css
:root {
  /* duration — M3 16-step scale collapsed to 5 + Primer caps [V] */
  --dur-micro: 100ms;  /* hover, focus ring */
  --dur-short: 200ms;  /* state change, exits */
  --dur-base:  300ms;  /* max for UI interactions (Primer MUST) */
  --dur-medium: 400ms; /* enters */
  --dur-long:  500ms;  /* large surfaces; NEVER exceed for interactions */
  /* easing — M3 CSS-usable set (emphasized path not in CSS → standard fallback, per M3) [V] */
  --ease-standard:   cubic-bezier(0.2, 0, 0, 1);
  --ease-decelerate: cubic-bezier(0.05, 0.7, 0.1, 1); /* enters */
  --ease-accelerate: cubic-bezier(0.3, 0, 0.8, 0.15); /* exits */
  --ease-linear:     cubic-bezier(0, 0, 1, 1);        /* spinners only */
  /* composite tokens (Primer pattern) [V] */
  --motion-hover: var(--dur-micro)  var(--ease-standard);
  --motion-enter: var(--dur-medium) var(--ease-decelerate);
  --motion-exit:  var(--dur-short)  var(--ease-accelerate);
}
```

**Per-component-class response rules** (each class reads tokens, never raw numbers):

| class | enter | exit | hover/focus | notes |
|---|---|---|---|---|
| button / chip / icon-btn | — | — | 100ms standard | pointer feedback instant |
| tooltip / popover | 200ms decelerate | 100–150ms accelerate | — | WCAG 1.4.13: hover content dismissible [V] |
| dropdown menu | 200ms decelerate | 150ms accelerate | items 100ms | |
| modal / dialog | 300–400ms decelerate | 200ms accelerate | — | backdrop fades 200ms linear |
| toast | 300ms decelerate | 200ms accelerate | — | |
| tab / view switch | 300ms standard | — | — | on-screen transform |
| page / full-screen | 400–500ms decelerate | 200ms accelerate | — | M3 default pair [V] |
| skeleton / spinner | — | — | — | 1s loop, LINEAR only (Primer) [V] |

Reduced motion (MUST — Primer [V]; pattern — web.dev [V]):

```css
@media (prefers-reduced-motion: reduce) {
  *, ::before, ::after { transition-duration: 1ms; animation-duration: 1ms; }
}
```
If JS-animated: Motion/Framer — set defaults once via `MotionConfig`, gestures via `whileHover/whileTap/whileFocus`, exits need `AnimatePresence`, first paint without flicker = `initial={false}`, stagger via `delayChildren` [V motion.dev].

## 4. Interaction QA checklist (per element; every item = one Playwright screenshot)

States matrix — capture ALL for every interactive element, light AND dark:
1. default — `expect(locator).toHaveScreenshot('btn-default.png')`
2. hover — `await locator.hover()` then shot
3. focus-visible — `await locator.focus()` (or real Tab key) then shot; ring must be visible+contrasted (WCAG 2.4.7 [V])
4. active/pressed — `await page.mouse.move(); page.mouse.down()` hold while shooting
5. disabled — set attribute/state, shot; visually distinct, no hover change
6. loading (buttons with async action; skeleton screens)
7. empty state (0 items ≠ blank page)
8. error state (form invalid, fetch fail)
9. reduced motion — `await page.emulateMedia({reducedMotion: 'reduce'})`, re-capture 1–2 hero flows [V web.dev]

Hard gates:
- target ≥24×24 CSS px (WCAG 2.5.8 AA [V]); aim 44×44 (Apple HIG [R])
- icon-only controls have aria-label; img have width/height (Vercel [R])
- keyboard: Tab order sane, Escape closes overlays, Enter/Space activate
- diff review: run `npx playwright test --update-snapshots` only when change is INTENDED; diffs get human eyes (Chromatic model [V])
- same machine/browser for golden vs compare — rendering varies by OS/GPU/headless (Playwright warning [V])

## 5. Replicating Chromatic/Percy locally with Playwright only

Chromatic = per-state snapshot + pixel diff + human review of *diffs*, change-scoped (TurboSnap) [V]. Local equivalent, zero SaaS:
1. One spec per screen: loop `[state, action]` matrix → `toHaveScreenshot()` each; goldens committed.
2. `maxDiffPixels` set small (e.g. 100) — configurable globally in `expect.toHaveScreenshot` [V]; `stylePath` CSS to hide volatile nodes (iframes, clocks) [V].
3. Playwright HTML reporter → reviewer sees only failing diffs = the review queue.
4. Change-scoping: run only specs whose files changed (git diff) — poor-man's TurboSnap.
5. Percy-equivalent responsive sweep: `projects` config per viewport.

## 6. Verdict — adopt as the gate

Copy: Linear's stress-test-before-build + dogfood-everything [V]; NN/g facilitation rules for the crit [V]; M3 role-based pairs + Primer caps for motion.md [V]; Chromatic per-state diff review, run on Playwright [V].
Do NOT copy: "no formal reviews" at Linear's scale — here the gate IS formal because the judge is the check on builder bias.
Steam/Epic bar restated: every row of §4 has a golden PNG for every interactive element, plus side-by-side vs reference, plus judge ≥ bar. Missing PNG = gate fails.

— end —
