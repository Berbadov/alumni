# Design — Reference Synthesis

How the reference sites and posters become our system. `BRAND.md` is the law;
this file explains the lineage and the reusable patterns. When they disagree,
`BRAND.md` wins.

| Field | Value |
|---|---|
| Version | 1.0 — 2026-11-18 |
| Sources | Mistral, Google for Developers, Inception Labs, Kriko, Fortiche, Polish culture poster |
| Output | Pattern library for `frontend/` |
| Palette | Pistachio + editorial wine/rose (`BRAND.md` §4) |

---

## 1. Rules of the merge

1. **Take principles, not pixels.** No reference's brand colour, logo, font, or
   layout ships literally.
2. **One reference per decision.** A card grid comes from Google, a reel from
   Fortiche, a source line from Kriko. Do not collage five sites into one screen.
3. **The palette is closed.** New references may suggest *roles* (a red action, a
   warm paper, a dark band), never new hues. All reds map to oxblood/mulberry/
   rose/blush; all greens to mint/hay/sage/signal.
4. **Two fonts only.** The references' mono and custom display faces are studied
   and rejected; Inter labels carry the technical register.
5. **Motion stays on budget.** Each reference contributes at most one motion
   idea, and a page still ships one ambient loop plus one reveal.

---

## 2. Source matrix

| Source | Take | Leave |
|---|---|---|
| **Mistral** | Warm paper canvas, oversized statement, node-chip diagrams, banded accordions, dense technical micro-labels | Orange flame accent, ALTMistral display face, mega-menu density |
| **Google for Developers** | Docs-grade clarity, product card garden, search and breadcrumb prominence, disciplined focus states, 12-column rhythm | Google blue, Material elevation, rainbow product logos |
| **Inception Labs** | Bone + deep ink contrast, electric accent as proof, latency/benchmark instrumentation, "every millisecond" honesty | Cyan/teal accent, Framer scroll effects, benchmark clutter |
| **Kriko** | Evidence beside every claim, numbered findings, severity/status chips, plain-spoken utility | Cobalt and powder blue, pixel-art product shelf, dense legal copy |
| **Fortiche** | Cinematic chapter bands, 21:9 reel frames, hot red title cards, frame-step motion, restrained grain | Custom hand-drawn font, full video hero, all-dark site, heavy texture |
| **Culture poster** | Wine/rose spot colour, halftone field, editorial side rail, circular arrow, two-tone headline | Condensed grotesk, full-bleed duotone photography, vertical body copy |

### What each source actually uses

Verified from the live sites, not from memory:

| Source | Type | Colour | Structure |
|---|---|---|---|
| Mistral | ALTMistral, Inter, Space Mono / DM Mono | Paper `#F9F9FB` / `#F5F4EF`, ink `#07070B`, orange `#FF6523` | Full-bleed bands, giant statements, node chips |
| Google for Developers | Google Sans Flex, Google Sans Code | White, Google blue `#1A73E8`, product colour | Card garden, docs nav, search, breadcrumbs |
| Inception Labs | KMR Apparat Book, Inter, Fragment Mono / Geist Mono | Bone `#FAF8F0`, ink `#041414`, teal `#17C4C4`, orange `#F0783D` | Benchmark strips, speed claims, monospace labels |
| Kriko | Inter, Chivo Mono | Paper `#EDEDEA`, ink `#0E0F11`, navy `#1F3A63`, powder `#A9C0E6` | Numbered findings, source links, severity tags |
| Fortiche | Custom `skribblughregular` | Black `#141414`, off-white `#ECEAE9`, red `#FF364A` | Cinematic reels, title cards, chaptering |

---

## 3. Translation by layer

### 3.1 Colour translation

| Reference move | Our token | Role |
|---|---|---|
| Mistral warm paper | `--bone #E3E2DF` | Alternating bands, archival paper |
| Inception deep ink | `--terminal #0B120E` | Data/cinematic band |
| Fortiche hot red | `--rose #EE4C7C` | Display emphasis, hover rule |
| Poster oxblood field | `--oxblood #5D001E` | Cellar band, deep editorial |
| Poster rose type | `--mulberry #9A1750` | Links, actions, body-safe accent |
| Poster pale rose | `--blush #E3AFBC` | Chips, hover, duotone highlight |
| Inception electric teal | `--signal #42D674` | Live data only — remapped, not copied |
| Mistral orange | *rejected* | Heat is carried by mulberry/rose |
| Google blue, Kriko navy | *rejected* | Information states use ink/sage |
| Poster bone | `--bone`, `--white` | Canvas |

**Rule:** green means live, red means voice. A reference that uses one hot accent
for everything is reduced to a single role here.

### 3.2 Type translation

| Reference move | Ours |
|---|---|
| Mistral/Google oversized display | IBM Plex Serif 400/500 at the same scale; serif instead of grotesk |
| Mistral/Inception mono labels | Inter 600 uppercase, `0.16em`, `tabular-nums` |
| Fortiche hand-drawn title | Plex Serif italic, one word, mulberry |
| Poster condensed headline | Plex Serif, tight tracking, never condensed |
| Google Sans Code | *rejected*; code blocks may use a system mono, never brand type |

### 3.3 Space translation

| Reference move | Ours |
|---|---|
| Mistral banded sections | Full-bleed chapter bands, 96px padding |
| Google 12-column card garden | 3-up product plate grid, 24px gutters |
| Kriko numbered evidence | `01`/`02` evidence blocks with source lines |
| Poster side rail | Vertical editorial rail with scroll progress |
| Fortiche 21:9 frames | Reel plates, 1px border, chapter tag |
| Inception stat strip | Instrument strip with serif numerals |

---

## 4. Pattern library

Each pattern lists anatomy, tokens, motion, and where it belongs. Sizes assume
desktop; halve horizontal padding and collapse to one column under 768px.

### 4.1 Chapter band

**From:** Mistral bands, Fortiche chaptering, poster colour fields.

- Full-bleed `Terminal` or `Cellar`, minimum 60vh, 96px vertical padding.
- Top rail: `01 / CHAPTER` left, `label` meta right, 1px line under.
- Display-2 serif, left-aligned or centered; one bone italic word.
- Optional instrument strip or reel below; nothing else.
- Ends with a 1px rule and the next chapter index.

**Motion:** 420ms chapter wipe on entry; no parallax.

**Use:** Home proof, About story, quote interludes. Never a form.

### 4.2 Reel frame

**From:** Fortiche cinematic reel, poster image panel.

- 21:9 (or 4:5 portrait) plate, 0 radius, 1px `--line`.
- Duotone image: blush→oxblood or hay→ink.
- `label` top-left `REEL / 01`, timecode bottom-right.
- 3% grain overlay; image scales 1.00→1.03 over 18s.
- Static poster fallback; no essential text inside the image.

**Motion:** reel drift only; one per page, pause offscreen.

**Use:** Home hero side panel, About, event recaps.

### 4.3 Editorial rail

**From:** Culture poster's vertical "By Culture.pl" rail.

- 1px vertical rule 24px from the viewport edge, full content height.
- Rotated `label` metadata (`ALUMNI / 2026`, `DIRECTORY`), reading bottom-up.
- Scroll progress: a mulberry 1px line and 3×3 node.
- Desktop only; hide under 1024px.

**Motion:** scroll-linked progress, no easing.

**Use:** Editorial pages. Never on dense tables or forms.

### 4.4 Halftone field

**From:** Poster's dotted patch; Mistral's technical texture.

- 4px dots, 8px pitch, square clip, 0 radius.
- Blush at 30% or mulberry at 12%; maximum 240×240px.
- Sits behind a label, at a hero edge, or beside a chapter index.
- Never over text, faces, or a control.

**Motion:** 8px diagonal drift over 20s, alternate. Use instead of the grid.

**Use:** One per page.

### 4.5 Node diagram

**From:** Mistral's "No one else should hold the strings" chip rows.

- Centered statement above, chips in 3–4 rows, 1px connectors with 3×3 nodes.
- Chip: 12×16px padding, 1px `--line-strong`, `ui` type, optional 16px icon.
- One chip may be "live" with a signal dot; the rest are ink.
- Connectors elbow at 90°, radius 2px.
- On Cellar/Terminal, hairlines use the surface line token.

**Motion:** hairline draw 800ms + live node pulse; once on entry.

**Use:** Home method section, About the project, architecture explainers.

### 4.6 Product garden

**From:** Google's developer product grid, Kriko's utility cards.

- 12-col, 3-up (2-up tablet, 1-up mobile).
- Plate, 0 radius, 1px `--line`, 24px padding, 32px gap.
- 24px ink icon, serif h3, one-line proof in body, 20px circular arrow.
- Hover: lift 2px, border ink, corner ticks extend.
- No shadows, no coloured card backgrounds; the icon is the only accent.

**Motion:** 160ms hover geometry; no entrance stagger beyond the section reveal.

**Use:** Home sections, Jobs categories, About links.

### 4.7 Instrument strip

**From:** Inception's speed proof, Mistral's stat rows.

- 3–4 items in a row, separated by 1px vertical hairlines.
- Number: Plex Serif 48–64px, tabular; caption `label`; delta `data`.
- One optional signal dot on the live metric.
- No boxes, no card backgrounds.

**Motion:** counter 400ms once on entry; no rolling after.

**Use:** Home, Alumni header, Jobs header.

### 4.8 Evidence block

**From:** Kriko's numbered findings with sources.

- Row: `01` label, finding in body, source in `label`, status chip right.
- 1px `--line` dividers; 24px vertical padding.
- Source format: `SOURCE / REGISTRAR · 2026-11-18`.
- Status: `verified` (signal dot), `unverified` (ink dot), `missing` (dotted rule).
- On mobile, source wraps under the finding, never truncates.

**Motion:** source line staggers in 120ms after the finding; visible by default.

**Use:** Alumni profile, imported records, job verification, About claims.

### 4.9 Docs rail

**From:** Google for Developers' docs navigation.

- 12-col: 3-col sticky index, 9-col content.
- Breadcrumb `label` above the title; search field full width above the grid.
- Index items: 44px rows, 1px divider, active = ink + mulberry 2px left rule.
- Mobile: index collapses to a select above the content.

**Motion:** none beyond focus and 120ms hover.

**Use:** About, Help, long Jobs detail.

### 4.10 Circular arrow

**From:** Poster's "Enter the Guides" ring.

- 48px ring (56px for a primary link), 1px ink or mulberry, 20px arrow.
- Label sits outside the ring, `ui` type.
- Hover: ring 2px, arrow rotates -45°, 300ms.
- Never spins continuously; never the only affordance without a text label.

**Use:** Section entries, "Browse alumni", "Read the archive".

### 4.11 Two-tone headline

**From:** Poster's red second line; Mistral's statement scale.

- Base line(s) ink serif; one word or one line mulberry italic.
- On white, a 64×2px rose rule may sit above the heading.
- On Cellar, emphasis is bone italic with a blush rule.
- Maximum two lines; never colour the whole heading.

**Use:** Home, About, chapter openers.

### 4.12 Source line

**From:** Kriko's receipt culture.

- `label` type, ink-muted, prefixed `SOURCE /`.
- Sits under instrument strips, evidence blocks, and imported tables.
- One source per claim; multiple sources get separate numbered lines.
- Links are mulberry with a 1px underline; external icon 12px.

**Use:** Anywhere a number is derived rather than directly stored.

---

## 5. Page recipes

### Home

1. Paper hero: `01 / DIRECTORY` label, two-tone display, lede, mulberry CTA +
   circular arrow secondary, halftone at the right edge, editorial rail.
2. Instrument strip: alumni count, years, live jobs.
3. Product garden: Alumni, Jobs, About (3 plates).
4. Terminal chapter band: node diagram + one live node.
5. Footer on Paper.

### Alumni

1. Docs rail + title.
2. Instrument strip: total, verified, missing data.
3. Data table with `label` headers, blush hover, signal live dot.
4. Empty state on Paper with one action.
5. Profile: evidence blocks for imported fields.

### Jobs

1. Heading + one-sentence lede.
2. Product garden for categories, or a table for the full list.
3. Evidence block for verification and source.
4. One mulberry primary action (`Post a job`).

### About

1. Cellar chapter band with reel.
2. Two-tone headline + body on Paper.
3. Editorial rail with scroll progress.
4. Evidence blocks for claims.
5. Closing quote in serif italic on Cellar.

---

## 6. Motion translation

| Source | Move | Our budget |
|---|---|---|
| Fortiche | Chapter wipe, reel drift, title card | 420ms wipe; 18s reel; one per page |
| Inception | Counter, benchmark bar draw | 400ms count; 800ms bar; once |
| Mistral | Node diagram draw, live pulse | 800ms draw; 1.8s pulse |
| Kriko | Source reveal | 120ms stagger |
| Google | Focus/scroll discipline | 120ms hover, native scroll |
| Poster | Arrow rotate, halftone drift | 300ms rotate; 20s drift |

Do not stack these. A page gets one ambient (grid **or** halftone **or** reel)
and one entrance (reveal **or** draw **or** wipe).

---

## 7. Rejected moves

- Reference brand colours: Mistral orange, Google blue, Inception teal, Kriko
  cobalt, Fortiche red `#FF364A`. Our reds and green are fixed.
- Reference fonts: ALTMistral, Google Sans, KMR Apparat, Fragment Mono, Chivo
  Mono, `skribblughregular`.
- Mono as a brand voice; code may use a system mono outside the brand layer.
- Full-bleed video heroes, autoplay audio, heavy grain, 3D renders.
- Material elevation, glassmorphism, gradient blobs, rounded 24px+ cards.
- Pixel-art shelves, emoji icons, rainbow product logos, mega-menus.
- Infinite scroll and spinner-only loading states.

---

## 8. Review questions

- Can you name the one pattern this screen is built from?
- Is green doing data and red doing voice, with no crossover?
- Would the screen still read as our brand without the reference imagery?
- Is there one ambient loop, and does it survive reduced motion?
- Does every derived number have a source line?
- Is the type still Plex Serif + Inter, with no borrowed face?
- Does the palette use only `BRAND.md` tokens?
