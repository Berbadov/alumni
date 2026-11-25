import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import {
  deleteJobPosting,
  fetchAlumni,
  fetchAnnouncements,
  fetchJobPostings,
  setJobPostingActive,
} from '@/api'
import { useAuth } from '@/auth'
import { JobPostingForm } from '@/components/JobPostingForm'
import { Button } from '@/components/ui/button'
import type { Alumni, Announcement, JobPosting } from '@/types'

function message(error: unknown): string {
  return error instanceof Error ? error.message : String(error)
}

export function GraduateWorkspace() {
  const { user } = useAuth()
  const [alumni, setAlumni] = useState<Alumni[]>([])
  const [myPostings, setMyPostings] = useState<JobPosting[]>([])
  const [announcements, setAnnouncements] = useState<Announcement[]>([])
  const [alumniLoading, setAlumniLoading] = useState(true)
  const [loading, setLoading] = useState(true)
  const [formOpen, setFormOpen] = useState(false)
  const [editing, setEditing] = useState<JobPosting | null>(null)
  const [busyId, setBusyId] = useState<number | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [notice, setNotice] = useState<string | null>(null)
  const [reloadKey, setReloadKey] = useState(0)

  useEffect(() => {
    if (!user) return
    let cancelled = false
    setLoading(true)
    setAlumniLoading(true)
    Promise.all([fetchAlumni(), fetchJobPostings(), fetchAnnouncements()])
      .then(
        ([alumniRows, jobRows, announcementRows]: [
          Alumni[],
          JobPosting[],
          Announcement[],
        ]) => {
          if (cancelled) return
          setAlumni(alumniRows)
          setMyPostings(jobRows.filter((item) => item.author_id === user._id))
          setAnnouncements(announcementRows)
          setError(null)
        },
      )
      .catch((e: unknown) => {
        if (!cancelled) setError(message(e))
      })
      .finally(() => {
        if (!cancelled) {
          setLoading(false)
          setAlumniLoading(false)
        }
      })
    return () => {
      cancelled = true
    }
  }, [user, reloadKey])

  if (!user) return null

  const linkedAlumni = alumni.find(
    (row) => row.email.toLowerCase() === user.email.toLowerCase(),
  )

  function handleSaved(saved: JobPosting, created: boolean) {
    setFormOpen(false)
    setEditing(null)
    setNotice(`${created ? 'Published' : 'Saved'} “${saved.title}”.`)
    setReloadKey((k) => k + 1)
  }

  async function handleToggle(posting: JobPosting) {
    setBusyId(posting._id)
    setError(null)
    try {
      const saved = await setJobPostingActive(posting._id, !posting.is_active)
      setNotice(
        saved.is_active
          ? `Activated “${saved.title}”.`
          : `Deactivated “${saved.title}”.`,
      )
      setReloadKey((k) => k + 1)
    } catch (e: unknown) {
      setError(message(e))
    } finally {
      setBusyId(null)
    }
  }

  async function handleDelete(posting: JobPosting) {
    if (!window.confirm(`Delete “${posting.title}”?`)) return
    setBusyId(posting._id)
    setError(null)
    try {
      await deleteJobPosting(posting._id)
      if (editing?._id === posting._id) {
        setEditing(null)
        setFormOpen(false)
      }
      setNotice(`Deleted “${posting.title}”.`)
      setReloadKey((k) => k + 1)
    } catch (e: unknown) {
      setError(message(e))
    } finally {
      setBusyId(null)
    }
  }

  return (
    <div>
      <section
        aria-labelledby="graduate-heading"
        className="relative overflow-hidden bg-white"
      >
        <div
          aria-hidden="true"
          className="grid-live pointer-events-none absolute inset-0 opacity-[0.06]"
        />
        <div className="relative mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <p className="type-label border-b border-line py-6 text-ink-muted">
            08 / GRADUATE WORKSPACE
          </p>

          <div className="grid gap-10 py-12 md:grid-cols-12 md:py-16">
            <div className="reveal md:col-span-7">
              <h1 id="graduate-heading" className="type-h1 text-ink">
                Alumni{' '}
                <em className="font-normal italic text-mulberry">desk</em>.
              </h1>
              <p className="type-lede mt-6 max-w-[54ch] text-ink-muted">
                Publish roles from your team, manage your active job offers, and
                stay in sync with class updates.
              </p>
            </div>
            <div className="flex flex-col items-start justify-end gap-3 md:col-span-5 md:items-end">
              <Button
                onClick={() => {
                  setEditing(null)
                  setFormOpen(true)
                  setNotice(null)
                }}
              >
                New job posting
              </Button>
              <p className="type-label text-ink-muted">
                {user.graduation_date
                  ? `CLASS / ${user.graduation_date}`
                  : 'VERIFIED ALUMNI'}
              </p>
            </div>
          </div>
        </div>
      </section>

      <div className="mx-auto w-full max-w-[1280px] space-y-8 px-6 py-10 md:px-12 md:py-14">
        {error ? (
          <div role="alert" className="rounded-plate border border-destructive/40 bg-white p-5">
            <p className="type-ui text-destructive">{error}</p>
          </div>
        ) : null}
        {notice ? (
          <div
            role="status"
            className="flex items-center gap-3 rounded-plate border border-line-strong bg-white px-4 py-3"
          >
            <span className="live-dot" aria-hidden="true" />
            <p className="type-ui text-ink">{notice}</p>
          </div>
        ) : null}

        {formOpen ? (
          <JobPostingForm
            key={editing?._id ?? 'new'}
            posting={editing}
            alumni={alumni}
            alumniLoading={alumniLoading}
            onCancel={() => {
              setFormOpen(false)
              setEditing(null)
            }}
            onSaved={handleSaved}
          />
        ) : null}

        <div className="grid gap-8 lg:grid-cols-12">
          <section className="rounded-plate border border-line bg-white p-6 lg:col-span-8">
            <div className="flex flex-wrap items-baseline justify-between gap-3 border-b border-line pb-4">
              <h2 className="type-h3 text-ink">My published job postings</h2>
              <span className="type-data text-ink-muted">
                {myPostings.length} total
              </span>
            </div>

            {loading ? (
              <p className="type-ui mt-6 text-ink-muted">Loading postings…</p>
            ) : myPostings.length === 0 ? (
              <p className="type-ui mt-6 text-ink-muted">
                You have not published any job postings under account #{user._id}{' '}
                yet.
              </p>
            ) : (
              <ul className="mt-4 divide-y divide-line">
                {myPostings.map((posting) => (
                  <li
                    key={posting._id}
                    className="flex flex-wrap items-center justify-between gap-4 py-4"
                  >
                    <div>
                      <div className="flex items-center gap-2">
                        {posting.is_active ? (
                          <span className="live-dot" aria-hidden="true" />
                        ) : (
                          <span
                            className="status-dot text-ink-muted"
                            aria-hidden="true"
                          />
                        )}
                        <p className="type-ui font-medium text-ink">
                          {posting.title}
                        </p>
                      </div>
                      <p className="mt-1 text-sm text-ink-muted">
                        {posting.company} &middot; {posting.employment_type}
                      </p>
                    </div>
                    <div className="flex flex-wrap gap-2">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => {
                          setEditing(posting)
                          setFormOpen(true)
                        }}
                      >
                        Edit
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        disabled={busyId === posting._id}
                        onClick={() => handleToggle(posting)}
                      >
                        {posting.is_active ? 'Deactivate' : 'Activate'}
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        className="border-destructive text-destructive hover:border-destructive hover:bg-destructive hover:text-white"
                        disabled={busyId === posting._id}
                        onClick={() => handleDelete(posting)}
                      >
                        Delete
                      </Button>
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </section>

          <aside className="space-y-6 lg:col-span-4">
            <section className="rounded-plate border border-line bg-white p-6">
              <p className="type-label text-ink-muted">DIRECTORY RECORD</p>
              {linkedAlumni ? (
                <div className="mt-4 space-y-2">
                  <p className="type-ui font-medium text-ink">
                    {linkedAlumni.full_name}
                  </p>
                  <p className="text-sm text-ink-muted">
                    {linkedAlumni.degree} &middot; Class of{' '}
                    {linkedAlumni.graduation_year}
                  </p>
                </div>
              ) : (
                <p className="type-ui mt-4 text-ink-muted">
                  Account #{user._id} is active. No separate directory entry
                  matches {user.email}.
                </p>
              )}
              <div className="mt-5">
                <Button asChild variant="outline" size="sm">
                  <Link to="/alumni">View alumni directory</Link>
                </Button>
              </div>
            </section>

            <section className="rounded-plate border border-line bg-white p-6">
              <p className="type-label text-ink-muted">ANNOUNCEMENTS</p>
              {announcements.length === 0 ? (
                <p className="type-ui mt-4 text-ink-muted">
                  No announcements yet.
                </p>
              ) : (
                <ul className="mt-4 space-y-4">
                  {announcements.slice(0, 4).map((item) => (
                    <li key={item._id} className="border-b border-line pb-3 last:border-b-0 last:pb-0">
                      <p className="type-ui font-medium text-ink">
                        {item.title}
                      </p>
                      <p className="mt-1 text-sm text-ink-muted">{item.body}</p>
                    </li>
                  ))}
                </ul>
              )}
            </section>
          </aside>
        </div>
      </div>
    </div>
  )
}
