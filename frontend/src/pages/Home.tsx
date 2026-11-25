import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import {
  ArrowUpRight,
  Briefcase,
  GraduationCap,
  Info,
  LogIn,
  Megaphone,
  ShieldCheck,
  UserCheck,
  UserPlus,
  Users,
  type LucideIcon,
} from 'lucide-react'
import { fetchAlumni, fetchJobPostings } from '@/api'
import { useAuth } from '@/auth'
import { Button } from '@/components/ui/button'
import {
  hasAnyRole,
  hasRole,
  isAdmin,
  isVerifiedMember,
  primaryWorkspaceFor,
} from '@/navigation'
import type { Alumni, JobPosting, User } from '@/types'

interface HomePlate {
  title: string
  href: string
  icon: LucideIcon
  proof: string
  cta: string
}

function getPlates(user: User | null): HomePlate[] {
  if (isAdmin(user)) {
    return [
      {
        title: 'Admin',
        href: '/admin',
        icon: ShieldCheck,
        proof: 'Verify government papers, assign roles, and revoke sessions.',
        cta: 'Open control ledger',
      },
      {
        title: 'Faculty',
        href: '/faculty',
        icon: Megaphone,
        proof: 'Publish and manage class announcements.',
        cta: 'Manage announcements',
      },
      {
        title: 'Alumni',
        href: '/alumni',
        icon: Users,
        proof: 'Names, degrees, and the years they left.',
        cta: 'View directory',
      },
      {
        title: 'Jobs',
        href: '/jobs',
        icon: Briefcase,
        proof: 'Roles posted by people you studied with.',
        cta: 'See open roles',
      },
    ]
  }

  if (isVerifiedMember(user)) {
    const workspacePlate: HomePlate = hasAnyRole(user, [
      'academic',
      'professor',
    ])
      ? {
          title: 'Faculty',
          href: '/faculty',
          icon: Megaphone,
          proof: 'Publish and edit notices for students and alumni.',
          cta: 'Open announcements',
        }
      : hasRole(user, 'alumni')
        ? {
            title: 'Graduate',
            href: '/graduate',
            icon: GraduationCap,
            proof: 'Manage job postings you published and read class news.',
            cta: 'Open graduate desk',
          }
        : {
            title: 'Student',
            href: '/student',
            icon: UserCheck,
            proof: 'Read announcements and active roles for your track.',
            cta: 'Open student space',
          }

    return [
      workspacePlate,
      {
        title: 'Alumni',
        href: '/alumni',
        icon: Users,
        proof: 'Names, degrees, and the years they left.',
        cta: 'View directory',
      },
      {
        title: 'Jobs',
        href: '/jobs',
        icon: Briefcase,
        proof: 'Roles posted by people you studied with.',
        cta: 'See open roles',
      },
    ]
  }

  if (user) {
    return [
      {
        title: 'Verification',
        href: '/verification',
        icon: ShieldCheck,
        proof: 'Track government-paper verification for your requested role.',
        cta: 'View status',
      },
      {
        title: 'Account',
        href: '/account',
        icon: UserCheck,
        proof: 'Update your personal profile details and password.',
        cta: 'Open account',
      },
      {
        title: 'About',
        href: '/about',
        icon: Info,
        proof: 'Why this record exists, and how it is kept.',
        cta: 'Read the archive',
      },
    ]
  }

  return [
    {
      title: 'Register',
      href: '/register',
      icon: UserPlus,
      proof: 'Request access as an undergrad, graduate, alumnus, or faculty.',
      cta: 'Create an account',
    },
    {
      title: 'Sign in',
      href: '/login',
      icon: LogIn,
      proof: 'Open your directory and job-board session.',
      cta: 'Sign in now',
    },
    {
      title: 'About',
      href: '/about',
      icon: Info,
      proof: 'Why this record exists, and how it is kept.',
      cta: 'Read the archive',
    },
  ]
}

function Node({ label, live = false }: { label: string; live?: boolean }) {
  return (
    <span className="inline-flex min-h-11 items-center gap-2 border border-line-strong px-3 py-4 type-ui text-bone">
      {live && <span className="live-dot" aria-hidden="true" />}
      {label}
      {live && <span className="type-label text-bone/70">live</span>}
    </span>
  )
}

function Wire() {
  return (
    <span className="relative h-px flex-1 bg-line-strong" aria-hidden="true">
      <span className="absolute -right-px -top-[1px] size-[3px] bg-bone" />
    </span>
  )
}

export function Home() {
  const { user, loading: authLoading } = useAuth()
  const [alumniCount, setAlumniCount] = useState('—')
  const [yearCount, setYearCount] = useState('—')
  const [jobCount, setJobCount] = useState('—')
  const [recordsUnavailable, setRecordsUnavailable] = useState(false)

  const verifiedMember = isVerifiedMember(user)
  const plates = getPlates(user)
  const workspace = primaryWorkspaceFor(user)

  useEffect(() => {
    if (!verifiedMember) {
      setAlumniCount('—')
      setYearCount('—')
      setJobCount('—')
      setRecordsUnavailable(false)
      return
    }

    let cancelled = false
    const format = new Intl.NumberFormat('en-US')

    Promise.all([fetchAlumni(), fetchJobPostings()])
      .then(([alumni, jobs]: [Alumni[], JobPosting[]]) => {
        if (cancelled) return
        const years = new Set(alumni.map((person) => person.graduation_year))
        setAlumniCount(format.format(alumni.length))
        setYearCount(format.format(years.size))
        setJobCount(format.format(jobs.filter((job) => job.is_active).length))
        setRecordsUnavailable(false)
      })
      .catch(() => {
        if (!cancelled) setRecordsUnavailable(true)
      })

    return () => {
      cancelled = true
    }
  }, [verifiedMember])

  return (
    <div>
      {/* 01 — Paper hero. The halftone is the page's single ambient loop. */}
      <section
        aria-labelledby="home-heading"
        className="relative overflow-hidden bg-white"
      >
        <div className="halftone pointer-events-none absolute -right-8 top-12 hidden size-60 opacity-30 md:block" aria-hidden="true" />

        <div className="relative mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <div className="flex flex-wrap items-center justify-between gap-4 border-b border-line py-6 type-label text-ink-muted">
            <span>01 / DIRECTORY</span>
            <span>WEB DEV · 2026–27</span>
          </div>

          <div className="grid grid-cols-1 gap-12 py-20 md:grid-cols-12 md:py-28">
            <div className="reveal-stagger md:col-span-7">
              <h1 id="home-heading" className="type-display text-ink">
                The class of 2026,{' '}
                <em className="font-display italic text-mulberry">
                  still in motion.
                </em>
              </h1>
              <p className="type-lede mt-8 max-w-[55ch] text-muted-foreground">
                A living record of the people who studied here — where they
                went, what they built, and who is hiring now.
              </p>
              <div className="mt-10 flex flex-wrap items-center gap-8">
                {verifiedMember ? (
                  <>
                    <Button asChild size="lg">
                      <Link to={workspace.to}>{workspace.label}</Link>
                    </Button>
                    <Link
                      to="/alumni"
                      className="arrow-link type-ui text-ink hover:text-mulberry"
                    >
                      <span>Browse alumni</span>
                      <span className="arrow-ring">
                        <ArrowUpRight size={20} strokeWidth={1.5} aria-hidden="true" />
                      </span>
                    </Link>
                  </>
                ) : user ? (
                  <>
                    <Button asChild size="lg">
                      <Link to="/verification">Verification status</Link>
                    </Button>
                    <Link
                      to="/account"
                      className="arrow-link type-ui text-ink hover:text-mulberry"
                    >
                      <span>Account settings</span>
                      <span className="arrow-ring">
                        <ArrowUpRight size={20} strokeWidth={1.5} aria-hidden="true" />
                      </span>
                    </Link>
                  </>
                ) : (
                  <>
                    <Button asChild size="lg">
                      <Link to="/register">Register</Link>
                    </Button>
                    <Link
                      to="/login"
                      className="arrow-link type-ui text-ink hover:text-mulberry"
                    >
                      <span>Sign in</span>
                      <span className="arrow-ring">
                        <ArrowUpRight size={20} strokeWidth={1.5} aria-hidden="true" />
                      </span>
                    </Link>
                  </>
                )}
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Instrument strip — counts carry a source line, never colour alone. */}
      <section aria-labelledby="records-heading" className="bg-white">
        <div className="mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <h2 id="records-heading" className="sr-only">
            The directory in numbers
          </h2>
          <dl className="grid grid-cols-1 divide-y divide-line sm:grid-cols-3 sm:divide-x sm:divide-y-0">
            <div className="py-12 sm:pr-8">
              <dt className="type-label text-ink-muted">Alumni on record</dt>
              <dd className="type-numeral mt-4 text-[3.5rem] leading-none tracking-[-0.04em] text-ink">
                {alumniCount}
              </dd>
            </div>
            <div className="py-12 sm:px-8">
              <dt className="type-label text-ink-muted">Graduation years</dt>
              <dd className="type-numeral mt-4 text-[3.5rem] leading-none tracking-[-0.04em] text-ink">
                {yearCount}
              </dd>
            </div>
            <div className="py-12 sm:pl-8">
              <dt className="type-label text-ink-muted">Active jobs</dt>
              <dd className="type-numeral mt-4 text-[3.5rem] leading-none tracking-[-0.04em] text-ink">
                {jobCount}
              </dd>
            </div>
          </dl>
          <div className="flex flex-wrap items-center justify-between gap-3 border-t border-line py-5">
            <p className="type-label text-ink-muted">
              SOURCE / REGISTRAR · 2026-11-18
            </p>
            {recordsUnavailable ? (
              <p className="type-ui text-ink-muted">Records unavailable.</p>
            ) : !authLoading && !verifiedMember ? (
              <p className="type-ui text-ink-muted">
                Verified member sign-in required for live counts.
              </p>
            ) : null}
          </div>
        </div>
      </section>

      {/* Product garden — plates only, no cards. */}
      <section aria-labelledby="explore-heading" className="bg-white pb-24">
        <div className="mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <div className="mb-10 flex flex-wrap items-end justify-between gap-4 border-t border-line pt-10">
            <h2 id="explore-heading" className="type-h2 text-ink">
              {plates.length === 4
                ? 'Four ways in.'
                : plates.length === 2
                  ? 'Where to next.'
                  : 'Three ways in.'}
            </h2>
            <p className="type-label text-ink-muted">
              INDEX / 01—0{plates.length}
            </p>
          </div>
          <div
            className={`grid grid-cols-1 gap-8 ${
              plates.length === 4
                ? 'md:grid-cols-2 lg:grid-cols-4'
                : plates.length === 2
                  ? 'md:grid-cols-2'
                  : 'md:grid-cols-3'
            }`}
          >
            {plates.map((plate) => (
              <Link
                key={plate.title}
                to={plate.href}
                className="corner-ticks group relative flex min-h-72 flex-col rounded-plate border border-line bg-white p-6 transition-[transform,border-color] duration-[160ms] ease-out hover:-translate-y-0.5 hover:border-ink"
              >
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />

                <plate.icon
                  size={24}
                  strokeWidth={1.5}
                  className="text-mulberry"
                  aria-hidden="true"
                />
                <h3 className="type-h3 mt-10 text-ink">{plate.title}</h3>
                <p className="type-ui mt-3 text-muted-foreground">{plate.proof}</p>

                <span className="arrow-link mt-auto pt-10 text-ink">
                  <span className="type-ui">{plate.cta}</span>
                  <span className="arrow-ring">
                    <ArrowUpRight size={20} strokeWidth={1.5} aria-hidden="true" />
                  </span>
                </span>
              </Link>
            ))}
          </div>
        </div>
      </section>

      {/* 02 — Terminal chapter band. Short copy, one live node. */}
      <section aria-labelledby="method-heading" className="surface-terminal">
        <div className="mx-auto w-full max-w-[1280px] px-6 py-24 md:px-12">
          <div className="flex flex-wrap items-center justify-between gap-4 border-b border-line pb-6 type-label">
            <span>02 / METHOD</span>
            <span>THE RECORD, TRACED</span>
          </div>

          <h2
            id="method-heading"
            className="type-display-2 mt-16 max-w-[16ch] text-bone"
          >
            A record that stays{' '}
            <em className="font-display italic text-bone">current</em>.
          </h2>

          <figure className="mt-16 max-w-3xl">
            <figcaption className="sr-only">
              Records move from the registrar to alumni records, into the
              directory, then to jobs — the jobs feed is live.
            </figcaption>
            <div className="flex items-center">
              <Node label="Registrar" />
              <Wire />
              <Node label="Alumni records" />
            </div>
            <div className="ml-8 h-10 w-px bg-line-strong" aria-hidden="true" />
            <div className="flex items-center">
              <Node label="Directory" />
              <Wire />
              <Node label="Jobs" live />
            </div>
          </figure>

          <div className="mt-20 flex flex-wrap items-center justify-between gap-4 border-t border-line pt-6 type-label">
            <span>NEXT / 03 ABOUT</span>
            <span>THE LIVING LEDGER</span>
          </div>
        </div>
      </section>

      {/* 03 — Closing strip on Paper. */}
      <section className="bg-white">
        <div className="mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <div className="flex flex-col gap-6 py-12 sm:flex-row sm:items-end sm:justify-between">
            <div>
              <p className="type-label text-ink-muted">03 / ABOUT</p>
              <p className="type-h3 mt-3 max-w-[24ch] text-ink">
                A record is only useful if you can trace it.
              </p>
            </div>
            <Link
              to="/about"
              className="arrow-link type-ui self-start text-mulberry sm:self-end"
            >
              <span>Read the archive</span>
              <span className="arrow-ring border-mulberry">
                <ArrowUpRight size={20} strokeWidth={1.5} aria-hidden="true" />
              </span>
            </Link>
          </div>
        </div>
      </section>
    </div>
  )
}
