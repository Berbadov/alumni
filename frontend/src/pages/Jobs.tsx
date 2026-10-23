import { useEffect, useState } from 'react'
import {
  deleteJobPosting,
  fetchAlumni,
  fetchJobPostings,
  setJobPostingActive,
} from '@/api'
import { JobPostingForm } from '@/components/JobPostingForm'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import {
  Table,
  TableBody,
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
    <div className="space-y-6">
      <div className="flex flex-wrap items-end justify-between gap-3">
        <div>
          <h1 className="text-2xl font-semibold tracking-tight">Job postings</h1>
          <p className="text-muted-foreground">
            Offers published by alumni. Filter by status, then edit or
            deactivate a posting.
          </p>
        </div>
        <Button onClick={openCreate}>New posting</Button>
      </div>

      {error ? (
        <p role="alert" className="text-destructive">
          Error: {error}
        </p>
      ) : null}
      {notice ? (
        <p
          role="status"
          className="rounded-md border border-primary/40 bg-primary/10 px-3 py-2 text-sm"
        >
          {notice}
        </p>
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

      <div className="flex flex-wrap items-center justify-between gap-3">
        <div className="flex gap-2" role="group" aria-label="Filter by status">
          {FILTERS.map((option) => (
            <Button
              key={option.value}
              size="sm"
              variant={filter === option.value ? 'default' : 'outline'}
              aria-pressed={filter === option.value}
              onClick={() => setFilter(option.value)}
            >
              {option.label}
            </Button>
          ))}
        </div>
        <p className="text-sm text-muted-foreground">
          {postings.length} {postings.length === 1 ? 'posting' : 'postings'}
        </p>
      </div>

      {loading ? (
        <p className="text-muted-foreground">Loading job postings…</p>
      ) : postings.length === 0 ? (
        <Card>
          <CardContent className="py-2 text-center text-muted-foreground">
            No job postings in this view.{' '}
            <Button variant="link" className="px-1" onClick={openCreate}>
              Create the first one
            </Button>
          </CardContent>
        </Card>
      ) : (
        <Card className="py-2">
          <CardContent className="px-2">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Role</TableHead>
                  <TableHead>Location</TableHead>
                  <TableHead>Type</TableHead>
                  <TableHead>Author</TableHead>
                  <TableHead>Status</TableHead>
                  <TableHead className="text-right">Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {postings.map((posting) => (
                  <TableRow key={posting._id}>
                    <TableCell className="whitespace-normal">
                      <div className="font-medium">{posting.title}</div>
                      <div className="text-sm text-muted-foreground">
                        {posting.company}
                      </div>
                    </TableCell>
                    <TableCell>{posting.location}</TableCell>
                    <TableCell>{posting.employment_type}</TableCell>
                    <TableCell>
                      {authorNames.get(posting.author_id) ??
                        `Alumni #${posting.author_id}`}
                    </TableCell>
                    <TableCell>
                      <span
                        className={
                          posting.is_active
                            ? 'inline-flex rounded-full bg-primary px-2 py-0.5 text-xs font-medium text-primary-foreground'
                            : 'inline-flex rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground'
                        }
                      >
                        {posting.is_active ? 'Active' : 'Inactive'}
                      </span>
                    </TableCell>
                    <TableCell className="text-right">
                      <div className="flex justify-end gap-1">
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
                          variant="destructive"
                          size="sm"
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
          </CardContent>
        </Card>
      )}
    </div>
  )
}
