# Asset Providers · online-fix.me desktop client (Rust/Tauri)

Researched 2026-09-16. Every `VERIFIED` claim was fetched/measure-run this session.
`UNVERIFIED` = could not be confirmed live; treat as hypothesis.

## TL;DR · recommended chains

| Asset | PRIMARY | Fallback 1 | Fallback 2 |
|---|---|---|---|
| Cover 2:3 | Steam CDN `library_600x900_2x.jpg` (true 600x900) | Steam CDN `library_600x900.jpg` (300x450!) | IGDB cover `t_cover_big`/`t_1080p` |
| Logo (transparent) | Steam CDN `logo_2x.png` (RGBA, e.g. 1280x720) | Steam CDN `logo.png` | SteamGridDB logos |
| Hero/banner wide | Steam CDN `library_hero.jpg` (1920x620) | SteamGridDB heroes | `capsule_616x353.jpg` |
| Small capsule | Steam CDN `capsule_231x87` / `capsule_sm_120` / `capsule_184x69` | Steam CDN `header.jpg` 460x215 | SteamGridDB grids 460x215 |
| Icon (extra) | SteamGridDB icons | Steam CDN `community_icon` (unverified) | — |

Cost: Steam CDN = free, no key. SteamGridDB = free key. IGDB = free (Twitch dev acct).
RAWG free tier is non-commercial + backlink — poor fit. MobyGames = PAID.

## appid resolution (online-fix.me → Steam appid)

Game page VERIFIED: `https://online-fix.me/games/officialservers/16889-ready-or-not-po-seti.html`
(HTTP 200). Steam link is NOT a raw URL — it is an obfuscated redirect anchor:

```html
<a href="https://online-fix.me/ext/R-vBLXs5iFbz_JmEpvGnKW-4n99sGU...=="
   title="store.steampowered.com" target="_blank">
```

Chain: parse anchor with `title="store.steampowered.com"` → GET `/ext/<token>`
(same session cookies + referer) → interstitial page titled "Переход на
store.steampowered.com" → final store URL revealed by JS after countdown.
Last hop UNVERIFIED over plain HTTP (JS/timer-gated); needs headless fetch or
token decode in the client.

Ready or Not appid = **1144200** VERIFIED via
`store.steampowered.com/api/appdetails?appids=1144200` → `"name":"Ready or Not"`.
Search on online-fix works via
`/index.php?do=search&subaction=search&story=ready+or+not` (VERIFIED HTTP 200).

Bonus zeroth source: online-fix hosts its own posters per game, e.g.
`https://online-fix.me/uploads/posts/2023-09/3416777649_poster.jpg` (VERIFIED in
page HTML). Low-res/RU-branded — use only as last-resort art.

## 1. Steam CDN — official art, no API key

Base: `https://cdn.cloudflare.steamstatic.com/steam/apps/<appid>/<file>`
(alias VERIFIED: `cdn.akamai.steamstatic.com` same paths). No key, plain GET.
All rows below fetched + pixel-measured this session on appids 1144200 & 730:

| File | Pixels (measured) | Type | Status |
|---|---|---|---|
| `header.jpg` | 460x215 | jpeg | VERIFIED both |
| `capsule_616x353.jpg` | 616x353 | jpeg | VERIFIED both |
| `capsule_231x87.jpg` | 231x87 | jpeg | VERIFIED both |
| `capsule_sm_120.jpg` | 120x45 | jpeg | VERIFIED both |
| `capsule_184x69.jpg` | 184x69 | jpeg | VERIFIED both |
| `library_600x900.jpg` | **300x450** | jpeg | VERIFIED both — see gotcha |
| `library_600x900_2x.jpg` | **600x900** | jpeg | VERIFIED both |
| `library_hero.jpg` | 1920x620 | jpeg | VERIFIED both |
| `library_hero_blur.jpg` | (blurred hero) | jpeg | VERIFIED both |
| `logo.png` | varies (640x199 / 639x360) | png RGBA | VERIFIED both |
| `logo_2x.png` | 1280x720 | png RGBA | VERIFIED 1144200 |
| `hero_capsule.jpg` | 374x448 (vertical!) | jpeg | VERIFIED both |
| `page_bg_raw.jpg` | 1438x810 | jpeg | VERIFIED 1144200 |
| `header_small.jpg` | — | — | **404 — does NOT exist** |

Example URLs (both fetched live):
- `https://cdn.cloudflare.steamstatic.com/steam/apps/1144200/header.jpg` → 206
- `https://cdn.cloudflare.steamstatic.com/steam/apps/1144200/library_600x900.jpg` → 206

### Steam CDN gotchas (measured)

1. **`library_600x900.jpg` lies**: serves the auto-generated 300x450 half-size
   (matches partner docs: "half-size 300x450 auto-generated"). Quality-first:
   always fetch `library_600x900_2x.jpg` for true 600x900.
2. `logo_2x.png` can be huge (1280x720) — grab both, prefer 2x.
3. `hero_capsule.jpg` is VERTICAL (374x448), not wide — good for 2:3-ish needs.
4. Guaranteed per app? Only if store page published (partner docs: library
   assets invisible otherwise). Old/delisted games may miss some files —
   always 404-fallback in the fetcher.
5. `store.steampowered.com/api/appdetails?appids=<id>&filters=basic` (no key)
   returns name + `header_image` etc. VERIFIED. Its rate limit: UNVERIFIED
   (undocumented; cache results).

### Licensing / hotlink stance (non-Valve client)

Assets are Valve/publisher marketing uploads governed by Steamworks partner
docs (https://partner.steamgames.com/doc/store/assets/standard,
.../libraryassets). There is **no explicit license for third-party clients**;
hotlinking store art is long-tolerated practice (SteamDB, IsThereAnyDeal do
it). UNVERIFIED legal stance — mitigate: cache on disk (already planned),
identify your UA, no re-distribution of the archive. Source sizes measured:
hero 304KB, page_bg 1.38MB, logo 59KB.

## 2. SteamGridDB (SGDB) — best-in-class fan art

Docs: https://www.steamgriddb.com/api/v2 (Swagger, JS-rendered; endpoint list
mirrored in github.com/SteamGridDB/node-steamgriddb README).

- Auth: `Authorization: Bearer <key>`, free key generated at
  steamgriddb.com/profile/preferences (VERIFIED via 2 official wrappers).
- Asset types: **grids** (wide 460x215 / 920x430 AND vertical 600x900),
  **heroes**, **logos**, **icons** (ico/png). No dedicated "wide capsule" —
  grids ARE the wide capsule replacement.
- Styles filter: `alternate, blurred, material, no_logo, white_logo`
  (VERIFIED in wrapper docs); `dimensions` filter param VERIFIED.
- Steam appid lookup built in: `GET /api/v2/grids/steam/<appid>` and
  `GET /api/v2/games/steam/<appid>`; name search
  `GET /api/v2/search/autocomplete/<term>`.
- Response includes `score`, `url`, `thumb`, `author` (attribution data).
- Rate limit: **UNVERIFIED** — free key has limits (429s reported by
  consumers, e.g. github.com/wesellis/vapor notes), no published number.
- Licensing: fan-submitted art for personal library customization; author
  metadata provided. Exact reuse license for a distributed client:
  UNVERIFIED — safest is opt-in "fetch fan art from SGDB" setting.

## 3. IGDB (Twitch)

Docs: https://api-docs.igdb.com (CF-blocked for bots; content mirrored via
context7 /websites/api-docs_igdb).

- Key: free — Twitch account with 2FA, register confidential app in Twitch
  Developer Portal → `POST id.twitch.tv/oauth2/token` (client_credentials) →
  send `Client-ID` + `Authorization: Bearer`. VERIFIED.
- Rate limit: **4 req/s, max 8 concurrent** (docs). VERIFIED.
- Covers: `POST api.igdb.com/v4/covers` with `game,<id>`; image URL
  `https://images.igdb.com/igdb/image/upload/t_{size}/{hash}.jpg`; sizes
  include `cover_small` 90x128, `cover_big` 264x374 (~2:3), `720p`, `1080p`;
  append `_2x` for retina. VERIFIED (docs).
- No transparent game logos (only size name `logo_med` 284x160 exists) →
  IGDB = cover fallback only.

## 4. RAWG

Docs: https://rawg.io/apidocs (VERIFIED fetch).

- Key: free, non-commercial ONLY + **required backlink** on pages using
  data/images; 20,000 req/month free. Business $149/mo (commercial, 50k).
- Art fields: `background_image`, `background_image_additional`,
  `short_screenshots` (field names from docs; live call not made — no key).
  No logos, no capsules. Verdict: poor fit for this client.

## 5. PCGamingWiki

MediaWiki Action API + Cargo (`/w/api.php`, cargoquery on `Infobox_game`,
field `Steam_AppID`). Metadata (engine, OS, fixes, appid mapping) — **no art
API worth using** (sparse local boxshots). Live cargoquery from this box =
Cloudflare-challenged → UNVERIFIED live; endpoint documented at
https://www.pcgamingwiki.com/wiki/PCGamingWiki:API. Use as appid/metadata
cross-check only.

## 6. Others worth naming

- **GiantBomb** (giantbomb.com/api): free key, 200 req/resource/hour
  (VERIFIED pricing data). Game images incl. logo/splash concept art but
  inconsistent per-game. Secondary.
- **MobyGames** (mobygames.com/info/api): **PAID** — Hobbyist $9.99/mo
  (1 req/s, non-commercial), commercial $99.99–$499.99/mo; legacy free
  non-commercial 720 req/h via request form. Cover images + screenshots.
  Flag: cost. VERIFIED from their subscribe page.
- **TheGamesDB** (api.thegamesdb.net): free key, ~1000 queries/month;
  has `clearlogos`, banners, fanart — console/emu-focused, weak modern-PC
  coverage. Secondary for logos only.

## Comparison table

| Provider | 2:3 cover | Transparent logo | Wide hero | Capsule | Key | Rate limit | Quality | Licensing/cost |
|---|---|---|---|---|---|---|---|---|
| Steam CDN | ✅ 600x900 (`_2x`) | ✅ logo(_2x).png RGBA | ✅ 1920x620 | ✅ 4 sizes | none | undocumented (cache) | official, consistent | Valve marketing assets; tolerated hotlink; UNVERIFIED license |
| SteamGridDB | ✅ grids 600x900 | ✅✅ logos (best) | ✅ heroes | ✅ grids 460x215/920x430 | free Bearer | exists, number UNVERIFIED | fan art, high peaks | fan-submitted, personal use; attribution data in API |
| IGDB | ✅ 264x374 (+_2x, 1080p) | ❌ | ❌ | ❌ | free (Twitch OAuth) | 4 req/s, 8 conc. | clean covers | free via Twitch dev terms |
| RAWG | ⚠️ background 16:9 only | ❌ | ⚠️ screenshot-ish | ❌ | free | 20k/mo free | varies | non-commercial + backlink; $149/mo commercial |
| PCGamingWiki | ❌ | ❌ | ❌ | ❌ | none | MediaWiki etiquette | — metadata | CC BY-NC-SA (wiki) |
| GiantBomb | ⚠️ boxart small | ⚠️ concept art | ⚠️ splash | ❌ | free | 200/res/hr | varies | Fandom terms; non-commercial-ish |
| MobyGames | ✅ covers | ❌ | ❌ | ❌ | **PAID** | 1 req/s hobbyist | great scans | $9.99–$499.99/mo |
| TheGamesDB | ✅ boxart | ✅ clearlogos | ✅ banners | ⚠️ | free | ~1000 q/mo | console-heavy | fan-submitted |

## Decisions (quality-first, cost-aware)

- **Cover 2:3** → Steam `library_600x900_2x.jpg` primary (true 600x900, no
  key, official). Never use plain `library_600x900.jpg` when `_2x` exists
  (300x450 trap). Fallback: IGDB `t_1080p` cover. Opt-in: SGDB grids 600x900.
- **Logo** → Steam `logo_2x.png` → `logo.png` → SGDB logos (SGDB is
  best-in-class when official is missing/ugly; it's the community standard
  for exactly this). Cache both variants.
- **Hero banner** → Steam `library_hero.jpg` (1920x620) + `library_hero_blur`
  for backdrop. Fallback: SGDB heroes. Wide list thumb: `capsule_616x353`.
- **Capsule/thumbnail** → `capsule_231x87` (list), `capsule_sm_120` (dense),
  `capsule_184x69` (compact), `header.jpg` (460x215 legacy).
- Cache everything on disk keyed by `<appid>/<file>`; fetcher must 404-fall
  through the chain (not all files exist for all apps).

## UNVERIFIED register (do not build on these without testing)

1. online-fix `/ext/` final store URL (JS/timer-gated) → appid extraction
   needs headless fetch or token decode. appid itself confirmed (1144200).
2. SGDB exact rate-limit number.
3. Steam CDN / appdetails rate limits (undocumented).
4. Valve legal stance on third-party hotlinking (tolerated by practice).
5. RAWG field names (docs-read, no live call).
6. PCGW cargoquery (Cloudflare-blocked from this IP).
7. SGDB fan-art license for redistribution inside a shipped client.
