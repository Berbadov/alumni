import { useEffect, useState, type FormEvent } from 'react'
import { Link } from 'react-router-dom'
import {
  fetchAdminUsers,
  revokeAdminUserSessions,
  updateAdminUser,
  verifyAdminUser,
} from '@/api'
import { useAuth } from '@/auth'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import type { AcademicStatus, AssignableRole, User } from '@/types'

type Filter = 'all' | 'pending' | 'verified' | 'rejected' | 'disabled'
type PanelMode = 'verify' | 'edit'

const FILTERS: { value: Filter; label: string }[] = [
  { value: 'all', label: 'All' },
  { value: 'pending', label: 'Pending' },
  { value: 'verified', label: 'Verified' },
  { value: 'rejected', label: 'Rejected' },
  { value: 'disabled', label: 'Disabled' },
]

const ASSIGNABLE_ROLES: { value: AssignableRole; label: string }[] = [
  { value: 'undergrad', label: 'Undergrad' },
  { value: 'grad', label: 'Grad' },
  { value: 'phd', label: 'PhD' },
  { value: 'alumni', label: 'Alumni' },
  { value: 'academic', label: 'Academic' },
  { value: 'professor', label: 'Professor' },
  { value: 'admin', label: 'Admin' },
]

const numberFormat = new Intl.NumberFormat('en-US')

function message(error: unknown): string {
  return error instanceof Error ? error.message : String(error)
}

function initialRolesFor(target: User, mode: PanelMode): AssignableRole[] {
  if (mode === 'verify') {
    if (target.requested_role && target.requested_role !== 'unverified') {
      return [target.requested_role]
    }
    const existing = target.roles.filter(
      (role): role is AssignableRole => role !== 'unverified',
    )
    return existing.length > 0 ? existing : ['undergrad']
  }
  const existing = target.roles.filter(
    (role): role is AssignableRole => role !== 'unverified',
  )
  return existing.length > 0 ? existing : ['undergrad']
}

function formatRole(role: AcademicStatus): string {
  switch (role) {
    case 'unverified':
      return 'unverified'
    case 'undergrad':
      return 'undergrad'
    case 'grad':
      return 'grad'
    case 'phd':
      return 'phd'
    case 'alumni':
      return 'alumni'
    case 'academic':
      return 'academic'
    case 'professor':
      return 'professor'
    case 'admin':
      return 'admin'
  }
}

export function Admin() {
  const { user, loading: authLoading } = useAuth()
  const [users, setUsers] = useState<User[]>([])
  const [loading, setLoading] = useState(false)
  const [filter, setFilter] = useState<Filter>('all')
  const [selected, setSelected] = useState<User | null>(null)
  const [mode, setMode] = useState<PanelMode>('verify')
  const [selectedRoles, setSelectedRoles] = useState<AssignableRole[]>([
    'undergrad',
  ])
  const [verificationMethod, setVerificationMethod] =
    useState('government_papers')
  const [activeDraft, setActiveDraft] = useState(true)
  const [saving, setSaving] = useState(false)
  const [busyId, setBusyId] = useState<number | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [notice, setNotice] = useState<string | null>(null)
  const [reloadKey, setReloadKey] = useState(0)

  const isAdmin = Boolean(user?.roles.includes('admin'))

  useEffect(() => {
    if (!isAdmin) {
      setUsers([])
      return
    }
    let cancelled = false
    setLoading(true)
    setError(null)
    fetchAdminUsers()
      .then((rows) => {
        if (!cancelled) setUsers(rows)
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
  }, [isAdmin, reloadKey])

  function openPanel(target: User, nextMode: PanelMode) {
    setSelected(target)
    setMode(nextMode)
    setSelectedRoles(initialRolesFor(target, nextMode))
    setVerificationMethod(target.verification_method ?? 'government_papers')
    setActiveDraft(target.is_active)
    setError(null)
    setNotice(null)
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }

  function closePanel() {
    setSelected(null)
  }

  function toggleRole(role: AssignableRole) {
    setSelectedRoles((current) =>
      current.includes(role)
        ? current.filter((item) => item !== role)
        : [...current, role],
    )
  }

  async function handleSave(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    if (!selected) return
    if (selectedRoles.length === 0) {
      setError('Select at least one role.')
      return
    }

    setSaving(true)
    setError(null)
    try {
      const updated =
        mode === 'verify'
          ? await verifyAdminUser(selected._id, {
              verification_status: 'verified',
              roles: selectedRoles,
              verification_method:
                verificationMethod.trim() || 'government_papers',
            })
          : await updateAdminUser(selected._id, {
              roles: selectedRoles,
              is_active: activeDraft,
            })
      closePanel()
      setNotice(
        mode === 'verify'
          ? `Verified ${updated.full_name} as ${updated.roles.join(', ')}.`
          : `Updated ${updated.full_name}.`,
      )
      setReloadKey((key) => key + 1)
    } catch (e: unknown) {
      setError(message(e))
    } finally {
      setSaving(false)
    }
  }

  async function handleReject(target: User) {
    setBusyId(target._id)
    setError(null)
    setNotice(null)
    try {
      const updated = await verifyAdminUser(target._id, {
        verification_status: 'rejected',
      })
      if (selected?._id === target._id) {
        closePanel()
      }
      setNotice(`Marked ${updated.full_name} as rejected.`)
      setReloadKey((key) => key + 1)
    } catch (e: unknown) {
      setError(message(e))
    } finally {
      setBusyId(null)
    }
  }

  async function handleToggleActive(target: User) {
    setBusyId(target._id)
    setError(null)
    setNotice(null)
    try {
      const updated = await updateAdminUser(target._id, {
        is_active: !target.is_active,
      })
      setNotice(
        updated.is_active
          ? `Enabled ${updated.full_name}.`
          : `Disabled ${updated.full_name} and revoked sessions.`,
      )
      setReloadKey((key) => key + 1)
    } catch (e: unknown) {
      setError(message(e))
    } finally {
      setBusyId(null)
    }
  }

  async function handleRevokeSessions(target: User) {
    setBusyId(target._id)
    setError(null)
    setNotice(null)
    try {
      await revokeAdminUserSessions(target._id)
      setNotice(`Revoked active sessions for ${target.full_name}.`)
    } catch (e: unknown) {
      setError(message(e))
    } finally {
      setBusyId(null)
    }
  }

  const pendingCount = users.filter(
    (item) => item.verification_status === 'pending',
  ).length
  const activeCount = users.filter((item) => item.is_active).length

  const filteredUsers = users.filter((item) => {
    switch (filter) {
      case 'all':
        return true
      case 'pending':
        return item.verification_status === 'pending'
      case 'verified':
        return item.verification_status === 'verified'
      case 'rejected':
        return item.verification_status === 'rejected'
      case 'disabled':
        return !item.is_active
    }
  })

  return (
    <div>
      {/* 06 — Paper header. The drifting grid is this view's one ambient loop. */}
      <section
        aria-labelledby="admin-heading"
        className="relative overflow-hidden bg-white"
      >
        <div
          aria-hidden="true"
          className="grid-live pointer-events-none absolute inset-0 opacity-[0.06]"
        />
        <div className="relative mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <p className="type-label border-b border-line py-6 text-ink-muted">
            06 / ADMIN
          </p>

          <div className="grid gap-10 py-12 md:grid-cols-12 md:py-16">
            <div className="reveal md:col-span-7">
              <h1 id="admin-heading" className="type-h1 text-ink">
                Control{' '}
                <em className="font-normal italic text-mulberry">ledger</em>.
              </h1>
              <p className="type-lede mt-6 max-w-[54ch] text-ink-muted">
                Inspect government-paper verifications, promote unverified
                accounts, manage roles, and revoke active sessions.
              </p>
            </div>
            <div className="flex flex-col items-start justify-end gap-2 md:col-span-5 md:items-end">
              <p className="type-label text-ink-muted">
                ADMINISTRATION / USERS &amp; PAPERS
              </p>
              <p className="type-ui text-ink-muted">
                Role changes revoke active user sessions automatically.
              </p>
            </div>
          </div>
        </div>
      </section>

      <div className="mx-auto w-full max-w-[1280px] space-y-8 px-6 py-10 md:px-12 md:py-14">
        {authLoading ? (
          <div className="rounded-plate border border-line bg-white px-5 py-4">
            <p className="type-ui text-ink-muted">Checking session…</p>
          </div>
        ) : !user ? (
          <div className="rounded-plate border border-line-strong bg-white p-6 md:p-8">
            <p className="type-label text-ink-muted">401 / SIGN IN REQUIRED</p>
            <h2 className="type-h2 mt-3 text-ink">
              Sign in as an administrator.
            </h2>
            <p className="type-ui mt-2 max-w-[54ch] text-ink-muted">
              An active session with the admin role is required to open the
              control ledger.
            </p>
            <div className="mt-6">
              <Button asChild>
                <Link to="/login">Sign in</Link>
              </Button>
            </div>
          </div>
        ) : !isAdmin ? (
          <div
            role="alert"
            className="rounded-plate border border-destructive/40 bg-white p-6 md:p-8"
          >
            <p className="type-label text-destructive">403 / RESTRICTED</p>
            <h2 className="type-h2 mt-3 text-ink">
              Administrator role required.
            </h2>
            <p className="type-ui mt-2 max-w-[54ch] text-ink-muted">
              Signed in as {user.email} ({user.roles.join(', ')}). Only admin
              accounts can verify papers and manage roles.
            </p>
            <div className="mt-6">
              <Button asChild variant="outline">
                <Link to="/">Return home</Link>
              </Button>
            </div>
          </div>
        ) : (
          <>
            <section
              aria-label="User counts"
              className="border-y border-line py-8 md:py-10"
            >
              <dl className="grid grid-cols-1 divide-y divide-line sm:grid-cols-3 sm:divide-x sm:divide-y-0">
                <div className="py-4 sm:pr-8 sm:py-0">
                  <dt className="type-label text-ink-muted">Total accounts</dt>
                  <dd className="type-numeral mt-3 text-[3rem] leading-none text-ink">
                    {loading ? '—' : numberFormat.format(users.length)}
                  </dd>
                </div>
                <div className="py-4 sm:px-8 sm:py-0">
                  <dt className="type-label text-ink-muted">
                    Pending verification
                  </dt>
                  <dd className="type-numeral mt-3 text-[3rem] leading-none text-ink">
                    {loading ? '—' : numberFormat.format(pendingCount)}
                  </dd>
                </div>
                <div className="py-4 sm:pl-8 sm:py-0">
                  <dt className="type-label text-ink-muted">Active accounts</dt>
                  <dd className="type-numeral mt-3 text-[3rem] leading-none text-ink">
                    {loading ? '—' : numberFormat.format(activeCount)}
                  </dd>
                </div>
              </dl>
              <p className="type-label mt-6 text-ink-muted">
                SOURCE / USER STORE &middot; 2026-11-25
              </p>
            </section>

            {error ? (
              <div
                role="alert"
                className="flex flex-wrap items-baseline gap-x-3 gap-y-1 border border-destructive/40 bg-white px-4 py-3"
              >
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

            {selected ? (
              <section
                aria-labelledby="admin-editor-heading"
                className="corner-ticks relative rounded-slab border border-line-strong bg-white p-6 md:p-8"
              >
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />

                <p className="type-label text-ink-muted">
                  {mode === 'verify'
                    ? '07 / PAPER VERIFICATION'
                    : '07 / ROLE MANAGEMENT'}
                </p>
                <h2
                  id="admin-editor-heading"
                  className="type-h2 mt-4 text-ink"
                >
                  {mode === 'verify'
                    ? `Verify ${selected.full_name} (#${selected._id})`
                    : `Manage ${selected.full_name} (#${selected._id})`}
                </h2>
                <p className="type-ui mt-2 text-ink-muted">
                  {selected.email} &middot; Age {selected.age}
                  {selected.requested_role
                    ? ` · Requested ${selected.requested_role}`
                    : ''}
                  {selected.graduation_date
                    ? ` · Graduated ${selected.graduation_date}`
                    : ''}
                </p>

                <form className="mt-8 space-y-6" onSubmit={handleSave}>
                  {mode === 'verify' ? (
                    <div className="max-w-md space-y-2">
                      <label
                        htmlFor="admin-verify-method"
                        className="type-label text-ink-muted"
                      >
                        Verification method
                      </label>
                      <Input
                        id="admin-verify-method"
                        value={verificationMethod}
                        onChange={(e) => setVerificationMethod(e.target.value)}
                        placeholder="government_papers"
                      />
                    </div>
                  ) : null}

                  <div className="space-y-3">
                    <span className="type-label block text-ink-muted">
                      Assigned roles
                    </span>
                    <div className="grid gap-3 sm:grid-cols-4">
                      {ASSIGNABLE_ROLES.map((item) => {
                        const checked = selectedRoles.includes(item.value)
                        const selfAdminLock =
                          mode === 'edit' &&
                          selected._id === user._id &&
                          item.value === 'admin'
                        return (
                          <label
                            key={item.value}
                            className="type-ui flex min-h-11 cursor-pointer items-center gap-3 rounded-field border border-line-strong bg-white px-4 py-2 text-ink"
                          >
                            <input
                              type="checkbox"
                              className="size-4 accent-mulberry"
                              checked={checked}
                              disabled={selfAdminLock}
                              onChange={() => toggleRole(item.value)}
                            />
                            <span>{item.label}</span>
                          </label>
                        )
                      })}
                    </div>
                  </div>

                  {mode === 'edit' ? (
                    <label className="type-ui flex items-center gap-3 text-ink">
                      <input
                        type="checkbox"
                        className="size-4 accent-mulberry"
                        checked={activeDraft}
                        disabled={selected._id === user._id}
                        onChange={(e) => setActiveDraft(e.target.checked)}
                      />
                      Active &mdash; allow this account to hold sessions
                    </label>
                  ) : null}

                  <div className="flex flex-wrap items-center gap-3">
                    <Button type="submit" disabled={saving}>
                      {saving
                        ? 'Saving…'
                        : mode === 'verify'
                          ? 'Approve & verify'
                          : 'Save changes'}
                    </Button>
                    {mode === 'verify' ? (
                      <Button
                        type="button"
                        variant="outline"
                        className="border-destructive text-destructive hover:border-destructive hover:bg-destructive hover:text-white"
                        disabled={saving || busyId === selected._id}
                        onClick={() => handleReject(selected)}
                      >
                        Reject papers
                      </Button>
                    ) : (
                      <Button
                        type="button"
                        variant="outline"
                        disabled={saving || busyId === selected._id}
                        onClick={() => handleRevokeSessions(selected)}
                      >
                        Revoke sessions
                      </Button>
                    )}
                    <Button
                      type="button"
                      variant="outline"
                      disabled={saving}
                      onClick={closePanel}
                    >
                      Cancel
                    </Button>
                  </div>
                </form>
              </section>
            ) : null}

            <div className="flex flex-wrap items-end justify-between gap-4">
              <div
                className="flex flex-wrap gap-2"
                role="group"
                aria-label="Filter users"
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
                {filteredUsers.length}{' '}
                {filteredUsers.length === 1 ? 'account' : 'accounts'}
              </p>
            </div>

            {loading ? (
              <div className="rounded-plate border border-line bg-white px-5 py-4">
                <p className="type-ui text-ink-muted">Loading users…</p>
              </div>
            ) : filteredUsers.length === 0 ? (
              <div className="rounded-plate border border-dotted border-line-strong px-6 py-16 text-center">
                <p className="type-label text-ink-muted">NO ACCOUNTS</p>
                <p className="type-ui mx-auto mt-4 max-w-[46ch] text-ink-muted">
                  No accounts match this filter.
                </p>
              </div>
            ) : (
              <section
                aria-label="Users table"
                className="corner-ticks relative rounded-plate border border-line bg-white"
              >
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />
                <span className="tick" aria-hidden="true" />

                <div className="flex flex-wrap items-center justify-between gap-3 border-b border-line px-5 py-4">
                  <p className="type-label text-ink-muted">08 / ACCOUNTS</p>
                  <p className="type-label text-ink-muted">
                    SOURCE / USERS COLLECTION &middot; 2026-11-25
                  </p>
                </div>

                <Table className="table-brand">
                  <TableCaption className="sr-only">User accounts</TableCaption>
                  <TableHeader>
                    <TableRow>
                      <TableHead scope="col">User</TableHead>
                      <TableHead scope="col">Roles</TableHead>
                      <TableHead scope="col">Requested</TableHead>
                      <TableHead scope="col">Verification</TableHead>
                      <TableHead scope="col">Account</TableHead>
                      <TableHead scope="col" className="text-right">
                        Actions
                      </TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {filteredUsers.map((row) => {
                      const isSelf = row._id === user._id
                      return (
                        <TableRow key={row._id}>
                          <TableCell className="whitespace-normal">
                            <span className="type-ui font-medium text-ink">
                              {row.full_name}
                            </span>
                            <span className="type-ui mt-1 block text-ink-muted">
                              #{row._id} &middot; {row.email} &middot; age{' '}
                              {row.age}
                            </span>
                          </TableCell>
                          <TableCell className="type-data text-ink">
                            {row.roles.map(formatRole).join(', ')}
                          </TableCell>
                          <TableCell className="type-data text-ink">
                            {row.requested_role
                              ? formatRole(row.requested_role)
                              : 'Not recorded.'}
                            {row.graduation_date ? (
                              <span className="type-ui mt-1 block text-ink-muted">
                                {row.graduation_date}
                              </span>
                            ) : null}
                          </TableCell>
                          <TableCell>
                            <span className="type-ui inline-flex items-center gap-2 lowercase text-ink">
                              {row.verification_status === 'verified' ? (
                                <span className="live-dot" aria-hidden="true" />
                              ) : (
                                <span
                                  className="status-dot text-ink-muted"
                                  aria-hidden="true"
                                />
                              )}
                              {row.verification_status}
                            </span>
                            {row.verification_method ? (
                              <span className="type-ui mt-1 block text-ink-muted">
                                {row.verification_method}
                              </span>
                            ) : null}
                          </TableCell>
                          <TableCell>
                            <span className="type-ui inline-flex items-center gap-2 lowercase text-ink">
                              {row.is_active ? (
                                <span className="live-dot" aria-hidden="true" />
                              ) : (
                                <span
                                  className="status-dot text-ink-muted"
                                  aria-hidden="true"
                                />
                              )}
                              {row.is_active ? 'active' : 'disabled'}
                            </span>
                          </TableCell>
                          <TableCell className="text-right">
                            <div className="flex flex-wrap justify-end gap-2">
                              {row.verification_status !== 'verified' ? (
                                <Button
                                  variant="outline"
                                  size="sm"
                                  onClick={() => openPanel(row, 'verify')}
                                >
                                  Verify papers
                                </Button>
                              ) : null}
                              <Button
                                variant="outline"
                                size="sm"
                                onClick={() => openPanel(row, 'edit')}
                              >
                                Manage
                              </Button>
                              <Button
                                variant="outline"
                                size="sm"
                                disabled={isSelf || busyId === row._id}
                                onClick={() => handleToggleActive(row)}
                              >
                                {row.is_active ? 'Disable' : 'Enable'}
                              </Button>
                            </div>
                          </TableCell>
                        </TableRow>
                      )
                    })}
                  </TableBody>
                </Table>
              </section>
            )}
          </>
        )}
      </div>
    </div>
  )
}
