import { useEffect, useState, type FormEvent } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { ArrowUpRight } from 'lucide-react'
import { updateCurrentUser } from '@/api'
import { useAuth } from '@/auth'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { isVerifiedMember, primaryWorkspaceFor } from '@/navigation'

function message(error: unknown): string {
  return error instanceof Error ? error.message : String(error)
}

export function Account() {
  const { user, setUser, logout } = useAuth()
  const navigate = useNavigate()

  const [firstName, setFirstName] = useState('')
  const [lastName, setLastName] = useState('')
  const [phoneNumber, setPhoneNumber] = useState('')
  const [graduationDate, setGraduationDate] = useState('')
  const [savingProfile, setSavingProfile] = useState(false)
  const [profileError, setProfileError] = useState<string | null>(null)
  const [profileNotice, setProfileNotice] = useState<string | null>(null)

  const [currentPassword, setCurrentPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')
  const [confirmPassword, setConfirmPassword] = useState('')
  const [savingPassword, setSavingPassword] = useState(false)
  const [passwordError, setPasswordError] = useState<string | null>(null)
  const [passwordNotice, setPasswordNotice] = useState<string | null>(null)

  const [signingOut, setSigningOut] = useState(false)

  useEffect(() => {
    if (!user) return
    setFirstName(user.first_name)
    setLastName(user.last_name)
    setPhoneNumber(user.phone_number ?? '')
    setGraduationDate(user.graduation_date ?? '')
  }, [user])

  if (!user) {
    return null
  }

  const verifiedMember = isVerifiedMember(user)
  const workspace = primaryWorkspaceFor(user)

  async function handleProfileSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setSavingProfile(true)
    setProfileError(null)
    setProfileNotice(null)
    try {
      const updated = await updateCurrentUser({
        first_name: firstName.trim(),
        last_name: lastName.trim(),
        phone_number: phoneNumber.trim(),
        graduation_date: graduationDate.trim(),
      })
      setUser(updated)
      setProfileNotice('Profile saved.')
    } catch (e: unknown) {
      setProfileError(message(e))
    } finally {
      setSavingProfile(false)
    }
  }

  async function handlePasswordSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setPasswordError(null)
    setPasswordNotice(null)
    if (newPassword !== confirmPassword) {
      setPasswordError('New passwords do not match.')
      return
    }
    setSavingPassword(true)
    try {
      const updated = await updateCurrentUser({
        current_password: currentPassword,
        new_password: newPassword,
      })
      setUser(updated)
      setCurrentPassword('')
      setNewPassword('')
      setConfirmPassword('')
      setPasswordNotice('Password updated.')
    } catch (e: unknown) {
      setPasswordError(message(e))
    } finally {
      setSavingPassword(false)
    }
  }

  async function handleLogout() {
    setSigningOut(true)
    try {
      await logout()
      navigate('/login')
    } finally {
      setSigningOut(false)
    }
  }

  return (
    <div>
      <section
        aria-labelledby="account-heading"
        className="relative overflow-hidden bg-white"
      >
        <div
          aria-hidden="true"
          className="grid-live pointer-events-none absolute inset-0 opacity-[0.06]"
        />
        <div className="relative mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <p className="type-label border-b border-line py-6 text-ink-muted">
            07 / ACCOUNT
          </p>

          <div className="grid gap-10 py-12 md:grid-cols-12 md:py-16">
            <div className="reveal md:col-span-7">
              <h1 id="account-heading" className="type-h1 text-ink">
                Personal{' '}
                <em className="font-normal italic text-mulberry">ledger</em>.
              </h1>
              <p className="type-lede mt-6 max-w-[54ch] text-ink-muted">
                Manage your own identity details, review verification channels,
                and update your password.
              </p>
            </div>
            <div className="flex flex-col items-start justify-end gap-3 md:col-span-5 md:items-end">
              <div className="inline-flex items-center gap-2">
                {verifiedMember && user.verification_status === 'verified' ? (
                  <span className="live-dot" aria-hidden="true" />
                ) : (
                  <span
                    className="status-dot text-ink-muted"
                    aria-hidden="true"
                  />
                )}
                <span className="type-label text-ink">
                  {user.roles.join(' · ')}
                </span>
              </div>
              <p className="type-ui text-ink-muted">
                Account #{user._id} &middot; {user.email}
              </p>
            </div>
          </div>
        </div>
      </section>

      <div className="mx-auto w-full max-w-[1280px] px-6 py-10 md:px-12 md:py-16">
        <div className="grid gap-10 lg:grid-cols-12 lg:gap-12">
          <aside className="space-y-6 lg:col-span-5">
            <section className="rounded-plate border border-line bg-white p-6">
              <p className="type-label text-ink-muted">01 / IDENTITY</p>
              <dl className="mt-6 space-y-4">
                <div className="flex items-baseline justify-between gap-4 border-b border-line pb-3">
                  <dt className="type-label text-ink-muted">Full name</dt>
                  <dd className="type-data text-ink">{user.full_name}</dd>
                </div>
                <div className="flex items-baseline justify-between gap-4 border-b border-line pb-3">
                  <dt className="type-label text-ink-muted">Age</dt>
                  <dd className="type-data text-ink">{user.age}</dd>
                </div>
                <div className="flex items-baseline justify-between gap-4 border-b border-line pb-3">
                  <dt className="type-label text-ink-muted">Assigned roles</dt>
                  <dd className="type-data text-ink">{user.roles.join(', ')}</dd>
                </div>
                {user.requested_role ? (
                  <div className="flex items-baseline justify-between gap-4 border-b border-line pb-3">
                    <dt className="type-label text-ink-muted">Requested role</dt>
                    <dd className="type-data text-ink">{user.requested_role}</dd>
                  </div>
                ) : null}
                <div className="flex items-baseline justify-between gap-4 border-b border-line pb-3">
                  <dt className="type-label text-ink-muted">Verification</dt>
                  <dd className="type-data text-ink">
                    {user.verification_status}
                    {user.verification_method
                      ? ` (${user.verification_method})`
                      : ''}
                  </dd>
                </div>
                <div className="flex items-baseline justify-between gap-4 border-b border-line pb-3">
                  <dt className="type-label text-ink-muted">Email check</dt>
                  <dd className="type-data text-ink-muted">
                    {user.email_verified ? 'verified' : 'inactive'}
                  </dd>
                </div>
                <div className="flex items-baseline justify-between gap-4">
                  <dt className="type-label text-ink-muted">SMS check</dt>
                  <dd className="type-data text-ink-muted">
                    {user.phone_verified ? 'verified' : 'inactive'}
                  </dd>
                </div>
              </dl>

              <div className="mt-8 flex flex-wrap items-center gap-4 border-t border-line pt-6">
                {workspace.to !== '/account' ? (
                  <Link
                    to={workspace.to}
                    className="arrow-link type-ui text-mulberry"
                  >
                    <span>{workspace.label}</span>
                    <span className="arrow-ring border-mulberry">
                      <ArrowUpRight size={18} strokeWidth={1.5} aria-hidden="true" />
                    </span>
                  </Link>
                ) : null}
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  disabled={signingOut}
                  onClick={handleLogout}
                >
                  {signingOut ? 'Signing out…' : 'Sign out'}
                </Button>
              </div>
            </section>
          </aside>

          <div className="space-y-8 lg:col-span-7">
            <section
              aria-labelledby="profile-form-heading"
              className="corner-ticks relative rounded-slab border border-line bg-white p-6 md:p-8"
            >
              <span className="tick" aria-hidden="true" />
              <span className="tick" aria-hidden="true" />
              <span className="tick" aria-hidden="true" />
              <span className="tick" aria-hidden="true" />

              <p className="type-label text-ink-muted">02 / PROFILE</p>
              <h2 id="profile-form-heading" className="type-h2 mt-3 text-ink">
                Update personal details
              </h2>

              <form className="mt-6 space-y-5" onSubmit={handleProfileSubmit}>
                <div className="grid gap-5 sm:grid-cols-2">
                  <div className="space-y-2">
                    <label htmlFor="acc-first" className="type-label text-ink-muted">
                      First name
                    </label>
                    <Input
                      id="acc-first"
                      required
                      value={firstName}
                      onChange={(e) => setFirstName(e.target.value)}
                    />
                  </div>
                  <div className="space-y-2">
                    <label htmlFor="acc-last" className="type-label text-ink-muted">
                      Last name
                    </label>
                    <Input
                      id="acc-last"
                      required
                      value={lastName}
                      onChange={(e) => setLastName(e.target.value)}
                    />
                  </div>
                  <div className="space-y-2">
                    <label htmlFor="acc-phone" className="type-label text-ink-muted">
                      Phone number
                    </label>
                    <Input
                      id="acc-phone"
                      type="tel"
                      placeholder="+90 555 000 0000"
                      value={phoneNumber}
                      onChange={(e) => setPhoneNumber(e.target.value)}
                    />
                  </div>
                  <div className="space-y-2">
                    <label htmlFor="acc-grad" className="type-label text-ink-muted">
                      Graduation date
                    </label>
                    <Input
                      id="acc-grad"
                      type="date"
                      value={graduationDate}
                      onChange={(e) => setGraduationDate(e.target.value)}
                    />
                  </div>
                </div>

                {profileError ? (
                  <p role="alert" className="type-ui text-destructive">
                    {profileError}
                  </p>
                ) : null}
                {profileNotice ? (
                  <p role="status" className="type-ui flex items-center gap-2 text-ink">
                    <span className="live-dot" aria-hidden="true" />
                    {profileNotice}
                  </p>
                ) : null}

                <Button type="submit" disabled={savingProfile}>
                  {savingProfile ? 'Saving…' : 'Save profile'}
                </Button>
              </form>
            </section>

            <section
              aria-labelledby="password-form-heading"
              className="rounded-slab border border-line bg-white p-6 md:p-8"
            >
              <p className="type-label text-ink-muted">03 / SECURITY</p>
              <h2 id="password-form-heading" className="type-h2 mt-3 text-ink">
                Change password
              </h2>

              <form className="mt-6 space-y-5" onSubmit={handlePasswordSubmit}>
                <div className="space-y-2">
                  <label htmlFor="acc-current-pw" className="type-label text-ink-muted">
                    Current password
                  </label>
                  <Input
                    id="acc-current-pw"
                    type="password"
                    required
                    value={currentPassword}
                    onChange={(e) => setCurrentPassword(e.target.value)}
                  />
                </div>
                <div className="grid gap-5 sm:grid-cols-2">
                  <div className="space-y-2">
                    <label htmlFor="acc-new-pw" className="type-label text-ink-muted">
                      New password
                    </label>
                    <Input
                      id="acc-new-pw"
                      type="password"
                      required
                      minLength={8}
                      value={newPassword}
                      onChange={(e) => setNewPassword(e.target.value)}
                    />
                  </div>
                  <div className="space-y-2">
                    <label htmlFor="acc-confirm-pw" className="type-label text-ink-muted">
                      Confirm password
                    </label>
                    <Input
                      id="acc-confirm-pw"
                      type="password"
                      required
                      minLength={8}
                      value={confirmPassword}
                      onChange={(e) => setConfirmPassword(e.target.value)}
                    />
                  </div>
                </div>

                {passwordError ? (
                  <p role="alert" className="type-ui text-destructive">
                    {passwordError}
                  </p>
                ) : null}
                {passwordNotice ? (
                  <p role="status" className="type-ui flex items-center gap-2 text-ink">
                    <span className="live-dot" aria-hidden="true" />
                    {passwordNotice}
                  </p>
                ) : null}

                <Button type="submit" variant="outline" disabled={savingPassword}>
                  {savingPassword ? 'Updating…' : 'Update password'}
                </Button>
              </form>
            </section>
          </div>
        </div>
      </div>
    </div>
  )
}
