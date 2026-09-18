# FIXED motion system

The product moves with a restrained, utility-first rhythm: content enters in a short blur-and-rise, controls answer immediately, and state changes settle without decorative motion.

## Tokens

```css
--dur-micro: 100ms;
--dur-short: 200ms;
--dur-med: 300ms;
--dur-long: 500ms;
--ease-standard: cubic-bezier(.2,0,0,1);
--ease-decel: cubic-bezier(.05,.7,.1,1);
--ease-accel: cubic-bezier(.3,0,.8,.15);
--ease-pop: cubic-bezier(.2,1.6,.4,1);
```

## Interaction rules

- Hover and focus feedback responds within `--dur-micro`.
- State changes use `--dur-short`; UI interactions never exceed `--dur-med`.
- Scene and primary-content entrances use `--dur-med` with `--ease-decel`.
- Exits use `--dur-short` with `--ease-accel`.
- Row items stagger by 50ms, with the first item at zero delay.
- The load order is primary content first, then secondary content trails behind it.
- Entrances may combine blur, opacity, and upward translation. After entry, only transform, opacity, and color are animated.
- `--ease-pop` is reserved for toggles and status badges.
- There is no looping ambient motion in the product surfaces.

## Screen choreography

- Home: hero enters first; the two game rails follow in a 50ms row stagger.
- Game detail: poster and game identity enter first; metadata, action, and sources follow.
- Downloads: the queue enters top-to-bottom; progress remains independent from its controls.
- Library: the six-card grid enters row-first; the empty shelf panel arrives last.
- Plugins: the PEAK context card enters before the setup checklist; the repair disclosure stays quiet until used.
- First run: the setup card settles first, then its settings and start action.
- Brand: the wordmark family enters before token and radius demonstrations.

## Reduced motion

Every screen uses:

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation: none !important;
    transition: none !important;
  }
}
```

Reduced motion removes blur, stagger, hover travel, spinner rotation, and all transitions while preserving state, contrast, and focus visibility.
