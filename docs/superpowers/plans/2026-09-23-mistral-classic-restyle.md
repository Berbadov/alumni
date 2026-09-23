# Mistral Classic Frontend Restyle — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Restyle `frontend/` to the approved "Mistral Classic" direction — cream canvas, ink text, vivid teal accent, Sora/Inter type, light-only — per spec `docs/superpowers/specs/2026-09-23-mistral-classic-restyle-design.md`.

**Architecture:** Pure CSS/markup change across the React SPA. Design tokens live in `frontend/src/index.css` (shadcn CSS-variable system, Tailwind v4). No API, routing, or component-interface changes. Dark-mode machinery is deleted; the class-based `@custom-variant dark` declaration is kept so leftover `dark:` utilities in vendored shadcn components stay inert.

**Tech Stack:** React 18 + Vite 5 + TypeScript, Tailwind CSS v4 (`@tailwindcss/vite`), shadcn/ui components, `@fontsource-variable/sora` + `@fontsource-variable/inter` (new deps).

**Testing note:** There is no frontend test framework, and this change is visual (CSS + JSX classes). TDD is not applicable; the per-task verification is `npm run build` (`tsc && vite build`) in `frontend/`, which type-checks every edited `.tsx` file and catches broken imports. Final visual check is done by the user via `npm run dev` or docker compose.

All shell commands run from `frontend/` unless noted. On this Windows host use Git Bash paths (`/c/Users/beraat.yildirim/Desktop/alumni/frontend`).

---

### Task 1: Self-hosted fonts (install + wiring)

**Files:**
- Modify: `frontend/package.json` (via npm)
- Modify: `frontend/src/main.tsx`
- Modify: `frontend/src/index.css:49-72` (`@theme inline` block + base layer)

- [ ] **Step 1: Install the two fontsource packages**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni/frontend && npm install @fontsource-variable/sora @fontsource-variable/inter
```

Expected: installs cleanly, `package.json` gains both in `dependencies`.

- [ ] **Step 2: Import the fonts in `main.tsx` (before `index.css`)**

```tsx
import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import { App } from './App'
import '@fontsource-variable/sora'
import '@fontsource-variable/inter'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>,
)
```

- [ ] **Step 3: Add font theme variables and body font in `index.css`**

Inside the existing `@theme inline { ... }` block (after the `--radius-*` lines), add:

```css
  --font-display: 'Sora Variable', 'Inter Variable', ui-sans-serif, system-ui, sans-serif;
  --font-sans: 'Inter Variable', ui-sans-serif, system-ui, sans-serif;
```

In the `@layer base` block, change the `body` rule to apply the sans font explicitly:

```css
  body {
    @apply bg-background font-sans text-foreground;
  }
```

- [ ] **Step 4: Verify build**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni/frontend && npm run build
```

Expected: `tsc` + `vite build` pass, no errors.

- [ ] **Step 5: Commit**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni && git add frontend/package.json frontend/package-lock.json frontend/src/main.tsx frontend/src/index.css && git commit -m "Add self-hosted Sora and Inter variable fonts"
```

(Append the session commit footer per repo convention.)

---

### Task 2: Design tokens — rewrite `:root`, drop `.dark`, bump radius

**Files:**
- Modify: `frontend/src/index.css:6-47`

- [ ] **Step 1: Replace the `:root` block with the new light-only tokens and delete the `.dark` block**

Replace lines 6–47 (`:root { ... }` and `.dark { ... }`) with:

```css
:root {
  --radius: 1rem;
  --background: #faf9f4;
  --foreground: #171512;
  --card: #ffffff;
  --card-foreground: #171512;
  --popover: #ffffff;
  --popover-foreground: #171512;
  --primary: #0d9aa0;
  --primary-foreground: #ffffff;
  --secondary: #e7f4ee;
  --secondary-foreground: #171512;
  --muted: #f0ede4;
  --muted-foreground: #6b675f;
  --accent: #d9ed92;
  --accent-foreground: #171512;
  --destructive: oklch(0.577 0.245 27.325);
  --border: #e9e5dc;
  --input: #e9e5dc;
  --ring: #0d9aa0;
}
```

Do NOT touch the `@custom-variant dark (&:is(.dark *));` line — it stays (see spec: it keeps leftover `dark:` utilities inert).

- [ ] **Step 2: Verify build**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni/frontend && npm run build
```

Expected: passes.

- [ ] **Step 3: Commit**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni && git add frontend/src/index.css && git commit -m "Mistral Classic tokens: cream canvas, ink text, vivid teal; drop dark palette"
```

---

### Task 3: Remove dark-mode machinery + restyle Layout

**Files:**
- Modify: `frontend/index.html:7-15`
- Delete: `frontend/src/components/ThemeToggle.tsx`
- Modify: `frontend/src/components/Layout.tsx`

- [ ] **Step 1: Remove the pre-paint theme script from `index.html`**

Delete lines 7–15 (the `<script>...})();</script>` block reading `localStorage.getItem('alumni-theme')`). Result:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Alumni</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

- [ ] **Step 2: Delete the ThemeToggle component**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni && git rm frontend/src/components/ThemeToggle.tsx
```

- [ ] **Step 3: Restyle `Layout.tsx` (no more ThemeToggle import/usage)**

Replace the whole file with:

```tsx
import { NavLink, Outlet } from 'react-router-dom'

function navLinkClass({ isActive }: { isActive: boolean }) {
  return isActive
    ? 'font-semibold text-foreground'
    : 'text-muted-foreground transition-colors hover:text-foreground'
}

export function Layout() {
  return (
    <div className="flex min-h-screen flex-col">
      <header className="border-b">
        <div className="mx-auto flex w-full max-w-4xl items-center justify-between p-4">
          <nav className="flex items-center gap-6">
            <span className="font-display text-xl font-extrabold tracking-tight">
              alumni<span className="text-primary">.</span>
            </span>
            <NavLink to="/" end className={navLinkClass}>
              Home
            </NavLink>
            <NavLink to="/alumni" className={navLinkClass}>
              Alumni
            </NavLink>
          </nav>
        </div>
      </header>
      <main className="mx-auto w-full max-w-4xl flex-1 p-4">
        <Outlet />
      </main>
      <footer className="border-t">
        <div className="mx-auto w-full max-w-4xl p-4 text-sm text-muted-foreground">
          Alumni Tracking System &mdash; Web Development, Fall 2026&ndash;2027
        </div>
      </footer>
    </div>
  )
}
```

- [ ] **Step 4: Verify build (catches any leftover ThemeToggle import)**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni/frontend && npm run build
```

Expected: passes. If `tsc` reports an unresolved `@/components/ThemeToggle` import anywhere, that call site was missed — remove it the same way.

- [ ] **Step 5: Commit**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni && git add frontend/index.html frontend/src/components/Layout.tsx && git commit -m "Remove dark mode; restyle header and footer chrome"
```

---

### Task 4: Home hero with lime highlight swash

**Files:**
- Modify: `frontend/src/pages/Home.tsx`

- [ ] **Step 1: Replace `Home.tsx` with the hero treatment**

```tsx
import { Link } from 'react-router-dom'
import { Button } from '@/components/ui/button'

export function Home() {
  return (
    <div className="space-y-8 py-24 text-center">
      <h1 className="font-display text-5xl font-extrabold tracking-tight sm:text-6xl">
        Know everyone in your{' '}
        <span className="relative inline-block">
          <span
            aria-hidden
            className="absolute inset-x-[-0.1em] bottom-[0.1em] h-[0.32em] rounded-sm bg-accent"
          />
          <span className="relative">cohort</span>
        </span>
        .
      </h1>
      <p className="mx-auto max-w-xl text-lg text-muted-foreground">
        Browse classmates, their degrees, and graduation years &mdash; Web
        Development, Fall 2026&ndash;2027.
      </p>
      <Button asChild size="lg">
        <Link to="/alumni">Browse alumni</Link>
      </Button>
    </div>
  )
}
```

The swash is an absolutely-positioned lime (`--accent`) bar behind the word "cohort" — `aria-hidden` so screen readers skip it; the visible text stays in normal flow.

- [ ] **Step 2: Verify build**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni/frontend && npm run build
```

Expected: passes.

- [ ] **Step 3: Commit**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni && git add frontend/src/pages/Home.tsx && git commit -m "Home hero: display type with lime highlight swash"
```

---

### Task 5: Alumni page + table restyle

**Files:**
- Modify: `frontend/src/pages/Alumni.tsx:17-19`
- Modify: `frontend/src/AlumniList.tsx`

- [ ] **Step 1: Display heading in `Alumni.tsx`**

Change the `<h1>` to:

```tsx
      <h1 className="font-display text-3xl font-bold tracking-tight">
        Alumni &mdash; Web Development, Fall 2026&ndash;2027
      </h1>
```

- [ ] **Step 2: Restyle the table in `AlumniList.tsx`**

Replace the file with:

```tsx
import type { Alumni } from './types'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

export function AlumniList({ alumni }: { alumni: Alumni[] }) {
  if (alumni.length === 0) {
    return <p className="text-muted-foreground">No alumni yet.</p>
  }
  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead className="h-11 px-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
            Name
          </TableHead>
          <TableHead className="h-11 px-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
            Degree
          </TableHead>
          <TableHead className="h-11 px-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
            Graduation year
          </TableHead>
          <TableHead className="h-11 px-4 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
            Email
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {alumni.map((a) => (
          <TableRow key={a._id}>
            <TableCell className="px-4 font-medium">{a.full_name}</TableCell>
            <TableCell className="px-4">{a.degree}</TableCell>
            <TableCell className="px-4">{a.graduation_year}</TableCell>
            <TableCell className="px-4">{a.email}</TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}
```

Row hover tint and row dividers come from the existing `table.tsx` (`hover:bg-muted/50`, `border-b`) driven by the new tokens — no `table.tsx` edits needed.

- [ ] **Step 3: Verify build**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni/frontend && npm run build
```

Expected: passes.

- [ ] **Step 4: Commit**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni && git add frontend/src/pages/Alumni.tsx frontend/src/AlumniList.tsx && git commit -m "Alumni page: display heading, micro-label table headers"
```

---

### Task 6: NotFound restyle

**Files:**
- Modify: `frontend/src/pages/NotFound.tsx`

- [ ] **Step 1: Replace `NotFound.tsx`**

```tsx
import { Link } from 'react-router-dom'
import { Button } from '@/components/ui/button'

export function NotFound() {
  return (
    <div className="space-y-6 py-24 text-center">
      <h1 className="font-display text-5xl font-extrabold tracking-tight">
        404 &mdash; Page not found
      </h1>
      <p className="text-muted-foreground">
        The page you are looking for does not exist.
      </p>
      <Button asChild>
        <Link to="/">Back to home</Link>
      </Button>
    </div>
  )
}
```

- [ ] **Step 2: Verify build**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni/frontend && npm run build
```

Expected: passes.

- [ ] **Step 3: Commit**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni && git add frontend/src/pages/NotFound.tsx && git commit -m "NotFound: display type treatment"
```

---

### Task 7: Pill buttons

**Files:**
- Modify: `frontend/src/components/ui/button.tsx:7` (base) and size variants `xs`, `sm`, `lg`

- [ ] **Step 1: Change every `rounded-md` in the cva to `rounded-full`**

In `buttonVariants`:
- base string: `rounded-md` → `rounded-full`
- size `xs`: `rounded-md` → `rounded-full`
- size `sm`: `rounded-md` → `rounded-full`
- size `lg`: `rounded-md` → `rounded-full`

(Size variants would otherwise override the base radius, so all four must change.)

- [ ] **Step 2: Verify build**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni/frontend && npm run build
```

Expected: passes.

- [ ] **Step 3: Commit**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni && git add frontend/src/components/ui/button.tsx && git commit -m "Pill-shaped buttons"
```

---

### Task 8: Docs (project rule: docs are first-class)

**Files:**
- Modify: `docs/done.md` (insert entry after the intro lines, newest-first)
- Modify: `docs/trade-offs.md` (insert entries after the intro lines)

- [ ] **Step 1: Add the done entry at the top of `docs/done.md`**

Insert after the `Completed work log, newest first.` line:

```markdown
## 2026-09-23 — "Mistral Classic" frontend restyle
- Full visual restyle of `frontend/` in the spirit of mistral.ai, on our green/teal family:
  warm cream canvas `#faf9f4`, near-black ink `#171512`, vivid teal primary `#0d9aa0`,
  original lime `#d9ed92` kept as the hero highlight accent.
- Typography: Sora (display, `--font-display`) + Inter (body, `--font-sans`), self-hosted via
  `@fontsource-variable/*` so fonts work offline and inside Docker.
- Components: pill buttons (`rounded-full`), `--radius` 1rem, white cards on cream; Home hero
  (`text-5xl/6xl` Sora, lime swash behind "cohort", teal CTA); Alumni table with uppercase
  micro-label headers, semibold names, hover tint; NotFound restyled.
- Dark mode removed: `ThemeToggle.tsx` deleted, `.dark` token block dropped, pre-paint theme
  script removed from `index.html`; class-based `@custom-variant dark` kept so leftover `dark:`
  utilities in vendored shadcn files stay inert (see `trade-offs.md`).
- Verified: `npm run build` passes.
```

- [ ] **Step 2: Add two trade-off entries at the top of `docs/trade-offs.md`**

Insert after the `Decisions and compromises, with rationale.` line:

```markdown
## Frontend restyle: light-only vs dark mode
Dropped dark mode for the "Mistral Classic" restyle (spec:
`docs/superpowers/specs/2026-09-23-mistral-classic-restyle-design.md`). A second palette
doubles token maintenance, and the dark variant was part of the muddy look. The class-based
`@custom-variant dark` declaration stays in `index.css` even though `.dark` is never applied:
deleting it would re-enable Tailwind v4's built-in media-query `dark:` variant, giving
dark-OS users stray dark styles from `dark:` utilities left in the vendored shadcn components;
stripping those classes from every vendored file would be a larger, noisier diff.

## Fonts: self-hosted fontsource vs Google Fonts CDN
Self-hosted via `@fontsource-variable/sora` + `@fontsource-variable/inter`. The app must run
inside Docker and look identical offline; a CDN link adds a runtime network dependency and a
font-flicker risk. Cost: two variable-font woff2 files in the bundle — acceptable for a
class app.
```

- [ ] **Step 3: Commit**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni && git add docs/done.md docs/trade-offs.md && git commit -m "Docs: Mistral Classic restyle log and trade-offs"
```

---

### Task 9: Final verification

- [ ] **Step 1: Clean full build**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni/frontend && npm run build
```

Expected: `tsc` reports no errors; `vite build` completes with the `dist/` bundle listing.

- [ ] **Step 2: Check working tree is clean**

```bash
cd /c/Users/beraat.yildirim/Desktop/alumni && git status --short
```

Expected: empty (everything committed across Tasks 1–8).

- [ ] **Step 3: Hand off to the user for visual check**

Tell the user: run `docker compose up --build` (or `npm run dev` in `frontend/`) and compare against `.superpowers/brainstorm/style-directions.html` direction A. Backend/API behavior is untouched.
