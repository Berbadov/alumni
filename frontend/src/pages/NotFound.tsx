import { Link } from 'react-router-dom'
import { ArrowUpRight } from 'lucide-react'
import { Button } from '@/components/ui/button'

type IndexEntry = {
  to: string
  number: string
  label: string
  missing?: boolean
}

const INDEX: IndexEntry[] = [
  { to: '/', number: '01', label: 'Home' },
  { to: '/alumni', number: '02', label: 'Alumni' },
  { to: '/jobs', number: '03', label: 'Jobs' },
]

export function NotFound() {
  return (
    <section
      aria-labelledby="not-found-title"
      className="flex h-full min-h-[600px] flex-col bg-white text-ink"
    >
      <div className="mx-auto flex w-full max-w-[1280px] flex-1 flex-col px-6 md:px-12">
        <div className="grid flex-1 items-center gap-16 py-16 lg:grid-cols-[5fr_7fr] lg:gap-24">
          <div className="reveal-stagger">
            <div>
              <p className="type-label text-ink-muted">ERROR / 404</p>
              <div aria-hidden="true" className="mt-4 h-px w-full bg-line" />
            </div>

            <h1 id="not-found-title" className="type-display mt-8 text-ink">
              40<span className="italic text-rose">4</span>
            </h1>

            <div className="mt-8">
              <h2 className="type-h2 text-ink">This page is not in the ledger.</h2>
              <p className="type-lede mt-4 max-w-[52ch] text-ink-muted">
                The address does not match a record. Return to the directory or
                browse the alumni.
              </p>
            </div>

            <div className="mt-10 flex flex-wrap items-center gap-x-8 gap-y-5">
              <Button asChild>
                <Link to="/">Back to home</Link>
              </Button>
              <Link
                to="/alumni"
                className="arrow-link type-ui text-ink transition-colors duration-[120ms] hover:text-mulberry focus-visible:text-mulberry"
              >
                Browse alumni
                <span className="arrow-ring">
                  <ArrowUpRight
                    aria-hidden="true"
                    className="size-5"
                    strokeWidth={1.5}
                  />
                </span>
              </Link>
            </div>
          </div>

          <nav
            aria-label="Directory index"
            className="ledger-frame relative flex min-h-[320px] flex-col overflow-hidden rounded-plate border border-line bg-white p-6 pb-48 md:p-8 md:pb-52"
          >
            <div
              aria-hidden="true"
              className="halftone pointer-events-none absolute right-0 bottom-0 size-[180px] max-w-[240px] opacity-70"
            />

            <p className="type-label text-ink-muted">INDEX / DIRECTORY</p>

            <ul className="relative mt-8 border-t border-line">
              {INDEX.map((entry) => (
                <li key={entry.number}>
                  <Link
                    to={entry.to}
                    className="group flex min-h-14 items-center justify-between gap-4 border-b border-line px-1 transition-colors duration-[120ms] hover:border-ink hover:bg-bone focus-visible:bg-bone"
                  >
                    <span className="flex items-baseline gap-3">
                      <span className="type-numeral text-base text-ink-muted">
                        {entry.number}
                      </span>
                      <span className="type-label text-ink-muted transition-colors duration-[120ms] group-hover:text-ink">
                        / {entry.label}
                      </span>
                    </span>
                    {entry.missing ? (
                      <span className="type-label flex items-center gap-2 text-ink-muted">
                        <span className="status-dot" aria-hidden="true" />
                        missing
                      </span>
                    ) : null}
                  </Link>
                </li>
              ))}
              <li className="flex min-h-14 items-center justify-between gap-4 border-b border-line px-1">
                <span className="flex items-baseline gap-3">
                  <span className="type-numeral text-base text-ink-muted">04</span>
                  <span className="type-label text-ink-muted">/ THIS PAGE</span>
                </span>
                <span className="type-label flex items-center gap-2 text-ink-muted">
                  <span className="status-dot" aria-hidden="true" />
                  missing
                </span>
              </li>
            </ul>
          </nav>
        </div>

        <div className="flex flex-wrap items-center justify-between gap-4 border-t border-line py-6">
          <p className="type-label text-ink-muted">SOURCE / ROUTER · 2026-11-18</p>
          <p className="type-label text-ink-muted">NO RECORD</p>
        </div>
      </div>
    </section>
  )
}
