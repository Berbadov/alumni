import { useState, type FormEvent } from 'react'
import { Link } from 'react-router-dom'
import { ArrowUpRight, Eye, EyeOff } from 'lucide-react'
import { loginUser } from '@/api'
import { useAuth } from '@/auth'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { primaryWorkspaceFor } from '@/navigation'

export function Login() {
  const { user, setUser, logout } = useAuth()
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [showPassword, setShowPassword] = useState(false)
  const [submitting, setSubmitting] = useState(false)
  const [signingOut, setSigningOut] = useState(false)
  const [error, setError] = useState<string | null>(null)

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setSubmitting(true)
    setError(null)
    try {
      const session = await loginUser({
        email: email.trim(),
        password,
      })
      setUser(session.user)
      setPassword('')
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e))
    } finally {
      setSubmitting(false)
    }
  }

  async function handleLogout() {
    setSigningOut(true)
    setError(null)
    try {
      await logout()
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e))
    } finally {
      setSigningOut(false)
    }
  }

  return (
    <div>
      {/* 05 — Paper header. The drifting grid is this view's one ambient loop. */}
      <section
        aria-labelledby="login-heading"
        className="relative overflow-hidden bg-white"
      >
        <div
          aria-hidden="true"
          className="grid-live pointer-events-none absolute inset-0 opacity-[0.06]"
        />
        <div className="relative mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <p className="type-label border-b border-line py-6 text-ink-muted">
            05 / SIGN IN
          </p>

          <div className="grid gap-10 py-12 md:grid-cols-12 md:py-16">
            <div className="reveal md:col-span-7">
              <h1 id="login-heading" className="type-h1 text-ink">
                Sign in to the{' '}
                <em className="font-normal italic text-mulberry">ledger</em>.
              </h1>
              <p className="type-lede mt-6 max-w-[54ch] text-ink-muted">
                Open your session with your registered e-mail and password.
              </p>
            </div>
            <div className="flex flex-col items-start justify-end gap-2 md:col-span-5 md:items-end">
              <p className="type-label text-ink-muted">SESSIONS / HTTPONLY</p>
              <p className="type-ui text-ink-muted">
                Sessions remain valid for 7 days unless you sign out.
              </p>
            </div>
          </div>
        </div>
      </section>

      <div className="mx-auto w-full max-w-[1280px] px-6 py-10 md:px-12 md:py-16">
        <div className="grid gap-10 lg:grid-cols-12 lg:gap-12">
          <aside className="space-y-6 lg:col-span-5">
            <div className="rounded-plate border border-line bg-white p-6">
              <p className="type-label text-ink-muted">01 / ACCESS STATUS</p>
              <p className="type-ui mt-4 text-ink-muted">
                New accounts hold the{' '}
                <strong className="font-medium text-ink">unverified</strong>{' '}
                role until an administrator checks government papers. You can
                sign in now to check your verification state.
              </p>
              <p className="type-label mt-6 text-ink-muted">
                SOURCE / AUTH SESSION &middot; 2026-11-25
              </p>
            </div>
          </aside>

          <div className="lg:col-span-7">
            {user ? (
              <section
                aria-labelledby="session-heading"
                className="corner-ticks relative rounded-slab border border-line-strong bg-white p-6 md:p-8"
              >
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />

                <div className="inline-flex items-center gap-2">
                  {user.verification_status === 'verified' ? (
                    <span className="live-dot" aria-hidden="true" />
                  ) : (
                    <span className="status-dot text-ink" aria-hidden="true" />
                  )}
                  <span className="type-label text-ink">
                    {user.verification_status === 'verified'
                      ? 'ACTIVE SESSION'
                      : 'SIGNED IN · UNVERIFIED'}
                  </span>
                </div>

                <h2 id="session-heading" className="type-h2 mt-4 text-ink">
                  Signed in as {user.full_name}.
                </h2>
                <p className="type-ui mt-2 text-ink-muted">
                  Account #{user._id} &middot; {user.email}
                </p>

                <dl className="mt-8 grid gap-4 border-y border-line py-6 sm:grid-cols-2">
                  <div>
                    <dt className="type-label text-ink-muted">Assigned role</dt>
                    <dd className="type-data mt-1 text-ink">
                      {user.roles.join(', ')}
                    </dd>
                  </div>
                  {user.requested_role ? (
                    <div>
                      <dt className="type-label text-ink-muted">
                        Requested role
                      </dt>
                      <dd className="type-data mt-1 text-ink">
                        {user.requested_role}
                      </dd>
                    </div>
                  ) : null}
                  <div>
                    <dt className="type-label text-ink-muted">Verification</dt>
                    <dd className="type-data mt-1 text-ink">
                      {user.verification_status}
                    </dd>
                  </div>
                </dl>

                {error ? (
                  <div
                    role="alert"
                    className="mt-6 flex flex-wrap items-baseline gap-x-3 gap-y-1 border border-destructive/40 bg-white px-4 py-3"
                  >
                    <span className="type-label text-destructive">Error</span>
                    <span className="type-ui text-destructive">{error}</span>
                  </div>
                ) : null}

                <div className="mt-8 flex flex-wrap items-center gap-6">
                  <Button asChild>
                    <Link to={primaryWorkspaceFor(user).to}>
                      {primaryWorkspaceFor(user).label}
                    </Link>
                  </Button>
                  <Button asChild variant="outline">
                    <Link to="/account">Account settings</Link>
                  </Button>
                  <Button
                    type="button"
                    variant="outline"
                    disabled={signingOut}
                    onClick={handleLogout}
                  >
                    {signingOut ? 'Signing out…' : 'Sign out'}
                  </Button>
                  <Link
                    to="/"
                    className="arrow-link type-ui text-ink hover:text-mulberry"
                  >
                    <span>Back to home</span>
                    <span className="arrow-ring">
                      <ArrowUpRight
                        size={20}
                        strokeWidth={1.5}
                        aria-hidden="true"
                      />
                    </span>
                  </Link>
                </div>
              </section>
            ) : (
              <section
                aria-labelledby="login-form-heading"
                className="corner-ticks relative rounded-slab border border-line bg-white p-6 md:p-8"
              >
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />

                <p className="type-label text-ink-muted">02 / CREDENTIALS</p>
                <h2 id="login-form-heading" className="type-h2 mt-4 text-ink">
                  Account sign in
                </h2>
                <p className="type-ui mt-2 text-ink-muted">
                  Enter the e-mail and password from your registration.
                </p>

                <form className="mt-8 space-y-6" onSubmit={handleSubmit}>
                  <div className="space-y-2">
                    <div className="flex items-baseline justify-between gap-3">
                      <label
                        htmlFor="login-email"
                        className="type-label text-ink-muted"
                      >
                        E-mail
                      </label>
                      <span className="type-ui text-ink-muted">Required</span>
                    </div>
                    <Input
                      id="login-email"
                      name="email"
                      type="email"
                      autoComplete="email"
                      required
                      placeholder="ada@example.edu"
                      value={email}
                      onChange={(e) => setEmail(e.target.value)}
                    />
                  </div>

                  <div className="space-y-2">
                    <div className="flex items-baseline justify-between gap-3">
                      <label
                        htmlFor="login-password"
                        className="type-label text-ink-muted"
                      >
                        Password
                      </label>
                      <span className="type-ui text-ink-muted">Required</span>
                    </div>
                    <div className="relative flex items-center gap-2">
                      <Input
                        id="login-password"
                        name="password"
                        type={showPassword ? 'text' : 'password'}
                        autoComplete="current-password"
                        required
                        placeholder="Your password"
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
                  </div>

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
                      {submitting ? 'Signing in…' : 'Sign in'}
                    </Button>
                    <Link
                      to="/register"
                      className="type-ui text-mulberry underline underline-offset-4 hover:decoration-rose"
                    >
                      Need an account? Register
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
