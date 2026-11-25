import { useState, type FormEvent, type ReactNode } from 'react'
import { Link } from 'react-router-dom'
import { ArrowUpRight, Eye, EyeOff } from 'lucide-react'
import { registerUser } from '@/api'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import type { AcademicStatus, RegisterInput, User } from '@/types'

type AffiliationMode = 'academic_status' | 'graduation_date'

const ACADEMIC_STATUSES: {
  value: Exclude<AcademicStatus, 'alumni' | 'unverified'>
  label: string
  note: string
}[] = [
  { value: 'undergrad', label: 'Undergrad', note: 'Undergraduate student' },
  { value: 'grad', label: 'Grad', note: 'Graduate student (Master’s)' },
  { value: 'phd', label: 'PhD', note: 'Doctoral student or candidate' },
  { value: 'academic', label: 'Academic', note: 'Lecturer, researcher, or assistant' },
  { value: 'professor', label: 'Professor', note: 'Faculty member' },
]

const selectClass =
  'h-12 w-full rounded-field border border-line-strong bg-white px-4 text-base text-ink outline-none transition-colors hover:border-ink focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ink disabled:cursor-not-allowed disabled:opacity-50 md:text-sm'

function Field({
  id,
  label,
  hint,
  required = false,
  className,
  children,
}: {
  id: string
  label: string
  hint?: string
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
        {required ? (
          <span className="type-ui text-ink-muted">Required</span>
        ) : null}
      </div>
      {children}
      {hint ? <p className="type-ui text-ink-muted">{hint}</p> : null}
    </div>
  )
}

function formatRole(role: AcademicStatus): string {
  switch (role) {
    case 'unverified':
      return 'Unverified'
    case 'undergrad':
      return 'Undergrad'
    case 'grad':
      return 'Grad'
    case 'phd':
      return 'PhD'
    case 'academic':
      return 'Academic'
    case 'professor':
      return 'Professor'
    case 'alumni':
      return 'Alumni'
    case 'admin':
      return 'Admin'
  }
}

export function Register() {
  const [firstName, setFirstName] = useState('')
  const [lastName, setLastName] = useState('')
  const [age, setAge] = useState('')
  const [email, setEmail] = useState('')
  const [mode, setMode] = useState<AffiliationMode>('academic_status')
  const [academicStatus, setAcademicStatus] =
    useState<Exclude<AcademicStatus, 'alumni' | 'unverified'>>('undergrad')
  const [graduationDate, setGraduationDate] = useState('')
  const [password, setPassword] = useState('')
  const [showPassword, setShowPassword] = useState(false)
  const [submitting, setSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [registered, setRegistered] = useState<User | null>(null)

  function resetForm() {
    setFirstName('')
    setLastName('')
    setAge('')
    setEmail('')
    setMode('academic_status')
    setAcademicStatus('undergrad')
    setGraduationDate('')
    setPassword('')
    setShowPassword(false)
    setError(null)
    setRegistered(null)
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()

    const parsedAge = Number(age)
    if (!Number.isInteger(parsedAge) || parsedAge < 16 || parsedAge > 120) {
      setError('Enter an age between 16 and 120.')
      return
    }

    if (mode === 'graduation_date' && !graduationDate.trim()) {
      setError('Enter a graduation date.')
      return
    }

    if (password.length < 8) {
      setError('Password must be at least 8 characters.')
      return
    }

    const payload: RegisterInput = {
      first_name: firstName.trim(),
      last_name: lastName.trim(),
      age: parsedAge,
      email: email.trim(),
      password,
      ...(mode === 'academic_status'
        ? { academic_status: academicStatus }
        : { academic_status: 'alumni', graduation_date: graduationDate }),
    }

    setSubmitting(true)
    setError(null)
    try {
      const user = await registerUser(payload)
      setRegistered(user)
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e))
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <div>
      {/* 04 — Paper header. The drifting grid is this view's one ambient loop. */}
      <section
        aria-labelledby="register-heading"
        className="relative overflow-hidden bg-white"
      >
        <div
          aria-hidden="true"
          className="grid-live pointer-events-none absolute inset-0 opacity-[0.06]"
        />
        <div className="relative mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <p className="type-label border-b border-line py-6 text-ink-muted">
            04 / REGISTER
          </p>

          <div className="grid gap-10 py-12 md:grid-cols-12 md:py-16">
            <div className="reveal md:col-span-7">
              <h1 id="register-heading" className="type-h1 text-ink">
                Create an{' '}
                <em className="font-normal italic text-mulberry">account</em>.
              </h1>
              <p className="type-lede mt-6 max-w-[54ch] text-ink-muted">
                Register as a student, faculty member, or graduate. Choose your
                current academic status or enter your graduation date.
              </p>
            </div>
            <div className="flex flex-col items-start justify-end gap-2 md:col-span-5 md:items-end">
              <p className="type-label text-ink-muted">
                SELF-SERVICE / ACCOUNTS
              </p>
              <p className="type-ui text-ink-muted">
                Every account starts as unverified until government papers are
                checked.
              </p>
            </div>
          </div>
        </div>
      </section>

      <div className="mx-auto w-full max-w-[1280px] px-6 py-10 md:px-12 md:py-16">
        <div className="grid gap-10 lg:grid-cols-12 lg:gap-12">
          <aside className="space-y-6 lg:col-span-5">
            <div className="rounded-plate border border-line bg-white p-6">
              <p className="type-label text-ink-muted">01 / ELIGIBLE ROLES</p>
              <ul className="mt-6 divide-y divide-line border-t border-line">
                {ACADEMIC_STATUSES.map((item) => (
                  <li
                    key={item.value}
                    className="flex items-baseline justify-between gap-4 py-3"
                  >
                    <span className="type-ui font-medium text-ink">
                      {item.label}
                    </span>
                    <span className="type-ui text-right text-ink-muted">
                      {item.note}
                    </span>
                  </li>
                ))}
                <li className="flex items-baseline justify-between gap-4 py-3">
                  <span className="type-ui font-medium text-ink">Alumni</span>
                  <span className="type-ui text-right text-ink-muted">
                    Graduated with graduation date
                  </span>
                </li>
              </ul>
              <p className="type-label mt-6 text-ink-muted">
                SOURCE / AUTH POLICY &middot; 2026-11-25
              </p>
            </div>

            <div className="rounded-plate border border-line bg-bone/50 p-6">
              <p className="type-label text-ink-muted">02 / VERIFICATION</p>
              <p className="type-ui mt-4 text-ink-muted">
                New accounts receive the <strong className="font-medium text-ink">unverified</strong>{' '}
                role until an administrator verifies your identity and status
                with government papers. E-mail addresses are lowercased and must
                be unique; passwords require at least 8 characters.
              </p>
            </div>
          </aside>

          <div className="lg:col-span-7">
            {registered ? (
              <section
                aria-labelledby="register-success-heading"
                className="corner-ticks relative rounded-slab border border-line-strong bg-white p-6 md:p-8"
              >
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />

                <div className="inline-flex items-center gap-2">
                  <span className="status-dot text-ink" aria-hidden="true" />
                  <span className="type-label text-ink">
                    UNVERIFIED &middot; PENDING PAPERS
                  </span>
                </div>

                <h2
                  id="register-success-heading"
                  className="type-h2 mt-4 text-ink"
                >
                  Account recorded for {registered.full_name}.
                </h2>
                <p className="type-ui mt-2 text-ink-muted">
                  Recorded under #{registered._id} with role{' '}
                  <strong className="font-medium text-ink">unverified</strong>.
                  Government papers are required to promote your account.
                </p>

                <dl className="mt-8 grid gap-4 border-y border-line py-6 sm:grid-cols-2">
                  <div>
                    <dt className="type-label text-ink-muted">E-mail</dt>
                    <dd className="type-data mt-1 text-ink">
                      {registered.email}
                    </dd>
                  </div>
                  <div>
                    <dt className="type-label text-ink-muted">Age</dt>
                    <dd className="type-data mt-1 text-ink">
                      {registered.age}
                    </dd>
                  </div>
                  <div>
                    <dt className="type-label text-ink-muted">Assigned role</dt>
                    <dd className="type-data mt-1 text-ink">
                      {registered.roles.map(formatRole).join(', ')}
                    </dd>
                  </div>
                  {registered.requested_role ? (
                    <div>
                      <dt className="type-label text-ink-muted">
                        Requested role
                      </dt>
                      <dd className="type-data mt-1 text-ink">
                        {formatRole(registered.requested_role)}
                      </dd>
                    </div>
                  ) : null}
                  <div>
                    <dt className="type-label text-ink-muted">Verification</dt>
                    <dd className="type-data mt-1 text-ink">
                      {registered.verification_status} (government papers)
                    </dd>
                  </div>
                  {registered.graduation_date ? (
                    <div>
                      <dt className="type-label text-ink-muted">
                        Graduation date
                      </dt>
                      <dd className="type-data mt-1 text-ink">
                        {registered.graduation_date}
                      </dd>
                    </div>
                  ) : null}
                </dl>

                <div className="mt-8 flex flex-wrap items-center gap-6">
                  <Button asChild>
                    <Link to="/login">Sign in</Link>
                  </Button>
                  <Link
                    to="/alumni"
                    className="arrow-link type-ui text-ink hover:text-mulberry"
                  >
                    <span>Browse alumni</span>
                    <span className="arrow-ring">
                      <ArrowUpRight
                        size={20}
                        strokeWidth={1.5}
                        aria-hidden="true"
                      />
                    </span>
                  </Link>
                  <Button type="button" variant="outline" onClick={resetForm}>
                    Register another
                  </Button>
                </div>
              </section>
            ) : (
              <section
                aria-labelledby="register-form-heading"
                className="corner-ticks relative rounded-slab border border-line bg-white p-6 md:p-8"
              >
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />

                <p className="type-label text-ink-muted">03 / CREDENTIALS</p>
                <h2
                  id="register-form-heading"
                  className="type-h2 mt-4 text-ink"
                >
                  Registration details
                </h2>
                <p className="type-ui mt-2 text-ink-muted">
                  Complete every required field to register your account.
                </p>

                <form className="mt-8 space-y-6" onSubmit={handleSubmit}>
                  <div className="grid gap-6 sm:grid-cols-2">
                    <Field id="reg-first-name" label="Name" required>
                      <Input
                        id="reg-first-name"
                        name="first_name"
                        autoComplete="given-name"
                        required
                        placeholder="Ada"
                        value={firstName}
                        onChange={(e) => setFirstName(e.target.value)}
                      />
                    </Field>

                    <Field id="reg-last-name" label="Surname" required>
                      <Input
                        id="reg-last-name"
                        name="last_name"
                        autoComplete="family-name"
                        required
                        placeholder="Lovelace"
                        value={lastName}
                        onChange={(e) => setLastName(e.target.value)}
                      />
                    </Field>

                    <Field id="reg-age" label="Age" required>
                      <Input
                        id="reg-age"
                        name="age"
                        type="number"
                        inputMode="numeric"
                        min={16}
                        max={120}
                        step={1}
                        required
                        placeholder="24"
                        value={age}
                        onChange={(e) => setAge(e.target.value)}
                      />
                    </Field>

                    <Field id="reg-email" label="E-mail" required>
                      <Input
                        id="reg-email"
                        name="email"
                        type="email"
                        autoComplete="email"
                        required
                        placeholder="ada@example.edu"
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                      />
                    </Field>
                  </div>

                  <div className="space-y-4 rounded-plate border border-line bg-bone/35 p-5">
                    <div className="flex flex-wrap items-baseline justify-between gap-2">
                      <span className="type-label text-ink-muted">
                        Academic status or graduation date
                      </span>
                      <span className="type-ui text-ink-muted">Required</span>
                    </div>

                    <div
                      role="group"
                      aria-label="Choose academic status or graduation date"
                      className="flex flex-wrap gap-2"
                    >
                      <Button
                        type="button"
                        size="sm"
                        variant="outline"
                        aria-pressed={mode === 'academic_status'}
                        onClick={() => setMode('academic_status')}
                        className={
                          mode === 'academic_status'
                            ? 'border-ink bg-ink text-white hover:border-ink hover:bg-ink'
                            : undefined
                        }
                      >
                        Academic status
                      </Button>
                      <Button
                        type="button"
                        size="sm"
                        variant="outline"
                        aria-pressed={mode === 'graduation_date'}
                        onClick={() => setMode('graduation_date')}
                        className={
                          mode === 'graduation_date'
                            ? 'border-ink bg-ink text-white hover:border-ink hover:bg-ink'
                            : undefined
                        }
                      >
                        Graduation date (Alumni)
                      </Button>
                    </div>

                    {mode === 'academic_status' ? (
                      <Field
                        id="reg-academic-status"
                        label="Academic status"
                        hint="Select your current role at the university."
                        required
                      >
                        <select
                          id="reg-academic-status"
                          name="academic_status"
                          className={selectClass}
                          value={academicStatus}
                          onChange={(e) =>
                            setAcademicStatus(
                              e.target.value as Exclude<
                                AcademicStatus,
                                'alumni' | 'unverified'
                              >,
                            )
                          }
                        >
                          {ACADEMIC_STATUSES.map((status) => (
                            <option key={status.value} value={status.value}>
                              {status.label} &mdash; {status.note}
                            </option>
                          ))}
                        </select>
                      </Field>
                    ) : (
                      <Field
                        id="reg-graduation-date"
                        label="Graduation date"
                        hint="Registers your account with the Alumni role."
                        required
                      >
                        <Input
                          id="reg-graduation-date"
                          name="graduation_date"
                          type="date"
                          required
                          value={graduationDate}
                          onChange={(e) => setGraduationDate(e.target.value)}
                        />
                      </Field>
                    )}
                  </div>

                  <Field
                    id="reg-password"
                    label="Password"
                    hint="Use at least 8 characters."
                    required
                  >
                    <div className="relative flex items-center gap-2">
                      <Input
                        id="reg-password"
                        name="password"
                        type={showPassword ? 'text' : 'password'}
                        autoComplete="new-password"
                        minLength={8}
                        maxLength={128}
                        required
                        placeholder="Minimum 8 characters"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                      />
                      <Button
                        type="button"
                        variant="outline"
                        size="icon"
                        className="h-12 w-12 shrink-0 rounded-field"
                        aria-label={
                          showPassword ? 'Hide password' : 'Show password'
                        }
                        onClick={() => setShowPassword((prev) => !prev)}
                      >
                        {showPassword ? (
                          <EyeOff size={18} strokeWidth={1.5} />
                        ) : (
                          <Eye size={18} strokeWidth={1.5} />
                        )}
                      </Button>
                    </div>
                  </Field>

                  {error ? (
                    <div
                      role="alert"
                      className="flex flex-wrap items-baseline gap-x-3 gap-y-1 border border-destructive/40 bg-white px-4 py-3"
                    >
                      <span className="type-label text-destructive">Error</span>
                      <span className="type-ui text-destructive">{error}</span>
                    </div>
                  ) : null}

                  <div className="flex flex-wrap items-center gap-6 pt-2">
                    <Button type="submit" disabled={submitting}>
                      {submitting ? 'Creating account…' : 'Create account'}
                    </Button>
                    <Link
                      to="/login"
                      className="type-ui text-mulberry underline underline-offset-4 hover:decoration-rose"
                    >
                      Already have an account? Sign in
                    </Link>
                  </div>
                </form>
              </section>
            )}
          </div>
        </div>
      </div>
    </div>
  )
}
