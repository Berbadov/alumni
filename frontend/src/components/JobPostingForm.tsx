import { useEffect, useState, type FormEvent, type ReactNode } from 'react'
import { createJobPosting, replaceJobPosting } from '@/api'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import type { Alumni, JobPosting, JobPostingInput } from '@/types'

function initialValues(
  posting: JobPosting | null,
  alumni: Alumni[],
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
    author_id: alumni[0]?._id ?? 0,
    is_active: true,
  }
}

function Field({
  id,
  label,
  className,
  children,
}: {
  id: string
  label: string
  className?: string
  children: ReactNode
}) {
  return (
    <div className={`space-y-1.5 ${className ?? ''}`}>
      <label htmlFor={id} className="text-sm font-medium">
        {label}
      </label>
      {children}
    </div>
  )
}

const textareaClass =
  'min-h-24 w-full rounded-md border border-foreground/25 bg-background/40 px-3 py-2 text-sm shadow-xs outline-none placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50'

const selectClass =
  'h-9 w-full rounded-md border border-foreground/25 bg-background/40 px-3 text-sm shadow-xs outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50'

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
  const [values, setValues] = useState<JobPostingInput>(() =>
    initialValues(posting, alumni),
  )
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (!posting && alumni.length > 0) {
      setValues((current) =>
        current.author_id === 0
          ? { ...current, author_id: alumni[0]._id }
          : current,
      )
    }
  }, [alumni, posting])

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
    <Card>
      <CardHeader>
        <CardTitle>
          {posting ? `Edit posting #${posting._id}` : 'New job posting'}
        </CardTitle>
        <CardDescription>
          Saving replaces every field. Fields marked * are required.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form className="space-y-4" onSubmit={handleSubmit}>
          {alumniLoading ? (
            <p className="text-sm text-muted-foreground">Loading alumni…</p>
          ) : alumni.length === 0 ? (
            <p role="alert" className="text-sm text-destructive">
              No alumni records exist. Add an alumni record before you create a
              posting.
            </p>
          ) : null}
          <div className="grid gap-4 sm:grid-cols-2">
            <Field id="posting-title" label="Title *">
              <Input
                id="posting-title"
                required
                value={values.title}
                onChange={(e) => set('title', e.target.value)}
              />
            </Field>
            <Field id="posting-company" label="Company *">
              <Input
                id="posting-company"
                required
                value={values.company}
                onChange={(e) => set('company', e.target.value)}
              />
            </Field>
            <Field id="posting-location" label="Location *">
              <Input
                id="posting-location"
                required
                placeholder="Remote"
                value={values.location}
                onChange={(e) => set('location', e.target.value)}
              />
            </Field>
            <Field id="posting-type" label="Employment type *">
              <Input
                id="posting-type"
                required
                placeholder="Full-time"
                value={values.employment_type}
                onChange={(e) => set('employment_type', e.target.value)}
              />
            </Field>
            <Field id="posting-author" label="Author *">
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
                {alumni.map((a) => (
                  <option key={a._id} value={a._id}>
                    {a.full_name} ({a.email})
                  </option>
                ))}
              </select>
            </Field>
            <Field id="posting-apply-url" label="Apply URL *">
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
              label="Description *"
              className="sm:col-span-2"
            >
              <textarea
                id="posting-description"
                className={textareaClass}
                required
                value={values.description}
                onChange={(e) => set('description', e.target.value)}
              />
            </Field>
          </div>
          <label className="flex items-center gap-2 text-sm">
            <input
              type="checkbox"
              className="size-4 accent-primary"
              checked={values.is_active}
              onChange={(e) => set('is_active', e.target.checked)}
            />
            Active &mdash; show this posting in the active list
          </label>
          {error ? (
            <p role="alert" className="text-sm text-destructive">
              {error}
            </p>
          ) : null}
          <div className="flex gap-2">
            <Button
              type="submit"
              disabled={saving || alumniLoading || alumni.length === 0}
            >
              {saving
                ? 'Saving…'
                : posting
                  ? 'Save changes'
                  : 'Create posting'}
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
      </CardContent>
    </Card>
  )
}
