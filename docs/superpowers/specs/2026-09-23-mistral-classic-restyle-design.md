# Design: "Mistral Classic" frontend restyle

Date: 2026-09-23
Status: approved (user selected direction A from three mockups in
`.superpowers/brainstorm/style-directions.html`)

## Context & goal

The current frontend palette (lime `#d9ed92` canvas, green cards `#b5e48c`,
teal `#34a0a4`, navy `#184e77`) reads muddy and low-contrast. Goal: a full
visual restyle of `frontend/` in the spirit of mistral.ai — warm cream canvas,
near-black ink, one vivid teal — while keeping our green/teal hue family and
making it vibrant. CSS/markup only; no functional, API, or routing changes.

## Decisions (from brainstorming)

- **Palette base:** refined teal/green — cream background, ink text, vivid
  teal hero accent. The original lime survives as a highlight accent.
- **Scope:** full restyle — palette, typography, hero, and every page
  (Home, Alumni, NotFound) plus Layout chrome.
- **Dark mode:** dropped. Light-only. ThemeToggle, the `.dark` CSS block,
  and the `index.html` theme bootstrap script are removed.

## Design tokens

`frontend/src/index.css`, `:root` only (light):

| Token | Value | Role |
|---|---|---|
| `--background` | `#FAF9F4` | warm cream canvas |
| `--foreground` | `#171512` | near-black ink |
| `--card` / `--card-foreground` | `#FFFFFF` / `#171512` | cards float on cream |
| `--popover` / `--popover-foreground` | `#FFFFFF` / `#171512` | |
| `--primary` / `--primary-foreground` | `#0D9AA0` / `#FFFFFF` | vivid teal — CTAs, active nav, links |
| `--secondary` / `--secondary-foreground` | `#E7F4EE` / `#171512` | soft mint pills/tags |
| `--muted` / `--muted-foreground` | `#F0EDE4` / `#6B675F` | subtle fills, secondary text |
| `--accent` / `--accent-foreground` | `#D9ED92` / `#171512` | lime highlight swash, ghost hover |
| `--destructive` | keep current oklch red | error text |
| `--border` / `--input` | `#E9E5DC` | hairline warm borders |
| `--ring` | `#0D9AA0` | focus rings |

`--radius` bumps `0.625rem` → `1rem` (rounder cards). The `.dark` token block
is deleted. The `@custom-variant dark (&:is(.dark *))` declaration is kept:
nothing ever applies the `.dark` class, so `dark:` utilities stay permanently
inert — but removing the declaration would re-enable Tailwind's built-in
media-query `dark:` variant, letting dark-OS users get stray dark styles from
the vendored shadcn components. Inert `dark:` utility classes inside vendored
shadcn `components/ui/*` files are left untouched (harmless; minimal diff).

## Typography

- **Display:** Sora (variable) — headings, logo. Weight 600–800,
  tracking `-0.03em`–`-0.04em`.
- **Body:** Inter (variable) — 400–600.
- Self-hosted via npm: `@fontsource-variable/sora`, `@fontsource-variable/inter`
  (works offline and inside Docker; no runtime Google Fonts request).
- Exposed in Tailwind v4 `@theme` as `--font-display` and `--font-sans`;
  imported in `main.tsx` before `index.css`.

## Page & component changes

- **`Layout.tsx`**: logo `alumni.` (teal dot, Sora 800), hairline header/footer
  borders, nav links (active = ink semibold, idle = muted with ink hover).
  ThemeToggle removed.
- **`Home.tsx`**: centered hero — Sora ~`text-6xl` heading with a lime swash
  (`--accent`) behind one word (absolutely-positioned block behind the text),
  muted subtitle, teal pill CTA "Browse alumni".
- **`Alumni.tsx`**: Sora heading, white card on cream containing the table.
- **`AlumniList.tsx` / table**: uppercase micro-label headers (`text-xs`,
  tracking-wide, muted), semibold name column, warm hairline row dividers,
  hover row tint.
- **`button.tsx`**: base radius `rounded-md` → `rounded-full` (pill). Default
  variant is teal on white; secondary is mint pill; outline keeps warm border.
- **`NotFound.tsx`**: Sora display heading + teal link home.

## Out of scope

- No new dependencies beyond the two fontsource packages; no component API
  changes; no backend, API, or routing changes; no dark mode replacement.

## Docs (project rules)

- `docs/done.md`: log the restyle.
- `docs/trade-offs.md`: light-only (dropped dark mode); self-hosted fonts
  over CDN; inert `dark:` classes left in vendored shadcn files.
- `docs/backlog.md`: no new backlog items (CRUD etc. unaffected).
- No `docs/architecture/*.mmd` changes — no architectural change.

## Verification

1. `npm run build` in `frontend/` — type-check + production build passes.
2. User visual check via docker compose (or `npm run dev`).
3. No frontend test suite exists; nothing functional changes, so no test
   updates required.
