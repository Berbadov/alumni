import { useState } from 'react'
import { Link, NavLink, Outlet, useNavigate } from 'react-router-dom'
import { useAuth } from '@/auth'
import { Button } from '@/components/ui/button'
import { getNavItems, isVerifiedMember } from '@/navigation'

function navLinkClass({ isActive }: { isActive: boolean }) {
  return [
    'type-ui relative py-2 transition-colors',
    isActive
      ? 'text-ink after:absolute after:inset-x-0 after:bottom-0 after:h-0.5 after:bg-mulberry'
      : 'text-ink-muted hover:text-ink',
  ].join(' ')
}

function Mark() {
  return (
    <span aria-hidden="true" className="grid size-6 grid-cols-2 gap-[2px]">
      <span className="border border-ink" />
      <span className="bg-rose" />
      <span className="border border-ink" />
      <span className="border border-ink" />
    </span>
  )
}

export function Layout() {
  const { user, loading, logout } = useAuth()
  const [signingOut, setSigningOut] = useState(false)
  const navigate = useNavigate()
  const navItems = getNavItems(user, loading)
  const verifiedMember = isVerifiedMember(user)

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
    <div className="flex min-h-screen flex-col bg-white">
      <a
        href="#main"
        className="sr-only focus:not-sr-only focus:absolute focus:top-2 focus:left-2 focus:z-50 focus:rounded-pill focus:bg-ink focus:px-4 focus:py-2 focus:text-white"
      >
        Skip to content
      </a>

      <header className="sticky top-0 z-40 border-b border-line-strong bg-white">
        <div className="mx-auto flex h-16 w-full max-w-[1280px] items-center justify-between gap-3 px-6 md:px-12 sm:gap-6">
          <Link to="/" className="flex items-center gap-3" aria-label="Alumni home">
            <Mark />
            <span className="max-[380px]:hidden font-display text-[1.375rem] leading-none font-medium tracking-[-0.02em]">
              alumni<span className="text-rose">.</span>
            </span>
          </Link>

          <nav
            aria-label="Primary"
            className="flex flex-wrap items-center gap-3 sm:gap-7"
          >
            {navItems.map((item) => (
              <NavLink
                key={`${item.to}:${item.label}`}
                to={item.to}
                end={item.end}
                className={navLinkClass}
              >
                {item.label}
              </NavLink>
            ))}
          </nav>

          <div className="flex items-center gap-3">
            {user ? (
              <>
                <Link
                  to="/account"
                  className="hidden items-center gap-2 lg:flex"
                  title="Account ledger"
                >
                  {verifiedMember && user.verification_status === 'verified' ? (
                    <span className="live-dot" aria-hidden="true" />
                  ) : (
                    <span
                      className="status-dot text-ink-muted"
                      aria-hidden="true"
                    />
                  )}
                  <span className="type-ui text-ink">{user.first_name}</span>
                  <span className="type-label text-ink-muted">
                    {user.roles.join(' · ')}
                  </span>
                </Link>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  disabled={signingOut}
                  onClick={handleLogout}
                >
                  {signingOut ? 'Signing out…' : 'Sign out'}
                </Button>
              </>
            ) : (
              <div className="hidden items-center gap-2 lg:flex">
                <span
                  className="status-dot text-ink-muted"
                  aria-hidden="true"
                />
                <span className="type-label text-ink-muted">
                  Directory 2026
                </span>
              </div>
            )}
          </div>
        </div>
      </header>

      <main id="main" className="flex-1">
        <Outlet />
      </main>

      <footer className="border-t border-line-strong bg-white">
        <div className="mx-auto flex w-full max-w-[1280px] flex-col gap-6 px-6 py-10 md:flex-row md:items-end md:justify-between md:px-12">
          <div className="space-y-2">
            <p className="type-label text-ink-muted">Alumni / Directory</p>
            <p className="max-w-sm text-sm text-ink-muted">
              Web Development, Fall 2026&ndash;2027. Sample records for one
              graduating class.
            </p>
          </div>
          <div className="flex flex-wrap items-center gap-x-4 gap-y-2">
            <span className="type-label text-ink-muted">
              Source / Registrar &middot; 2026-11-18
            </span>
            <span aria-hidden="true" className="hidden h-5 w-px bg-line sm:block" />
            <span className="type-label text-ink-muted">Version 1.1</span>
          </div>
        </div>
      </footer>
    </div>
  )
}
