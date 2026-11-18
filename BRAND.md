# Brand — Pistachio Dream

**The Living Ledger.** A botanical archive printed in wine ink. Growth-green
surfaces, editorial-red voice, ink structure, and a grid that never sits still.

| Field | Value |
|---|---|
| Brand name | Pistachio Dream |
| Concept | The Living Ledger |
| Version | 1.1 — 2026-11-18 |
| Scope | `frontend/`, docs, decks, project identity |
| Palette | Pistachio (green) + Editorial ink (wine/rose) |
| Reference rationale | `DESIGN.md` |

The two palettes have different jobs. **Green is data** — surfaces, growth, live
state. **Red is voice** — headlines, links, action, cinema. **Ink is structure** —
text, rules, focus. A view that mixes the jobs is off-brand.

The synthesis of Mistral, Google for Developers, Inception Labs, Kriko, Fortiche,
and the wine/rose poster is documented in `DESIGN.md`: what each contributes, the
pattern library built from them, and what we deliberately reject.

---

## 1. Concept

An alumni archive grows like a planting and is catalogued like a printed ledger.
Every row is a node in a network that keeps growing after graduation. The visual
system makes that visible:

- **Cultivated** — pistachio greens as fields and surfaces, never as decoration.
- **Inked** — mulberry, rose, and oxblood as the editorial voice: emphasis,
  captions, actions, image duotone.
- **Engineered** — 1px hairlines, coordinates, ticks, and annotations show the
  machinery.
- **Alive** — the grid drifts, halftone fields breathe, reels move, live nodes
  pulse. Motion is proof the record is current.

### Principles

1. **Green is data. Red is voice. Ink is structure.** Apply this before any other
   colour decision.
2. **Structure is visible.** If a line exists, it means something: a divider, a
   connection, a boundary. Never decorate with borders.
3. **Two accents, never competing.** Beyond the logo and inline links, one
   editorial-red gesture and one live-green system per view. Red never encodes
   status; green never decorates a headline.
4. **Slow ambient, fast interface.** Backgrounds breathe over 18–48s. Controls
   answer in 120–240ms. Nothing bounces.
5. **Paper first, dark for chapters.** The app is light. Ink (`Terminal`) and
   oxblood (`Cellar`) bands separate chapters; they are not a theme setting.
6. **Type does the talking.** IBM Plex Serif for voice, Inter for instruction.
   No third family, no condensed face, no emoji in UI.

---

## 2. Brand at a glance

| Layer | Decision |
|---|---|
| Canvas | White `#FFFFFF`, archival bone `#E3E2DF` for bands |
| Data surfaces | Mint `#80EF80`, hay `#E3F0A3`, sage `#BADBA2` |
| Live | Signal `#42D674` — dot, pulse, success only |
| Voice | Mulberry `#9A1750` links/actions, rose `#EE4C7C` display emphasis |
| Depth | Oxblood `#5D001E` Cellar bands; near-black `#0B120E` Terminal bands |
| Type | IBM Plex Serif 400/500/600 + italic; Inter 400/500/600 |
| Shape | 1px hairlines, 0px plates, 20px slabs, pill actions |
| Motion | One ambient loop + one reveal per view, reduced-motion safe |

---

## 3. Voice

An archivist with an engineer's notebook: calm, exact, quietly warm. Evidence
beside every claim. Sentence case in UI; uppercase only for annotations.

| Do | Don't |
|---|---|
| `412 alumni. 9 years. One directory.` | `Welcome to our amazing alumni community! 🎉` |
| `No records match these filters.` | `Oops! Nothing here :(` |
| `Synced 18 Nov 2026, 10:42.` | `Last updated just now!!` |
| `Source: registrar · 2026-11-18` | `Trust us, it's accurate.` |
| `Degree not recorded.` | `N/A` |

Rules:

- Buttons are verbs: `Add alumnus`, `Browse jobs`, `Clear filters`.
- Errors say what happened and what to do: `Could not save. Check the graduation
  year and try again.`
- Numbers are facts, not hype: no `+`, no `K` abbreviations under 10,000.
- A derived or aggregated number gets a source line in `label` type.
- No exclamation marks, no emoji, no motivational filler.

---

## 4. Colour

### 4.1 Growing season — the green family

| Token | Hex | Name | Role |
|---|---|---|---|
| `--mint` | `#80EF80` | Bloom | Major surfaces, hero blocks, active tiles |
| `--hay` | `#E3F0A3` | Hay | Section washes, secondary fills, chips |
| `--sage` | `#BADBA2` | Sage | Muted blocks, chart fills, decorative grid |
| `--signal` | `#42D674` | Signal | Live state only: dot, pulse, live tag |

### 4.2 Editorial ink — the red family

| Token | Hex | Name | Role |
|---|---|---|---|
| `--oxblood` | `#5D001E` | Oxblood | Cellar bands, deepest editorial surface |
| `--bone` | `#E3E2DF` | Bone | Archival paper, alternating bands |
| `--blush` | `#E3AFBC` | Blush | Soft fills, hover, chart, duotone highlight |
| `--mulberry` | `#9A1750` | Mulberry | Voice: links, primary action, headline accent, rules |
| `--rose` | `#EE4C7C` | Rose | Large display emphasis, hover rule, dot field |

### 4.3 Derived neutrals

| Token | Hex | Role |
|---|---|---|
| `--white` | `#FFFFFF` | Default canvas |
| `--ink` | `#101A13` | All text, functional borders, focus ring |
| `--ink-muted` | `#4F6056` | Secondary text — safe on white, bone, mint |
| `--line` | `#C9D6C3` | Decorative grid and hairlines |
| `--line-strong` | `#6E8074` | Functional borders: inputs, dividers, buttons |
| `--terminal` | `#0B120E` | Dark data band |
| `--terminal-line` | `rgba(227,226,223,0.16)` | Hairlines on Terminal |
| `--cellar-line` | `rgba(227,175,188,0.24)` | Hairlines on Cellar |
| `--destructive` | `#B4342A` | Real errors and destructive actions only |

### 4.4 Roles — the two-accent law

| Colour | Means | Never |
|---|---|---|
| Signal `#42D674` | Live, current, success | Headline, large fill, decorative border |
| Mulberry `#9A1750` | Editorial voice, link, primary action | Status badge, chart series |
| Rose `#EE4C7C` | Display emphasis, hover rule, dot field | Body text, small text, error state |
| Oxblood `#5D001E` | Cinematic band, deep editorial surface | Status, chart series |
| Mint / hay / sage | Data surfaces and fields | Error, live state |

- **Live** is always signal dot + the word: `live`, `current`, `verified`.
- **Error** is `--destructive` plus an icon and a written reason. Editorial red is
  not an alarm.
- **One** editorial-red gesture per view: a CTA, a headline accent, or a rule.
  The logo and inline links are baseline, not gestures. A rose headline does not
  get a second rose rule beside it.

### 4.5 Ratio

Aim for roughly: **55% white, 15% bone, 14% mint, 7% hay, 4% sage/lines, 3% ink,
1.5% editorial red, 0.5% signal.** Red is ink with temperature, not a second
theme. Green fields are mint, never signal.

### 4.6 Pairing rules

- **Ink on every light green.** `#101A13` on signal is 9.4:1; white on signal is
  1.9:1 and forbidden.
- **Mulberry on paper.** 8.0:1 on white, 6.2:1 on bone — safe for body links.
- **Rose is large only.** 3.5:1 on white passes large-text/UI; 2.7:1 on bone does
  not, so rose is decoration there, never text.
- **Blush carries ink or oxblood**, not rose.
- **On Cellar:** bone text (10.9:1), blush secondary (7.5:1), signal only as a live
  dot. Rose on oxblood is 2.3:1 — decorative only.
- **No gradients between hue families.** A single mint→hay wash at 8–12% is the
  only permitted gradient.

Measured pairs:

| Pair | Ratio | Use |
|---|---|---|
| Ink on white | 17.8:1 | Body, headings |
| Ink on bone | 13.8:1 | Body on paper |
| Ink on mint | 12.4:1 | Text on hero blocks |
| Ink on blush | 9.4:1 | Chips, callouts |
| Mulberry on white | 8.0:1 | Links, small accent |
| Mulberry on bone | 6.2:1 | Links on paper |
| White on mulberry | 8.0:1 | Primary button |
| Bone on oxblood | 10.9:1 | Cellar body |
| Blush on oxblood | 7.5:1 | Cellar secondary |
| Rose on white | 3.5:1 | Large display / UI only |
| Rose on bone | 2.7:1 | Decorative only |
| Muted on bone | 5.2:1 | Secondary text |
| Line-strong on bone | 3.2:1 | Functional borders |
| Signal on Terminal | 10.0:1 | Live accents |
| White on signal | 1.9:1 | **Forbidden** |
| Rose on oxblood | 2.3:1 | **Forbidden text** |

---

## 5. Typography

### 5.1 Families

| Role | Family | Weights | Notes |
|---|---|---|---|
| Voice | **IBM Plex Serif** | 400, 500, 600, 400 italic | Headings, hero, quotes, big stats |
| Instruction | **Inter** | 400, 500, 600 | Body, UI, labels, tables |

Self-host via npm (`@fontsource/ibm-plex-serif`, `@fontsource-variable/inter`) —
no runtime Google Fonts request, works offline and in Docker. The reference sites'
mono faces are deliberately rejected; Inter tabular numerals carry annotations.

### 5.2 Scale

| Token | Size | Family / weight | Line height | Tracking | Use |
|---|---|---|---|---|---|
| `display-1` | `clamp(3.5rem, 8vw, 6.5rem)` | Plex Serif 400 | 0.92 | -0.03em | Hero statement |
| `display-2` | `clamp(2.75rem, 5vw, 4rem)` | Plex Serif 400 | 0.98 | -0.025em | Page opener |
| `h1` | `clamp(2.25rem, 4vw, 3.25rem)` | Plex Serif 500 | 1.05 | -0.02em | Page title |
| `h2` | `2rem` | Plex Serif 500 | 1.15 | -0.015em | Section |
| `h3` | `1.375rem` | Plex Serif 500 | 1.3 | -0.01em | Subsection, card title |
| `quote` | `1.5rem` | Plex Serif 400 italic | 1.45 | 0 | Pull quote |
| `lede` | `1.25rem` | Inter 400 | 1.55 | -0.01em | Intro paragraph |
| `body` | `1rem` | Inter 400 | 1.6 | 0 | Default |
| `ui` | `0.875rem` | Inter 500 | 1.4 | 0 | Buttons, nav, controls |
| `label` | `0.6875rem` | Inter 600 uppercase | 1.2 | 0.16em | Annotations, table heads |
| `data` | `0.875rem` | Inter 500, `tabular-nums` | 1.4 | 0 | Tables, counts |

### 5.3 Two-tone headline

The reference posters put one line in colour. We translate that into serif:

- One word or one line per heading may be **Plex Serif italic**.
- Colour it **mulberry** (any size) or **rose** (white canvas, 32px+ only).
- The rest of the heading stays ink. Never colour a whole heading.
- On Cellar, the emphasis is **bone italic**, with a **blush** rule; rose is
  decoration only.

### 5.4 Rules

- Serif never below 20px. Under that, it becomes texture instead of voice.
- Inter never letterspaced except the `label` style.
- Only real serif italics. Never synthesize oblique, never condense.
- Display type may use `text-wrap: balance`; body uses `text-wrap: pretty`.
- Body measure 60–70ch; display measure 12–20ch.
- Headings stay sentence case. Uppercase is reserved for annotations.
- Numbers in tables and counters use `font-variant-numeric: tabular-nums`.

---

## 6. Logo and mark

**Wordmark.** `alumni.` in IBM Plex Serif 500, lowercase, `-0.02em`, ink. The
period is **rose** — the voice accent. Never letterspace it, never set it in
Inter.

**Mark.** A 2×2 ledger tile: three 11px squares outlined 1px ink, one (top-right)
filled **rose**; 2px gutters; 24px total. Signal green is reserved for live data,
so it never appears in the logo.

- Clearspace: one tile (24px) on all sides.
- Minimum: wordmark 96px wide, mark 20px.
- On Terminal: wordmark bone, dot rose, outlines bone.
- On Cellar: wordmark bone, dot blush, outlines bone.
- Never: outline the wordmark, add shadows, stretch, rotate, place on a photo, or
  use signal green in the mark.

---

## 7. Space and grid

- **Base unit:** 8px. All spacing is a multiple; 4px only for optical nudges.
- **Columns:** 12. Gutter 24px. Page margin 24px mobile, 48px desktop.
- **Max width:** 1280px. Long-form text column: 720px.
- **Checker module:** 96px desktop / 64px mobile, 1px `--line` at 45% opacity.
- **Halftone field:** 4px dots on an 8px pitch, one colour at low opacity. Maximum
  240px wide, sits behind a label or the hero's edge.
- **Ledger frame:** 1px frame with 6px corner ticks inset 8px. Optional on dense
  app pages, required on editorial pages.
- **Editorial rail:** a 1px vertical rule 24px from the viewport edge with rotated
  `label` metadata (`ALUMNI / 2026`, `01 — DIRECTORY`). Desktop only.
- **Chapter index:** `01`, `02` in serif tabular figures beside a `label` title.
- **Reel ratios:** 21:9 for cinematic bands, 4:5 for portrait archive plates.
- **Asymmetry:** prefer 5/7 or 4/8 splits. Centred layouts are for one hero
  statement only.
- **Density:** generous. Whitespace is the paper the record is printed on.

---

## 8. Shape and borders — concrete

Four radius roles, two families: containers are plates or slabs; interaction
surfaces are fields or pills.

| Family | Radius | Use |
|---|---|---|
| **Plate** | `0px` | Data cells, reels, image frames, annotation boxes |
| **Slab** | `20px` | Large white content panels, modals, hero container |
| **Pill** | `999px` | Buttons, chips, status tags, circular links |
| **Field** | `10px` | Inputs, selects, small utility surfaces |

Border rules:

- Default stroke is **1px**. Icons 1.5px. Focus is 2px. Nothing else.
- Use `--line-strong` for anything functional. `--line` is decorative grid only.
- Borders are never neutral grey; they are tinted with the palette.
- **Corner ticks:** 6px right-angle arms, 1px, inset 8px, 32% ink.
- **Node ticks:** connectors end in 3×3px ink squares (2×2 on mobile).
- **Elbows:** right-angle connectors only. Corner radius 2px maximum.
- **Dividers:** solid 1px. Dashed/dotted means pending, empty, or estimated.
- **Circular arrow:** 48px ring, 1px ink or mulberry, 20px arrow. Hover: ring
  thickens to 2px, arrow rotates -45°, 300ms.
- **Reel frame:** 21:9 plate, 1px border, `label` chapter tag in the top-left and
  timestamp bottom-right. Never rounded.
- **Halftone patch:** square or rectangular, 0 radius, clipped, never a soft mask.
- **Shadows:** almost none. One permitted shadow is a hard plate offset on a
  pressed primary action: `3px 3px 0 var(--ink)`. No blurred drop shadows.
- **Hover geometry:** cards lift 2px, border moves from `--line` to `--ink`, corner
  ticks extend to 9px. No glow, no scale on cards.

---

## 9. Components

| Component | Spec |
|---|---|
| **Primary button** | Pill, 44px, mulberry fill, white label, `ui`. Hover oxblood; press `scale(0.98)`; 160ms. One per view. |
| **Secondary button** | Pill, transparent, 1px `--line-strong`, ink label. Hover blush fill. |
| **Ghost / link** | Mulberry label, 1px mulberry underline offset 4px. Hover underline rose. |
| **Input** | 48px, white, 1px `--line-strong`, 10px radius, 16px padding. Focus: 2px ink outline offset 2. |
| **Chip / tag** | Pill, 28px, hay or blush fill, `label` type, ink text. No shadow. |
| **Live tag** | Pill with 6px signal dot + lowercase word (`live`, `current`). Pulse only for genuinely live data. |
| **Status tag** | 6px ink dot + lowercase word. Colour is never the only signal. |
| **Panel (slab)** | White, 20px radius, 1px `--line`, 32px padding. Optional corner ticks. |
| **Panel (plate)** | Transparent or bone, 0 radius, 1px `--line-strong`, 24px padding, label top-left. |
| **Data table** | Header `label` + `--ink-muted`; row 56px; 1px `--line` dividers; hover blush at 35%; index `data` tabular; no zebra under 12 rows. |
| **Nav** | Wordmark left. Inter 500 links. Active = ink + 2px mulberry underline; idle = muted + ink hover. 64px, 1px bottom border. |
| **Footer** | 1px top border, `label` annotations: `ALUMNI / DIRECTORY` left, version + year right. |
| **Modal** | Slab 20px, white, 1px ink at 12%, no shadow. Backdrop `#0B120E` at 40% + 2px blur. Enter 8px rise, 200ms. |
| **Empty state** | Dotted 1px plate, centred label + one sentence + one action. No sad illustration. |
| **Skeleton** | Drifting checkerboard at 40% opacity, no shimmer sweep. |
| **Chapter band** | Full-bleed Terminal or Cellar, chapter index + serif title + `label` meta, 96px vertical padding. |
| **Reel** | 21:9 plate with one muted loop or still; 3% grain; chapter label; slow drift. |
| **Instrument strip** | 3–4 stats: serif 48px number, `label` caption, `data` delta, optional live dot. |
| **Evidence block** | Numbered lines (`01`, `02`), finding in body, source in `label`, status chip right. |
| **Product card** | Plate, icon + title + one-line proof + circular arrow; hover lifts 2px. 3-up desktop. |
| **Docs rail** | Sticky 12-col index left, content right, `label` breadcrumb above. Collapses to a select on mobile. |

Layout recipes that combine these patterns live in `DESIGN.md` §4.

---

## 10. Motion — alive

### 10.1 Principles

1. **Motion is required.** Every page carries at least one ambient motion; a
   completely static screen is off-brand.
2. **Motion is evidence, not decoration.** Grids drift, halftone breathes, reels
   move, live nodes pulse with real data.
3. **Ambient is slow.** Loops run 18–48s at low amplitude (4–16px, scale ≤1.03).
   If you notice it immediately, it is too fast.
4. **Interface is quick.** Hover 120ms, press 160ms, panel 240ms, reveal 640ms.
5. **No bounce, no elastic, no confetti.** Easings are exponential or linear.
6. **One focal loop per viewport.** Ambient texture plus one reveal. Never two
   competing animations.
7. **Every animated element has a resting state.** Reduced motion removes the
   loop, never the content.

### 10.2 Tokens

| Token | Value | Use |
|---|---|---|
| `--dur-tap` | `120ms` | Hover, colour, underline |
| `--dur-press` | `160ms` | Press, tick extension |
| `--dur-ui` | `200ms` | Menus, modals, toggles |
| `--dur-panel` | `420ms` | Chapter wipe, panel enter |
| `--dur-reveal` | `640ms` | Section and type reveals |
| `--dur-ambient` | `18s` / `24s` / `36s` / `48s` | Loops |
| `--ease-out` | `cubic-bezier(0.16, 1, 0.3, 1)` | Entrances, UI |
| `--ease-mech` | `cubic-bezier(0.65, 0, 0.35, 1)` | Wipes, mechanical slides |
| `--ease-linear` | `linear` | Marquees, drift, pulses |

### 10.3 Signature motions

**1. Drifting grid (ambient).** One checker layer moves exactly one 96px tile over
36s, linear, infinite. Opacity 5–7%. Seamless.

```css
@keyframes grid-drift {
  to { background-position: 96px 96px; }
}
.grid-live {
  background-image:
    linear-gradient(var(--line) 1px, transparent 1px),
    linear-gradient(90deg, var(--line) 1px, transparent 1px);
  background-size: 96px 96px;
  animation: grid-drift 36s linear infinite;
}
```

**2. Halftone drift (ambient).** A dot field moves 8px diagonally over 20s,
alternating, at 12% mulberry or 30% blush. Use instead of the grid, never both.

**3. Reel drift (cinematic ambient).** A 21:9 image scales 1.00→1.03 over 18s,
alternating `ease-in-out`; 3% grain overlay. Pause offscreen; one reel per page.

**4. Hairline draw (reveal).** SVG connectors use `stroke-dasharray` +
`stroke-dashoffset` to 0 over 800ms, staggered 60ms. Once per section, never hover.

**5. Live node pulse.** A 3px signal node emits a ring 3px→13px over 1.8s, opacity
0.6→0, staggered 240ms. Only on real live data.

**6. Type reveal.** 12px rise + opacity over 640ms, 60ms per line. The two-tone
italic word arrives 120ms after its line, in mulberry.

**7. Cursor field.** A 320px radial blush glow at 10% follows the pointer with a
0.08 lerp. Disabled on touch and under reduced motion.

**8. Marquee annotations.** `label` text in `--ink-muted`, 30s linear, pauses on
hover/focus; duplicate `aria-hidden`.

**9. Counter ticker.** Numbers roll to value over 400ms with tabular numerals,
once on entry. No count-up on every scroll.

**10. Chapter wipe.** A 1px bone rule sweeps across a Terminal/Cellar band over
420ms `--ease-mech`, revealing the next chapter title. One per page.

**11. Circular arrow.** Hover rotates the arrow -45° and thickens the ring over
300ms; the ring does not spin continuously.

**12. Rail progress.** A 1px mulberry line on the editorial rail tracks scroll;
node moves with it. Scroll-linked, no easing, no bounce.

**13. Source reveal.** Evidence blocks stagger their source line in 120ms after
the finding. On touch, the source is always visible.

### 10.4 Reduced motion

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 120ms !important;
    scroll-behavior: auto !important;
  }
  .grid-live, .halftone, .marquee, .cursor-field, .reel { animation: none !important; }
}
```

Content must be fully visible with motion off. Never hide elements that only
appear through animation.

### 10.5 Performance

- Animate `transform` and `opacity` only. Never `width`, `height`, `top`, `left`.
- One `requestAnimationFrame` loop for the cursor field; pause on
  `visibilitychange` and when offscreen (`IntersectionObserver`).
- Max two animated layers per section; cap device pixel ratio at 2 for canvas.
- Respect `prefers-reduced-motion` before starting any loop.
- Video/reel loops: muted, no controls, ≤8s, poster fallback, one per page.

---

## 11. Surfaces: Paper, Terminal, Cellar

**Paper** is the default: white or bone, ink text, green fields. **Terminal** is
near-black and carries data and cinematic interludes. **Cellar** is oxblood and
carries editorial interludes and quotes.

| Surface | Background | Text | Secondary | Accent | Live |
|---|---|---|---|---|---|
| Paper | white / bone | ink | ink-muted | mulberry / rose | signal |
| Terminal | `#0B120E` | bone | bone at 72% | hay / mint | signal |
| Cellar | `#5D001E` | bone | blush | blush, bone italic | signal |

Rules:

- Long copy stays on Paper. Terminal and Cellar are for short statements,
  diagrams, quotes, and reels.
- Terminal/Cellar never contain a white slab; contrast is the point.
- On Cellar, rose is decorative only (2.3:1); never set text in it.
- A page ends on Paper, never on a dark band.

---

## 12. Iconography and imagery

- 24px grid, 1.5px stroke, round joins, ink. Lucide is the base set.
- One accent element per icon maximum: mulberry for editorial, signal only for
  live. Never multicolour, never emoji.
- Custom pictograms are schematic: rack, node, folder, path, badge, archive box.
- **Duotone only:** blush→oxblood for editorial imagery, hay→ink for data
  imagery. No full-colour photography except scanned documents.
- Halftone dots may overlay image edges at 12–20%, never across faces.
- Reels are silent loops or stills in a 21:9 plate; 3% grain; one per page.
- Illustration is diagrammatic, not 3D: isometric tiles at 30°, 1px connectors,
  annotations. No photorealism, no gradient blobs.

---

## 13. Data and content formatting

| Case | Format | Example |
|---|---|---|
| Count | `Intl.NumberFormat('en-US')` | `1,248 alumni` |
| Label | uppercase, Inter 600, 0.16em | `GRADUATION YEAR` |
| Date in table | ISO 8601 | `2026-11-18` |
| Date in prose | `d MMM yyyy` | `18 Nov 2026` |
| Degree | Title case, no abbreviations | `BSc Computer Engineering` |
| Missing value | Explicit sentence, muted | `Not recorded.` |
| Status | lowercase word + dot | `live`, `open`, `draft` |
| Source | `SOURCE / ORIGIN · DATE` | `SOURCE / REGISTRAR · 2026-11-18` |
| Pluralisation | Spell the singular/plural | `1 alumnus` / `412 alumni` |

Evidence is part of the brand: aggregated counts, verification badges, and
imported records carry a numbered source line in `label` type. Empty states name
the filter and the exit: `No alumni in 2027. Clear filters.`

---

## 14. Accessibility

- Body text ≥ 4.5:1; large text and UI boundaries ≥ 3:1.
- Mulberry is the only red safe for small text on paper. Rose is large/decorative.
  Never set rose on bone, never set rose text on oxblood.
- Focus is always visible: 2px ink outline, 2px offset. A 1px signal edge is
  optional decoration; the ink ring is the indicator.
- Hit targets ≥ 44×44px, including icon-only buttons.
- Status uses dot + word, never colour alone.
- Body links keep mulberry labels and a visible underline; hover changes the rule
  to rose, never the text.
- Motion respects `prefers-reduced-motion`; ambient loops stop.
- Terminal/Cellar body text ≥ 7:1. No low-contrast green body copy.
- Tables use real `<th scope>` and a caption; annotations are not the only labels.
- Reels have a static poster and no essential information inside the loop.

---

## 15. Tokens

Target state for `frontend/src/index.css`. It was implemented on 2026-11-18;
the `@theme` block, font imports, and pattern classes live in that file and
`frontend/src/main.tsx`.

```css
:root {
  color-scheme: light;

  /* surfaces */
  --background: #ffffff;
  --foreground: #101a13;
  --card: #ffffff;
  --card-foreground: #101a13;
  --popover: #ffffff;
  --popover-foreground: #101a13;
  --white: #ffffff;
  --bone: #e3e2df;
  --paper: #e3e2df;
  --ink: #101a13;
  --ink-muted: #4f6056;
  --terminal: #0b120e;
  --terminal-line: rgba(227, 226, 223, 0.16);
  --cellar: #5d001e;
  --cellar-line: rgba(227, 175, 188, 0.24);

  /* growing season */
  --mint: #80ef80;
  --hay: #e3f0a3;
  --sage: #badba2;
  --signal: #42d674;
  --signal-hover: #35c468;

  /* editorial ink */
  --oxblood: #5d001e;
  --blush: #e3afbc;
  --mulberry: #9a1750;
  --mulberry-hover: #5d001e;
  --rose: #ee4c7c;

  /* actions */
  --primary: #9a1750;
  --primary-foreground: #ffffff;
  --secondary: #e3f0a3;
  --secondary-foreground: #101a13;
  --muted: #e3e2df;
  --muted-foreground: #4f6056;
  --accent: #e3afbc;
  --accent-foreground: #101a13;
  --destructive: #b4342a;

  /* borders and focus */
  --border: #c9d6c3;
  --input: #6e8074;
  --ring: #101a13;
  --line: #c9d6c3;
  --line-strong: #6e8074;

  /* shape — raw values, mapped to radius utilities in @theme below */
  --radius: 1.25rem;
  --shape-plate: 0px;
  --shape-field: 10px;
  --shape-pill: 999px;

  /* motion */
  --dur-tap: 120ms;
  --dur-press: 160ms;
  --dur-ui: 200ms;
  --dur-panel: 420ms;
  --dur-reveal: 640ms;
  --ease-out: cubic-bezier(0.16, 1, 0.3, 1);
  --ease-mech: cubic-bezier(0.65, 0, 0.35, 1);
  --ease-linear: linear;
}
```

```css
@theme inline {
  --font-display: 'IBM Plex Serif', ui-serif, Georgia, serif;
  --font-sans: 'Inter', ui-sans-serif, system-ui, sans-serif;
  --radius-plate: var(--shape-plate);
  --radius-field: var(--shape-field);
  --radius-pill: var(--shape-pill);
  --color-mint: var(--mint);
  --color-hay: var(--hay);
  --color-sage: var(--sage);
  --color-signal: var(--signal);
  --color-bone: var(--bone);
  --color-oxblood: var(--oxblood);
  --color-blush: var(--blush);
  --color-mulberry: var(--mulberry);
  --color-rose: var(--rose);
  --color-paper: var(--paper);
  --color-terminal: var(--terminal);
  --color-cellar: var(--cellar);
  --color-line: var(--line);
  --color-line-strong: var(--line-strong);
}
```

Font imports, before `index.css`:

```ts
import '@fontsource/ibm-plex-serif/400.css'
import '@fontsource/ibm-plex-serif/500.css'
import '@fontsource/ibm-plex-serif/600.css'
import '@fontsource/ibm-plex-serif/400-italic.css'
import '@fontsource-variable/inter'
```

Notes:

- Remove the `.dark` block. Terminal and Cellar are component classes
  (`.surface-terminal`, `.surface-cellar`), not themes.
- `--primary` is mulberry with white text; links inherit mulberry via `text-primary`.
- `--signal` is separate from `--primary` and never used for actions.
- Focus ring tokens point to ink, not red or green.
- Both families are wired through fontsource in `main.tsx`; nothing is loaded at
  runtime.
- `--accent` is blush so shadcn hover states stay soft; mint and hay remain
  available through `--color-mint` / `--color-hay`.

---

## 16. Do / Don't

| Do | Don't |
|---|---|
| Keep green for data and red for voice | Use red for a live status |
| Use one mulberry CTA per view | Make every button red |
| Set one italic word in mulberry | Colour an entire headline |
| Let the halftone or grid drift slowly | Animate every card and icon |
| Use 1px hairlines with corner ticks | Use blurred shadows and 4px round cards |
| Put long copy on Paper | Put paragraphs on Terminal or Cellar |
| Show a source beside a derived number | Assert numbers without provenance |
| Use duotone archive imagery | Use full-colour stock photography |
| Keep the app light | Add a dark-mode toggle |
| Wire IBM Plex Serif + Inter | Add a mono or condensed third family |

---

## 17. Review checklist

Before shipping a screen:

- [ ] Is green doing data only, red doing voice only?
- [ ] Is there at most one editorial-red gesture beyond the logo/links, and one
      signal-green live system?
- [ ] Are all headings Plex Serif, all UI/body Inter?
- [ ] Do serif sizes stay at 20px or above?
- [ ] Is rose used only at 32px+ on white, or as decoration?
- [ ] Is every functional border `--line-strong` or ink, not sage?
- [ ] Are corner ticks, node ticks, or a rail marking at least one structure?
- [ ] Does the screen have one ambient motion and one entrance reveal, no more?
- [ ] Does it still read with `prefers-reduced-motion: reduce` on?
- [ ] Are the contrast pairs from §4.6 respected (no white on signal, no rose on
      oxblood)?
- [ ] Is the wordmark lowercase with a rose period?
- [ ] Does every derived number carry a source line?
- [ ] Does the copy pass the voice table (no exclamation marks, no emoji)?
- [ ] On dark bands: no long body copy and no white slab?
- [ ] Does the layout use a pattern from `DESIGN.md` rather than a generic card
      grid?
