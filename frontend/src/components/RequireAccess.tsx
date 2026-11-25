import type { ReactNode } from 'react'
import { Link } from 'react-router-dom'
import { useAuth } from '@/auth'
import { canAccess, primaryWorkspaceFor, type RouteAccess } from '@/navigation'
import { Button } from '@/components/ui/button'
import type { AcademicStatus } from '@/types'

interface RequireAccessProps {
  access: Extract<RouteAccess, 'authenticated' | 'unverified' | 'verified' | 'admin'>
  roles?: readonly AcademicStatus[]
  sectionLabel: string
  heading: string
  children: ReactNode
}

export function RequireAccess({
  access,
  roles,
  sectionLabel,
  heading,
  children,
}: RequireAccessProps) {
  const { user, loading } = useAuth()

  if (loading) {
    return (
      <div className="mx-auto w-full max-w-[1280px] px-6 py-14 md:px-12">
        <p className="type-label border-b border-line pb-6 text-ink-muted">
          {sectionLabel}
        </p>
        <div className="mt-8 rounded-plate border border-line bg-white px-5 py-4">
          <p className="type-ui text-ink-muted">Checking session…</p>
        </div>
      </div>
    )
  }

  if (canAccess(access, user, roles)) {
    return <>{children}</>
  }

  const signedIn = Boolean(user)
  const roleLabel = user ? user.roles.join(', ') : 'guest'
  const fallback = primaryWorkspaceFor(user)

  return (
    <div className="mx-auto w-full max-w-[1280px] px-6 py-14 md:px-12">
      <p className="type-label border-b border-line pb-6 text-ink-muted">
        {sectionLabel}
      </p>

      <div className="mt-8 rounded-plate border border-line-strong bg-white p-6 md:p-8">
        {!signedIn ? (
          <>
            <p className="type-label text-ink-muted">401 / SIGN IN REQUIRED</p>
            <h1 className="type-h2 mt-3 text-ink">{heading}</h1>
            <p className="type-ui mt-2 max-w-[54ch] text-ink-muted">
              Sign in to view this page, or register a new account to begin
              verification.
            </p>
            <div className="mt-6 flex flex-wrap gap-3">
              <Button asChild>
                <Link to="/login">Sign in</Link>
              </Button>
              <Button asChild variant="outline">
                <Link to="/register">Register</Link>
              </Button>
            </div>
          </>
        ) : access === 'unverified' ? (
          <>
            <p className="type-label text-ink-muted">ALREADY VERIFIED</p>
            <h1 className="type-h2 mt-3 text-ink">
              Your account is already verified.
            </h1>
            <p className="type-ui mt-2 max-w-[54ch] text-ink-muted">
              Signed in as {user?.email} ({roleLabel}). Open your workspace to
              continue.
            </p>
            <div className="mt-6 flex flex-wrap gap-3">
              <Button asChild>
                <Link to={fallback.to}>{fallback.label}</Link>
              </Button>
              <Button asChild variant="outline">
                <Link to="/account">Account</Link>
              </Button>
            </div>
          </>
        ) : access === 'admin' ? (
          <>
            <p className="type-label text-destructive">403 / RESTRICTED</p>
            <h1 className="type-h2 mt-3 text-ink">
              Administrator role required.
            </h1>
            <p className="type-ui mt-2 max-w-[54ch] text-ink-muted">
              Signed in as {user?.email} ({roleLabel}). This page is available
              to admin accounts only.
            </p>
            <div className="mt-6 flex flex-wrap gap-3">
              <Button asChild variant="outline">
                <Link to={fallback.to}>{fallback.label}</Link>
              </Button>
            </div>
          </>
        ) : roles && roles.length > 0 && user && user.roles.some((r) => r !== 'unverified') ? (
          <>
            <p className="type-label text-destructive">403 / ROLE SPECIFIC PAGE</p>
            <h1 className="type-h2 mt-3 text-ink">
              This workspace is restricted by role.
            </h1>
            <p className="type-ui mt-2 max-w-[54ch] text-ink-muted">
              Signed in as {user.email} ({roleLabel}). Required role:{' '}
              {roles.join(', ')}.
            </p>
            <div className="mt-6 flex flex-wrap gap-3">
              <Button asChild>
                <Link to={fallback.to}>{fallback.label}</Link>
              </Button>
              <Button asChild variant="outline">
                <Link to="/account">Account</Link>
              </Button>
            </div>
          </>
        ) : (
          <>
            <p className="type-label text-destructive">
              403 / VERIFICATION REQUIRED
            </p>
            <h1 className="type-h2 mt-3 text-ink">
              {user?.verification_status === 'rejected'
                ? 'Verification rejected.'
                : 'Account verification pending.'}
            </h1>
            <p className="type-ui mt-2 max-w-[54ch] text-ink-muted">
              Signed in as {user?.email} ({roleLabel}). Access opens once an
              administrator approves government papers and assigns an active role.
            </p>
            <div className="mt-6 flex flex-wrap gap-3">
              <Button asChild>
                <Link to="/verification">View verification status</Link>
              </Button>
              <Button asChild variant="outline">
                <Link to="/account">Account</Link>
              </Button>
            </div>
          </>
        )}
      </div>
    </div>
  )
}
