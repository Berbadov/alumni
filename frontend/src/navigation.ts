import type { AcademicStatus, User } from '@/types'

export type RouteAccess =
  | 'public'
  | 'guest'
  | 'authenticated'
  | 'unverified'
  | 'verified'
  | 'admin'

export interface NavItem {
  to: string
  label: string
  end?: boolean
  access: RouteAccess
  roles?: readonly AcademicStatus[]
}

const STUDENT_ROLES: readonly AcademicStatus[] = ['undergrad', 'grad', 'phd']
const FACULTY_ROLES: readonly AcademicStatus[] = ['academic', 'professor', 'admin']
const ANNOUNCEMENT_AUTHOR_ROLES: readonly AcademicStatus[] = ['academic', 'professor']
const JOB_AUTHOR_ROLES: readonly AcademicStatus[] = [
  'alumni',
  'academic',
  'professor',
]

export const NAV_ITEMS: readonly NavItem[] = [
  { to: '/', label: 'Home', end: true, access: 'public' },
  { to: '/verification', label: 'Verification', access: 'unverified' },
  { to: '/student', label: 'Student', access: 'verified', roles: STUDENT_ROLES },
  { to: '/graduate', label: 'Graduate', access: 'verified', roles: ['alumni'] },
  { to: '/faculty', label: 'Faculty', access: 'verified', roles: FACULTY_ROLES },
  { to: '/alumni', label: 'Alumni', access: 'verified' },
  { to: '/jobs', label: 'Jobs', access: 'verified' },
  { to: '/admin', label: 'Admin', access: 'admin', roles: ['admin'] },
  { to: '/account', label: 'Account', access: 'authenticated' },
  { to: '/about', label: 'About', access: 'public' },
  { to: '/register', label: 'Register', access: 'guest' },
  { to: '/login', label: 'Sign in', access: 'guest' },
]

export function hasRole(user: User | null, role: AcademicStatus): boolean {
  return Boolean(user && user.is_active && user.roles.includes(role))
}

export function hasAnyRole(
  user: User | null,
  roles: readonly AcademicStatus[],
): boolean {
  return Boolean(
    user && user.is_active && roles.some((role) => user.roles.includes(role)),
  )
}

export function isVerifiedMember(user: User | null): boolean {
  return Boolean(
    user &&
      user.is_active &&
      user.roles.some((role) => role !== 'unverified'),
  )
}

export function isAdmin(user: User | null): boolean {
  return hasRole(user, 'admin')
}

export function canPostJobs(user: User | null): boolean {
  return isAdmin(user) || hasAnyRole(user, JOB_AUTHOR_ROLES)
}

export function canModifyJobPosting(
  user: User | null,
  authorId: number,
): boolean {
  return (
    isAdmin(user) ||
    (hasAnyRole(user, JOB_AUTHOR_ROLES) && user?._id === authorId)
  )
}

export function canManageAnnouncements(user: User | null): boolean {
  return isAdmin(user) || hasAnyRole(user, ANNOUNCEMENT_AUTHOR_ROLES)
}

export function canModifyAnnouncement(
  user: User | null,
  authorId: number,
): boolean {
  return (
    isAdmin(user) ||
    (hasAnyRole(user, ANNOUNCEMENT_AUTHOR_ROLES) && user?._id === authorId)
  )
}

export function primaryWorkspaceFor(user: User | null): {
  to: string
  label: string
} {
  if (!user) {
    return { to: '/login', label: 'Sign in' }
  }
  if (!isVerifiedMember(user)) {
    return { to: '/verification', label: 'Verification status' }
  }
  if (isAdmin(user)) {
    return { to: '/admin', label: 'Admin control ledger' }
  }
  if (hasAnyRole(user, ANNOUNCEMENT_AUTHOR_ROLES)) {
    return { to: '/faculty', label: 'Faculty announcements' }
  }
  if (hasRole(user, 'alumni')) {
    return { to: '/graduate', label: 'Graduate workspace' }
  }
  if (hasAnyRole(user, STUDENT_ROLES)) {
    return { to: '/student', label: 'Student workspace' }
  }
  return { to: '/account', label: 'Account settings' }
}

export function canAccess(
  access: RouteAccess,
  user: User | null,
  roles?: readonly AcademicStatus[],
): boolean {
  if (roles && roles.length > 0 && !hasAnyRole(user, roles)) {
    return false
  }

  switch (access) {
    case 'public':
      return true
    case 'guest':
      return user === null
    case 'authenticated':
      return user !== null
    case 'unverified':
      return user !== null && !isVerifiedMember(user)
    case 'verified':
      return isVerifiedMember(user)
    case 'admin':
      return isAdmin(user)
  }
}

export function getNavItems(user: User | null, loading: boolean): NavItem[] {
  return NAV_ITEMS.filter((item) => {
    if (loading && item.access !== 'public') {
      return false
    }
    return canAccess(item.access, user, item.roles)
  })
}
