import { useEffect, useState } from 'react'
import {
  deleteJobPosting,
  fetchAlumni,
  fetchJobPostings,
  setJobPostingActive,
} from '@/api'
import { JobPostingForm } from '@/components/JobPostingForm'
import { Button } from '@/components/ui/button'
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import type { Alumni, JobPosting } from '@/types'

type Filter = 'all' | 'active' | 'inactive'

const FILTERS: { value: Filter; label: string }[] = [
  { value: 'all', label: 'All' },
  { value: 'active', label: 'Active' },
  { value: 'inactive', label: 'Inactive' },
]

function message(error: unknown): string {
  return error instanceof Error ? error.message : String(error)
}

export function Jobs() {
  const [postings, setPostings] = useState<JobPosting[]>([])
  const [alumni, setAlumni] = useState<Alumni[]>([])
  const [alumniLoading, setAlumniLoading] = useState(true)
  const [filter, setFilter] = useState<Filter>('all')
  const [formOpen, setFormOpen] = useState(false)
  const [editing, setEditing] = useState<JobPosting | null>(null)
  const [loading, setLoading] = useState(true)
  const [busyId, setBusyId] = useState<number | null>(null)
  const [listError, setListError] = useState<string | null>(null)
  const [alumniError, setAlumniError] = useState<string | null>(null)
  const [notice, setNotice] = useState<string | null>(null)
  const [reloadKey, setReloadKey] = useState(0)

  useEffect(() => {
    let cancelled = false
    fetchAlumni()
      .then((rows) => {
        if (!cancelled) setAlumni(rows)
      })
      .catch((error: unknown) => {
        if (!cancelled) setAlumniError(`Could not load alumni: ${message(error)}`)
      })
      .finally(() => {
        if (!cancelled) setAlumniLoading(false)
      })
    return () => {
      cancelled = true
    }
  }, [])

  useEffect(() => {
    let cancelled = false
    setLoading(true)
    setListError(null)
    fetchJobPostings(filter === 'all' ? undefined : filter === 'active')
      .then((rows) => {
        if (!cancelled) setPostings(rows)
      })
      .catch((error: unknown) => {
        if (!cancelled) {
          setPostings([])
          setListError(message(error))
        }
      })
      .finally(() => {
        if (!cancelled) setLoading(false)
      })
    return () => {
      cancelled = true
    }
  }, [filter, reloadKey])

  const error = listError ?? alumniError
  const authorNames = new Map(alumni.map((a) => [a._id, a.full_name]))

  function refresh() {
    setReloadKey((key) => key + 1)
  }

  function openCreate() {
    setEditing(null)
    setFormOpen(true)
    setNotice(null)
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }

  function openEdit(posting: JobPosting) {
    setEditing(posting)
    setFormOpen(true)
    setNotice(null)
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }

  function closeForm() {
    setFormOpen(false)
    setEditing(null)
  }

  function handleSaved(saved: JobPosting, created: boolean) {
    closeForm()
    setNotice(`${created ? 'Created' : 'Saved'} “${saved.title}”.`)
    setListError(null)
    refresh()
  }

  async function handleToggle(posting: JobPosting) {
    setBusyId(posting._id)
    setNotice(null)
    setListError(null)
    try {
      const saved = await setJobPostingActive(posting._id, !posting.is_active)
      setNotice(
        saved.is_active
          ? `Activated “${saved.title}”.`
          : `Deactivated “${saved.title}”.`,
      )
      refresh()
    } catch (e: unknown) {
      setListError(message(e))
    } finally {
      setBusyId(null)
    }
  }

  async function handleDelete(posting: JobPosting) {
    if (!window.confirm(`Delete “${posting.title}”?`)) {
      return
    }
    setBusyId(posting._id)
    setNotice(null)
    setListError(null)
    try {
      await deleteJobPosting(posting._id)
      if (editing?._id === posting._id) {
        closeForm()
      }
      setNotice(`Deleted “${posting.title}”.`)
      refresh()
    } catch (e: unknown) {
      setListError(message(e))
    } finally {
      setBusyId(null)
    }
  }

  return (
    <div>
      {/* 03 — Paper header. The drifting grid is this view's one ambient loop. */}
      <section aria-labelledby="jobs-heading" className="relative overflow-hidden bg-white">
        <div
          aria-hidden="true"
          className="grid-live pointer-events-none absolute inset-0 opacity-[0.06]"
        />
        <div className="relative mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <p className="type-label border-b border-line py-6 text-ink-muted">
            03 / OFFERS
          </p>

          <div className="grid gap-10 py-12 md:grid-cols-12 md:py-16">
            <div className="reveal md:col-span-7">
              <h1 id="jobs-heading" className="type-h1 text-ink">
                Job <em className="font-normal italic text-mulberry">postings</em>.
              </h1>
              <p className="type-lede mt-6 max-w-[52ch] text-ink-muted">
                Offers published by alumni. Filter by status, then edit or
                deactivate a posting.
              </p>
            </div>

            <div className="flex flex-col items-start gap-4 md:col-span-5 md:items-end md:pt-2">
              <Button onClick={openCreate}>New posting</Button>
              <p className="type-label text-left text-ink-muted md:text-right">
                OPEN / ACTIVE / INACTIVE
              </p>
            </div>
          </div>
        </div>
      </section>

      <div className="mx-auto w-full max-w-[1280px] space-y-8 px-6 py-10 md:px-12 md:py-14">
        {error ? (
          <div role="alert" className="flex flex-wrap items-baseline gap-x-3 gap-y-1">
            <span className="type-label text-destructive">Error</span>
            <span className="type-ui text-destructive">{error}</span>
          </div>
        ) : null}

        {notice ? (
          <div
            role="status"
            className="flex items-start gap-3 rounded-plate border border-line-strong bg-white px-4 py-3"
          >
            <span className="live-dot mt-1.5" aria-hidden="true" />
            <p className="type-ui text-ink">{notice}</p>
          </div>
        ) : null}

        {formOpen ? (
          <JobPostingForm
            key={editing?._id ?? 'new'}
            posting={editing}
            alumni={alumni}
            alumniLoading={alumniLoading}
            onCancel={closeForm}
            onSaved={handleSaved}
          />
        ) : null}

        <div className="flex flex-wrap items-end justify-between gap-4">
          <div
            className="flex flex-wrap gap-2"
            role="group"
            aria-label="Filter by status"
          >
            {FILTERS.map((option) => (
              <Button
                key={option.value}
                size="sm"
                variant="outline"
                aria-pressed={filter === option.value}
                onClick={() => setFilter(option.value)}
                className={
                  filter === option.value
                    ? 'border-ink bg-ink text-white hover:border-ink hover:bg-ink'
                    : undefined
                }
              >
                <span className="type-label">{option.label}</span>
              </Button>
            ))}
          </div>
          <p className="type-data tabular-nums text-ink-muted">
            {postings.length} {postings.length === 1 ? 'posting' : 'postings'}
          </p>
        </div>

        {loading ? (
          <div className="rounded-plate border border-line bg-white px-5 py-4">
            <p className="type-ui text-ink-muted">Loading postings…</p>
          </div>
        ) : postings.length === 0 ? (
          <div className="rounded-plate border border-dotted border-line-strong px-6 py-16 text-center">
            <p className="type-label text-ink-muted">NO POSTINGS</p>
            <p className="type-ui mx-auto mt-4 max-w-[46ch] text-ink-muted">
              No postings in this view;{' '}
              <Button
                variant="link"
                className="h-auto p-0 align-baseline"
                onClick={openCreate}
              >
                create the first one.
              </Button>
            </p>
          </div>
        ) : (
          <section aria-label="Postings list" className="rounded-plate border border-line bg-white">
            <div className="flex flex-wrap items-center justify-between gap-3 border-b border-line px-5 py-4">
              <p className="type-label text-ink-muted">05 / LIST</p>
              <p className="type-label text-ink-muted">
                SOURCE / ALUMNI POSTINGS &middot; 2026-11-18
              </p>
            </div>

            <Table className="table-brand">
              <TableCaption className="sr-only">Job postings</TableCaption>
              <TableHeader>
                <TableRow>
                  <TableHead scope="col">Role</TableHead>
                  <TableHead scope="col">Location</TableHead>
                  <TableHead scope="col">Type</TableHead>
                  <TableHead scope="col">Author</TableHead>
                  <TableHead scope="col">Status</TableHead>
                  <TableHead scope="col" className="text-right">
                    Actions
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {postings.map((posting) => (
                  <TableRow key={posting._id}>
                    <TableCell className="whitespace-normal">
                      <span className="type-ui font-medium text-ink">
                        {posting.title}
                      </span>
                      <span className="type-ui mt-1 block text-ink-muted">
                        {posting.company}
                      </span>
                    </TableCell>
                    <TableCell className="type-data text-ink">
                      {posting.location || 'Not recorded.'}
                    </TableCell>
                    <TableCell className="type-data text-ink">
                      {posting.employment_type || 'Not recorded.'}
                    </TableCell>
                    <TableCell className="type-data text-ink">
                      {authorNames.get(posting.author_id) ??
                        `Alumni #${posting.author_id}`}
                    </TableCell>
                    <TableCell>
                      <span className="type-ui inline-flex items-center gap-2 lowercase text-ink">
                        {posting.is_active ? (
                          <span className="live-dot" aria-hidden="true" />
                        ) : (
                          <span
                            className="status-dot text-ink-muted"
                            aria-hidden="true"
                          />
                        )}
                        {posting.is_active ? 'active' : 'inactive'}
                      </span>
                    </TableCell>
                    <TableCell className="text-right">
                      <div className="flex flex-wrap justify-end gap-2">
                        <Button asChild variant="ghost" size="sm">
                          <a
                            href={posting.apply_url}
                            target="_blank"
                            rel="noreferrer"
                          >
                            Apply
                          </a>
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => openEdit(posting)}
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
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </section>
        )}
      </div>
    </div>
  )
}
