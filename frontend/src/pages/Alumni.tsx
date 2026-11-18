import { useEffect, useState } from 'react'
import type { CSSProperties } from 'react'
import { fetchAlumni } from '@/api'
import { AlumniList } from '@/AlumniList'
import type { Alumni } from '@/types'

const numberFormat = new Intl.NumberFormat('en-US')

const checkerboard: CSSProperties = {
  backgroundImage:
    'linear-gradient(45deg, var(--line) 25%, transparent 25%, transparent 75%, var(--line) 75%), linear-gradient(45deg, var(--line) 25%, transparent 25%, transparent 75%, var(--line) 75%)',
  backgroundSize: '8px 8px',
  backgroundPosition: '0 0, 4px 4px',
}

const halftoneStyle = {
  '--dot': 'color-mix(in srgb, var(--mulberry) 12%, transparent)',
} as CSSProperties

function message(error: unknown): string {
  return error instanceof Error ? error.message : String(error)
}

export function Alumni() {
  const [alumni, setAlumni] = useState<Alumni[]>([])
  const [error, setError] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)
  const [reloadKey, setReloadKey] = useState(0)

  useEffect(() => {
    let cancelled = false
    setLoading(true)
    setError(null)
    fetchAlumni()
      .then((rows) => {
        if (!cancelled) setAlumni(rows)
      })
      .catch((e: unknown) => {
        if (!cancelled) setError(message(e))
      })
      .finally(() => {
        if (!cancelled) setLoading(false)
      })
    return () => {
      cancelled = true
    }
  }, [reloadKey])

  const years = new Set(alumni.map((a) => a.graduation_year)).size
  const degrees = alumni.filter((a) => Boolean(a.degree?.trim())).length

  const stats = [
    { label: 'Total records', value: alumni.length },
    { label: 'Graduation years', value: years },
    { label: 'Degrees recorded', value: degrees },
  ]

  return (
    <div className="mx-auto w-full max-w-[1280px] px-6 pb-24 md:px-12">
      <header className="relative overflow-hidden pt-16 pb-12 md:pt-20 md:pb-16">
        <div
          aria-hidden="true"
          className="halftone pointer-events-none absolute top-6 right-0 h-24 w-24 md:top-1/2 md:h-60 md:w-60 md:-translate-y-1/2"
          style={halftoneStyle}
        />
        <div className="reveal relative">
          <p className="type-label text-ink-muted">02 / Directory</p>
          <div className="mt-5 h-px w-full bg-line" />
          <h1 className="type-h1 mt-10 max-w-3xl text-ink">
            The <em className="font-normal italic text-mulberry">directory</em>.
          </h1>
          <p className="type-lede mt-6 max-w-2xl text-ink-muted">
            A working register of the Web Development class, Fall 2026–2027.
            Names, degrees and years below are taken from the registrar&apos;s record.
          </p>
        </div>
      </header>

      <section
        aria-label="Directory figures"
        className="border-y border-line py-10 md:py-12"
      >
        <div className="grid grid-cols-1 divide-y divide-line sm:grid-cols-3 sm:divide-x sm:divide-y-0">
          {stats.map((stat) => (
            <div key={stat.label} className="py-5 sm:px-8 sm:py-0 sm:first:pl-0">
              <p
                className={`type-numeral text-[3.5rem] leading-none ${
                  loading ? 'text-ink-muted' : 'text-ink'
                }`}
              >
                {loading ? '—' : numberFormat.format(stat.value)}
              </p>
              <p className="type-label mt-4 text-ink-muted">{stat.label}</p>
            </div>
          ))}
        </div>
        <p className="type-label mt-8 text-ink-muted">
          Source / Registrar · 2026-11-18
        </p>
      </section>

      <section className="pt-12 md:pt-16">
        {error ? (
          <div
            role="alert"
            className="border border-destructive/50 bg-white p-6 md:p-8"
          >
            <p className="type-label text-destructive">Could not read the register</p>
            <p className="mt-4 max-w-prose text-sm text-ink">
              The directory could not be loaded. {error.replace(/[.!?]+$/, '')}. Try
              again once the service is reachable.
            </p>
            <button
              type="button"
              onClick={() => setReloadKey((key) => key + 1)}
              className="type-ui mt-6 rounded-pill border border-line-strong px-5 py-2 text-ink transition-colors hover:bg-blush/60"
            >
              Try again
            </button>
          </div>
        ) : loading ? (
          <div className="border border-line">
            <div className="flex items-center justify-between border-b border-line px-6 py-4">
              <span className="type-label text-ink-muted">03 / Records</span>
              <span className="type-label text-ink-muted">Loading</span>
            </div>
            <div
              role="status"
              aria-label="Loading alumni records"
              className="h-[420px] opacity-40"
              style={checkerboard}
            >
              <span className="sr-only">Reading the register.</span>
            </div>
          </div>
        ) : (
          <div className="corner-ticks relative border border-line bg-white">
            <span className="tick" aria-hidden="true" />
            <span className="tick" aria-hidden="true" />
            <span className="tick" aria-hidden="true" />
            <span className="tick" aria-hidden="true" />
            <div className="flex items-center justify-between border-b border-line px-6 py-4">
              <span className="type-label text-ink-muted">03 / Records</span>
              <span className="type-data text-ink-muted">
                {numberFormat.format(alumni.length)}{' '}
                {alumni.length === 1 ? 'record' : 'records'}
              </span>
            </div>
            <AlumniList alumni={alumni} />
          </div>
        )}
      </section>
    </div>
  )
}
