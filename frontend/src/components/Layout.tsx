import { NavLink, Outlet } from 'react-router-dom'
import { ThemeToggle } from '@/components/ThemeToggle'

function navLinkClass({ isActive }: { isActive: boolean }) {
  return isActive
    ? 'font-semibold text-primary underline underline-offset-4'
    : 'text-muted-foreground hover:text-foreground'
}

export function Layout() {
  return (
    <div className="flex min-h-screen flex-col">
      <header className="border-b">
        <div className="mx-auto flex w-full max-w-4xl items-center justify-between p-4">
          <nav className="flex items-center gap-4">
            <span className="text-lg font-semibold tracking-tight">Alumni</span>
            <NavLink to="/" end className={navLinkClass}>
              Home
            </NavLink>
            <NavLink to="/alumni" className={navLinkClass}>
              Alumni
            </NavLink>
          </nav>
          <ThemeToggle />
        </div>
      </header>
      <main className="mx-auto w-full max-w-4xl flex-1 p-4">
        <Outlet />
      </main>
      <footer className="border-t">
        <div className="mx-auto w-full max-w-4xl p-4 text-sm text-muted-foreground">
          Alumni Tracking System &mdash; Web Development, Fall 2026&ndash;2027
        </div>
      </footer>
    </div>
  )
}
