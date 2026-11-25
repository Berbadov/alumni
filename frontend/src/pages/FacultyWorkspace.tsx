import { useEffect, useState, type FormEvent } from 'react'
import { Link } from 'react-router-dom'
import {
  createAnnouncement,
  deleteAnnouncement,
  fetchAnnouncements,
  replaceAnnouncement,
} from '@/api'
import { useAuth } from '@/auth'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  canManageAnnouncements,
  canModifyAnnouncement,
  isAdmin,
} from '@/navigation'
import type { Announcement } from '@/types'

function message(error: unknown): string {
  return error instanceof Error ? error.message : String(error)
}

const textareaClass =
  'min-h-28 w-full rounded-field border border-line-strong bg-white px-4 py-3 text-base text-ink outline-none placeholder:text-ink-muted focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ink disabled:cursor-not-allowed disabled:opacity-50'

export function FacultyWorkspace() {
  const { user } = useAuth()
  const [announcements, setAnnouncements] = useState<Announcement[]>([])
  const [loading, setLoading] = useState(true)
  const [formOpen, setFormOpen] = useState(false)
  const [editing, setEditing] = useState<Announcement | null>(null)
  const [title, setTitle] = useState('')
  const [body, setBody] = useState('')
  const [saving, setSaving] = useState(false)
  const [busyId, setBusyId] = useState<number | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [notice, setNotice] = useState<string | null>(null)
  const [reloadKey, setReloadKey] = useState(0)

  useEffect(() => {
    let cancelled = false
    setLoading(true)
    fetchAnnouncements()
      .then((rows) => {
        if (!cancelled) {
          setAnnouncements(rows)
          setError(null)
        }
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

  if (!user) return null
  const canAuthor = canManageAnnouncements(user)
  const adminUser = isAdmin(user)

  function openCreate() {
    setEditing(null)
    setTitle('')
    setBody('')
    setFormOpen(true)
    setError(null)
    setNotice(null)
  }

  function openEdit(item: Announcement) {
    setEditing(item)
    setTitle(item.title)
    setBody(item.body)
    setFormOpen(true)
    setError(null)
    setNotice(null)
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setSaving(true)
    setError(null)
    try {
      const payload = {
        title: title.trim(),
        body: body.trim(),
        author_id: editing ? editing.author_id : user!._id,
      }
      const saved = editing
        ? await replaceAnnouncement(editing._id, payload)
        : await createAnnouncement(payload)
      setFormOpen(false)
      setEditing(null)
      setNotice(`${editing ? 'Updated' : 'Published'} “${saved.title}”.`)
      setReloadKey((k) => k + 1)
    } catch (e: unknown) {
      setError(message(e))
    } finally {
      setSaving(false)
    }
  }

  async function handleDelete(item: Announcement) {
    if (!window.confirm(`Delete announcement “${item.title}”?`)) return
    setBusyId(item._id)
    setError(null)
    try {
      await deleteAnnouncement(item._id)
      if (editing?._id === item._id) {
        setEditing(null)
        setFormOpen(false)
      }
      setNotice(`Deleted “${item.title}”.`)
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
        aria-labelledby="faculty-heading"
        className="relative overflow-hidden bg-white"
      >
        <div
          aria-hidden="true"
          className="grid-live pointer-events-none absolute inset-0 opacity-[0.06]"
        />
        <div className="relative mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <p className="type-label border-b border-line py-6 text-ink-muted">
            08 / FACULTY WORKSPACE
          </p>

          <div className="grid gap-10 py-12 md:grid-cols-12 md:py-16">
            <div className="reveal md:col-span-7">
              <h1 id="faculty-heading" className="type-h1 text-ink">
                Faculty{' '}
                <em className="font-normal italic text-mulberry">
                  announcements
                </em>
                .
              </h1>
              <p className="type-lede mt-6 max-w-[54ch] text-ink-muted">
                Publish notices for students and alumni, and maintain the ones
                you authored.
              </p>
            </div>
            <div className="flex flex-col items-start justify-end gap-3 md:col-span-5 md:items-end">
              <div className="flex flex-wrap gap-3">
                {canAuthor ? (
                  <Button onClick={openCreate}>New announcement</Button>
                ) : null}
                {adminUser ? (
                  <Button asChild variant="outline">
                    <Link to="/admin">Open admin</Link>
                  </Button>
                ) : null}
              </div>
              <p className="type-label text-ink-muted">
                ROLE / {user.roles.join(' · ').toUpperCase()}
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
          <section
            aria-labelledby="announcement-form-heading"
            className="corner-ticks relative rounded-slab border border-line-strong bg-white p-6 md:p-8"
          >
            <span className="tick" aria-hidden="true" />
            <span className="tick" aria-hidden="true" />
            <span className="tick" aria-hidden="true" />
            <span className="tick" aria-hidden="true" />

            <p className="type-label text-ink-muted">01 / COMPOSE</p>
            <h2
              id="announcement-form-heading"
              className="type-h2 mt-3 text-ink"
            >
              {editing ? `Edit announcement #${editing._id}` : 'New announcement'}
            </h2>

            <form className="mt-6 space-y-5" onSubmit={handleSubmit}>
              <div className="space-y-2">
                <label htmlFor="ann-title" className="type-label text-ink-muted">
                  Title
                </label>
                <Input
                  id="ann-title"
                  required
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                />
              </div>
              <div className="space-y-2">
                <label htmlFor="ann-body" className="type-label text-ink-muted">
                  Body
                </label>
                <textarea
                  id="ann-body"
                  required
                  className={textareaClass}
                  value={body}
                  onChange={(e) => setBody(e.target.value)}
                />
              </div>
              <div className="flex flex-wrap gap-3">
                <Button type="submit" disabled={saving}>
                  {saving
                    ? 'Saving…'
                    : editing
                      ? 'Save announcement'
                      : 'Publish announcement'}
                </Button>
                <Button
                  type="button"
                  variant="outline"
                  disabled={saving}
                  onClick={() => {
                    setFormOpen(false)
                    setEditing(null)
                  }}
                >
                  Cancel
                </Button>
              </div>
            </form>
          </section>
        ) : null}

        <section className="rounded-plate border border-line bg-white p-6">
          <div className="flex flex-wrap items-baseline justify-between gap-3 border-b border-line pb-4">
            <h2 className="type-h3 text-ink">All announcements</h2>
            <span className="type-data text-ink-muted">
              {announcements.length} recorded
            </span>
          </div>

          {loading ? (
            <p className="type-ui mt-6 text-ink-muted">
              Loading announcements…
            </p>
          ) : announcements.length === 0 ? (
            <p className="type-ui mt-6 text-ink-muted">
              No announcements published yet.
            </p>
          ) : (
            <ul className="mt-4 divide-y divide-line">
              {announcements.map((item) => {
                const editable = canModifyAnnouncement(user, item.author_id)
                return (
                  <li
                    key={item._id}
                    className="flex flex-wrap items-start justify-between gap-4 py-5"
                  >
                    <div className="max-w-2xl">
                      <p className="type-ui font-medium text-ink">
                        {item.title}
                      </p>
                      <p className="mt-1 text-sm text-ink-muted">{item.body}</p>
                      <p className="type-label mt-2 text-ink-muted">
                        #{item._id} &middot;{' '}
                        {item.author_id === user._id
                          ? `${user.full_name} (you)`
                          : `Author #${item.author_id}`}
                      </p>
                    </div>
                    {editable ? (
                      <div className="flex flex-wrap gap-2">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => openEdit(item)}
                        >
                          Edit
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          className="border-destructive text-destructive hover:border-destructive hover:bg-destructive hover:text-white"
                          disabled={busyId === item._id}
                          onClick={() => handleDelete(item)}
                        >
                          Delete
                        </Button>
                      </div>
                    ) : null}
                  </li>
                )
              })}
            </ul>
          )}
        </section>
      </div>
    </div>
  )
}
