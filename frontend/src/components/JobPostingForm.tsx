import { useEffect, useState, type FormEvent, type ReactNode } from 'react'
import { createJobPosting, replaceJobPosting } from '@/api'
import { useAuth } from '@/auth'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { isAdmin } from '@/navigation'
import type { Alumni, JobPosting, JobPostingInput, User } from '@/types'

function defaultAuthorId(
  posting: JobPosting | null,
  alumni: Alumni[],
  user: User | null,
): number {
  if (posting) return posting.author_id
  if (user && !isAdmin(user)) return user._id
  return user?._id ?? alumni[0]?._id ?? 0
}

function initialValues(
  posting: JobPosting | null,
  alumni: Alumni[],
  user: User | null,
): JobPostingInput {
  if (posting) {
    return {
      title: posting.title,
      company: posting.company,
      description: posting.description,
      location: posting.location,
      employment_type: posting.employment_type,
      apply_url: posting.apply_url,
      author_id: posting.author_id,
      is_active: posting.is_active,
    }
  }
  return {
    title: '',
    company: '',
    description: '',
    location: '',
    employment_type: '',
    apply_url: '',
    author_id: defaultAuthorId(posting, alumni, user),
    is_active: true,
  }
}

function Field({
  id,
  label,
  required = false,
  className,
  children,
}: {
  id: string
  label: string
  required?: boolean
  className?: string
  children: ReactNode
}) {
  return (
    <div className={`space-y-2 ${className ?? ''}`}>
      <div className="flex items-baseline justify-between gap-3">
        <label htmlFor={id} className="type-label text-ink-muted">
          {label}
        </label>
        {required ? <span className="type-ui text-ink-muted">Required</span> : null}
      </div>
      {children}
    </div>
  )
}

const textareaClass =
  'min-h-28 w-full rounded-field border border-line-strong bg-white px-4 py-3 text-base text-ink outline-none placeholder:text-ink-muted focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ink disabled:cursor-not-allowed disabled:opacity-50'

const selectClass =
  'h-12 w-full rounded-field border border-line-strong bg-white px-4 text-base text-ink outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ink disabled:cursor-not-allowed disabled:opacity-50'

interface JobPostingFormProps {
  posting: JobPosting | null
  alumni: Alumni[]
  alumniLoading: boolean
  onCancel: () => void
  onSaved: (posting: JobPosting, created: boolean) => void
}

export function JobPostingForm({
  posting,
  alumni,
  alumniLoading,
  onCancel,
  onSaved,
}: JobPostingFormProps) {
  const { user } = useAuth()
  const adminUser = isAdmin(user)
  const [values, setValues] = useState<JobPostingInput>(() =>
    initialValues(posting, alumni, user),
  )
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (!posting) {
      const nextAuthor = defaultAuthorId(null, alumni, user)
      if (nextAuthor > 0) {
        setValues((current) =>
          current.author_id === 0 ? { ...current, author_id: nextAuthor } : current,
        )
      }
    }
  }, [alumni, posting, user])

  function set<K extends keyof JobPostingInput>(
    key: K,
    value: JobPostingInput[K],
  ) {
    setValues((current) => ({ ...current, [key]: value }))
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    if (!values.author_id) {
      setError('Select an author.')
      return
    }
    setSaving(true)
    setError(null)
    try {
      const saved = posting
        ? await replaceJobPosting(posting._id, values)
        : await createJobPosting(values)
      onSaved(saved, !posting)
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e))
    } finally {
      setSaving(false)
    }
  }

  return (
    <section
      aria-labelledby="posting-form-heading"
      className="rounded-slab border border-line bg-white p-6 md:p-8"
    >
      <p className="type-label text-ink-muted">04 / POSTING</p>
      <h2 id="posting-form-heading" className="type-h2 mt-4 text-ink">
        {posting ? `Edit posting #${posting._id}` : 'New posting'}
      </h2>
      <p className="type-ui mt-2 text-ink-muted">
        Saving replaces every field. Fields marked Required must be completed.
      </p>

      <form className="mt-8 space-y-6" onSubmit={handleSubmit}>
        {adminUser && alumniLoading ? (
          <p className="type-ui text-ink-muted">Loading alumni…</p>
        ) : null}

        <div className="grid gap-6 sm:grid-cols-2">
          <Field id="posting-title" label="Title" required>
            <Input
              id="posting-title"
              required
              maxLength={120}
              value={values.title}
              onChange={(e) => set('title', e.target.value)}
            />
          </Field>
          <Field id="posting-company" label="Company" required>
            <Input
              id="posting-company"
              required
              value={values.company}
              onChange={(e) => set('company', e.target.value)}
            />
          </Field>
          <Field id="posting-location" label="Location">
            <Input
              id="posting-location"
              placeholder="Remote"
              value={values.location}
              onChange={(e) => set('location', e.target.value)}
            />
          </Field>
          <Field id="posting-type" label="Employment type" required>
            <Input
              id="posting-type"
              required
              placeholder="Full-time"
              value={values.employment_type}
              onChange={(e) => set('employment_type', e.target.value)}
            />
          </Field>
          <Field id="posting-author" label="Author" required>
            {adminUser ? (
              <select
                id="posting-author"
                className={selectClass}
                required
                value={values.author_id}
                onChange={(e) => set('author_id', Number(e.target.value))}
              >
                <option value={0} disabled>
                  Select an author
                </option>
                {user ? (
                  <option value={user._id}>
                    {user.full_name} (your account #{user._id})
                  </option>
                ) : null}
                {alumni.map((a) => (
                  <option key={a._id} value={a._id}>
                    {a.full_name} ({a.email})
                  </option>
                ))}
              </select>
            ) : (
              <Input
                id="posting-author"
                readOnly
                value={user ? `${user.full_name} (#${user._id})` : ''}
              />
            )}
          </Field>
          <Field id="posting-apply-url" label="Apply URL" required>
            <Input
              id="posting-apply-url"
              type="url"
              required
              placeholder="https://example.com/jobs/1"
              value={values.apply_url}
              onChange={(e) => set('apply_url', e.target.value)}
            />
          </Field>
          <Field
            id="posting-description"
            label="Description"
            className="sm:col-span-2"
          >
            <textarea
              id="posting-description"
              className={textareaClass}
              value={values.description}
              onChange={(e) => set('description', e.target.value)}
            />
          </Field>
        </div>

        <label className="type-ui flex items-center gap-3 text-ink">
          <input
            type="checkbox"
            className="size-4 accent-mulberry"
            checked={values.is_active}
            onChange={(e) => set('is_active', e.target.checked)}
          />
          Active &mdash; show this posting in the active list
        </label>

        {error ? (
          <p role="alert" className="type-ui text-destructive">
            {error}
          </p>
        ) : null}

        <div className="flex flex-wrap gap-3">
          <Button type="submit" disabled={saving || values.author_id === 0}>
            {saving ? 'Saving…' : posting ? 'Save changes' : 'Create posting'}
          </Button>
          <Button
            type="button"
            variant="outline"
            disabled={saving}
            onClick={onCancel}
          >
            Cancel
          </Button>
        </div>
      </form>
    </section>
  )
}
