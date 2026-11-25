import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import { ArrowUpRight } from 'lucide-react'
import { fetchAlumni, fetchAnnouncements, fetchJobPostings } from '@/api'
import { useAuth } from '@/auth'
import { Button } from '@/components/ui/button'
import type { Alumni, Announcement, JobPosting } from '@/types'

function message(error: unknown): string {
  return error instanceof Error ? error.message : String(error)
}

export function StudentPortal() {
  const { user } = useAuth()
  const [alumniCount, setAlumniCount] = useState<number | null>(null)
  const [announcements, setAnnouncements] = useState<Announcement[]>([])
  const [jobs, setJobs] = useState<JobPosting[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    let cancelled = false
    setLoading(true)
    Promise.all([fetchAlumni(), fetchAnnouncements(), fetchJobPostings(true)])
      .then(
        ([alumniRows, announcementRows, jobRows]: [
          Alumni[],
          Announcement[],
          JobPosting[],
        ]) => {
          if (cancelled) return
          setAlumniCount(alumniRows.length)
          setAnnouncements(announcementRows)
          setJobs(jobRows)
          setError(null)
        },
      )
      .catch((e: unknown) => {
        if (!cancelled) setError(message(e))
      })
      .finally(() => {
        if (!cancelled) setLoading(false)
      })
    return () => {
      cancelled = true
    }
  }, [])

  if (!user) return null
  const track =
    user.roles.find((r) => r === 'undergrad' || r === 'grad' || r === 'phd') ??
    user.roles[0]

  return (
    <div>
      <section
        aria-labelledby="student-heading"
        className="relative overflow-hidden bg-white"
      >
        <div
          aria-hidden="true"
          className="grid-live pointer-events-none absolute inset-0 opacity-[0.06]"
        />
        <div className="relative mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <p className="type-label border-b border-line py-6 text-ink-muted">
            08 / STUDENT WORKSPACE · {track?.toUpperCase()}
          </p>

          <div className="grid gap-10 py-12 md:grid-cols-12 md:py-16">
            <div className="reveal md:col-span-7">
              <h1 id="student-heading" className="type-h1 text-ink">
                Welcome back,{' '}
                <em className="font-normal italic text-mulberry">
                  {user.first_name}
                </em>
                .
              </h1>
              <p className="type-lede mt-6 max-w-[54ch] text-ink-muted">
                Read faculty announcements, inspect active alumni job offers,
                and explore graduates in the directory.
              </p>
            </div>
            <div className="flex flex-col items-start justify-end gap-3 md:col-span-5 md:items-end">
              <div className="flex flex-wrap gap-3">
                <Button asChild>
                  <Link to="/alumni">Browse directory</Link>
                </Button>
                <Button asChild variant="outline">
                  <Link to="/jobs">See all jobs</Link>
                </Button>
              </div>
            </div>
          </div>
        </div>
      </section>

      <div className="mx-auto w-full max-w-[1280px] space-y-10 px-6 py-10 md:px-12 md:py-14">
        <section className="border-y border-line py-8">
          <dl className="grid grid-cols-1 divide-y divide-line sm:grid-cols-3 sm:divide-x sm:divide-y-0">
            <div className="py-4 sm:pr-8 sm:py-0">
              <dt className="type-label text-ink-muted">Alumni on record</dt>
              <dd className="type-numeral mt-3 text-[3rem] leading-none text-ink">
                {loading || alumniCount === null ? '—' : alumniCount}
              </dd>
            </div>
            <div className="py-4 sm:px-8 sm:py-0">
              <dt className="type-label text-ink-muted">Active open roles</dt>
              <dd className="type-numeral mt-3 text-[3rem] leading-none text-ink">
                {loading ? '—' : jobs.length}
              </dd>
            </div>
            <div className="py-4 sm:pl-8 sm:py-0">
              <dt className="type-label text-ink-muted">Announcements</dt>
              <dd className="type-numeral mt-3 text-[3rem] leading-none text-ink">
                {loading ? '—' : announcements.length}
              </dd>
            </div>
          </dl>
        </section>

        {error ? (
          <div role="alert" className="rounded-plate border border-destructive/40 bg-white p-6">
            <p className="type-label text-destructive">Error</p>
            <p className="type-ui mt-2 text-destructive">{error}</p>
          </div>
        ) : null}

        <div className="grid gap-10 lg:grid-cols-12">
          <section className="rounded-plate border border-line bg-white p-6 lg:col-span-6">
            <div className="flex items-baseline justify-between gap-3 border-b border-line pb-4">
              <h2 className="type-h3 text-ink">Faculty announcements</h2>
              <span className="type-label text-ink-muted">LATEST</span>
            </div>
            {loading ? (
              <p className="type-ui mt-6 text-ink-muted">Loading…</p>
            ) : announcements.length === 0 ? (
              <p className="type-ui mt-6 text-ink-muted">
                No announcements published yet.
              </p>
            ) : (
              <ul className="mt-4 divide-y divide-line">
                {announcements.slice(0, 6).map((item) => (
                  <li key={item._id} className="py-4">
                    <p className="type-ui font-medium text-ink">{item.title}</p>
                    <p className="mt-1 text-sm text-ink-muted">{item.body}</p>
                    <p className="type-label mt-2 text-ink-muted">
                      Author #{item.author_id}
                    </p>
                  </li>
                ))}
              </ul>
            )}
          </section>

          <section className="rounded-plate border border-line bg-white p-6 lg:col-span-6">
            <div className="flex items-baseline justify-between gap-3 border-b border-line pb-4">
              <h2 className="type-h3 text-ink">Open roles</h2>
              <Link to="/jobs" className="type-label text-mulberry hover:underline">
                View all
              </Link>
            </div>
            {loading ? (
              <p className="type-ui mt-6 text-ink-muted">Loading…</p>
            ) : jobs.length === 0 ? (
              <p className="type-ui mt-6 text-ink-muted">
                No active postings right now.
              </p>
            ) : (
              <ul className="mt-4 divide-y divide-line">
                {jobs.slice(0, 6).map((job) => (
                  <li
                    key={job._id}
                    className="flex flex-wrap items-center justify-between gap-4 py-4"
                  >
                    <div>
                      <p className="type-ui font-medium text-ink">{job.title}</p>
                      <p className="text-sm text-ink-muted">
                        {job.company}
                        {job.location ? ` · ${job.location}` : ''}
                      </p>
                    </div>
                    <a
                      href={job.apply_url}
                      target="_blank"
                      rel="noreferrer"
                      className="arrow-link type-ui text-mulberry"
                    >
                      <span>Apply</span>
                      <span className="arrow-ring border-mulberry">
                        <ArrowUpRight size={16} strokeWidth={1.5} aria-hidden="true" />
                      </span>
                    </a>
                  </li>
                ))}
              </ul>
            )}
          </section>
        </div>
      </div>
    </div>
  )
}
