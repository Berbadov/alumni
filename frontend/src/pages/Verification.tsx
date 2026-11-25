import { Link } from 'react-router-dom'
import { ArrowUpRight } from 'lucide-react'
import { useAuth } from '@/auth'
import { Button } from '@/components/ui/button'

export function Verification() {
  const { user } = useAuth()
  if (!user) return null

  const rejected = user.verification_status === 'rejected'

  return (
    <div>
      <section
        aria-labelledby="verification-heading"
        className="relative overflow-hidden bg-white"
      >
        <div
          aria-hidden="true"
          className="grid-live pointer-events-none absolute inset-0 opacity-[0.06]"
        />
        <div className="relative mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <p className="type-label border-b border-line py-6 text-ink-muted">
            08 / VERIFICATION
          </p>

          <div className="grid gap-10 py-12 md:grid-cols-12 md:py-16">
            <div className="reveal md:col-span-7">
              <h1 id="verification-heading" className="type-h1 text-ink">
                {rejected ? (
                  <>
                    Papers{' '}
                    <em className="font-normal italic text-mulberry">
                      declined
                    </em>
                    .
                  </>
                ) : (
                  <>
                    Awaiting{' '}
                    <em className="font-normal italic text-mulberry">papers</em>
                    .
                  </>
                )}
              </h1>
              <p className="type-lede mt-6 max-w-[54ch] text-ink-muted">
                Every newly registered account stays on the{' '}
                <strong className="font-medium text-ink">unverified</strong>{' '}
                role until an administrator checks government papers.
              </p>
            </div>
            <div className="flex flex-col items-start justify-end gap-2 md:col-span-5 md:items-end">
              <div className="inline-flex items-center gap-2">
                <span
                  className={`status-dot ${
                    rejected ? 'text-destructive' : 'text-ink-muted'
                  }`}
                  aria-hidden="true"
                />
                <span className="type-label text-ink">
                  {rejected ? 'REJECTED' : 'PENDING REVIEW'}
                </span>
              </div>
              <p className="type-ui text-ink-muted">
                Requested role: {user.requested_role ?? 'not recorded'}
              </p>
            </div>
          </div>
        </div>
      </section>

      <div className="mx-auto w-full max-w-[1280px] px-6 py-10 md:px-12 md:py-16">
        <div className="grid gap-8 lg:grid-cols-12">
          <section className="corner-ticks relative rounded-slab border border-line-strong bg-white p-6 md:p-8 lg:col-span-7">
            <span className="tick" aria-hidden="true" />
            <span className="tick" aria-hidden="true" />
            <span className="tick" aria-hidden="true" />
            <span className="tick" aria-hidden="true" />

            <p className="type-label text-ink-muted">01 / STATUS CHECKLIST</p>
            <h2 className="type-h2 mt-3 text-ink">
              What happens before access unlocks
            </h2>

            <dl className="mt-6 space-y-5 border-t border-line pt-6">
              <div className="flex flex-wrap items-baseline justify-between gap-2 border-b border-line pb-4">
                <dt className="type-ui font-medium text-ink">
                  Government papers review
                </dt>
                <dd className="type-label text-ink-muted">
                  {user.verification_status} &middot; admin approval required
                </dd>
              </div>
              <div className="flex flex-wrap items-baseline justify-between gap-2 border-b border-line pb-4">
                <dt className="type-ui font-medium text-ink">
                  Requested role
                </dt>
                <dd className="type-data text-ink">
                  {user.requested_role ?? 'Not selected'}
                  {user.graduation_date ? ` (${user.graduation_date})` : ''}
                </dd>
              </div>
              <div className="flex flex-wrap items-baseline justify-between gap-2 border-b border-line pb-4">
                <dt className="type-ui font-medium text-ink">
                  E-mail verification
                </dt>
                <dd className="type-label text-ink-muted">
                  {user.email_verified
                    ? 'verified'
                    : 'ready in code · inactive'}
                </dd>
              </div>
              <div className="flex flex-wrap items-baseline justify-between gap-2">
                <dt className="type-ui font-medium text-ink">
                  SMS verification
                </dt>
                <dd className="type-label text-ink-muted">
                  {user.phone_verified
                    ? 'verified'
                    : 'ready in code · inactive'}
                </dd>
              </div>
            </dl>

            <div className="mt-8 flex flex-wrap gap-4">
              <Button asChild>
                <Link to="/account">Review account details</Link>
              </Button>
              <Link
                to="/about"
                className="arrow-link type-ui self-center text-ink hover:text-mulberry"
              >
                <span>Read the archive</span>
                <span className="arrow-ring">
                  <ArrowUpRight size={18} strokeWidth={1.5} aria-hidden="true" />
                </span>
              </Link>
            </div>
          </section>

          <aside className="rounded-plate border border-line bg-white p-6 md:p-8 lg:col-span-5">
            <p className="type-label text-ink-muted">02 / AFTER VERIFICATION</p>
            <h2 className="type-h3 mt-3 text-ink">Role-specific pages</h2>
            <ul className="mt-6 space-y-4 type-ui text-ink-muted">
              <li className="border-b border-line pb-3">
                <strong className="font-medium text-ink">
                  Undergrad / Grad / PhD:
                </strong>{' '}
                unlocks the Student portal, Alumni directory, and active Job
                postings.
              </li>
              <li className="border-b border-line pb-3">
                <strong className="font-medium text-ink">Alumni:</strong>{' '}
                unlocks the Graduate desk where you can publish and manage your
                own job postings.
              </li>
              <li>
                <strong className="font-medium text-ink">
                  Academic / Professor:
                </strong>{' '}
                unlocks the Faculty announcements console and job publishing.
              </li>
            </ul>
          </aside>
        </div>
      </div>
    </div>
  )
}
