import type {
  AdminUpdateUserInput,
  Alumni,
  Announcement,
  AnnouncementInput,
  AuthSession,
  JobPosting,
  JobPostingInput,
  LoginInput,
  RegisterInput,
  UpdateOwnProfileInput,
  User,
  VerifyUserInput,
} from './types'

async function errorMessage(res: Response): Promise<string> {
  try {
    const body: unknown = await res.json()
    if (
      typeof body === 'object' &&
      body !== null &&
      'error' in body &&
      typeof body.error === 'string'
    ) {
      return body.error
    }
  } catch {
    // Fall through to the status text when the body is not JSON.
  }
  return `Request failed (${res.status})`
}

async function request<T>(url: string, init?: RequestInit): Promise<T> {
  const res = await fetch(url, { credentials: 'same-origin', ...init })
  if (!res.ok) {
    throw new Error(await errorMessage(res))
  }
  if (res.status === 204) {
    return undefined as T
  }
  return res.json() as Promise<T>
}

const json = (method: string, body: unknown): RequestInit => ({
  method,
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify(body),
})

export function registerUser(input: RegisterInput): Promise<User> {
  return request('/api/v1/auth/register', json('POST', input))
}

export function loginUser(input: LoginInput): Promise<AuthSession> {
  return request('/api/v1/auth/login', json('POST', input))
}

export function logoutUser(): Promise<void> {
  return request('/api/v1/auth/logout', { method: 'POST' })
}

export async function fetchCurrentUser(): Promise<User | null> {
  const res = await fetch('/api/v1/auth/me', { credentials: 'same-origin' })
  if (res.status === 401) {
    return null
  }
  if (!res.ok) {
    throw new Error(await errorMessage(res))
  }
  return res.json() as Promise<User>
}

export function updateCurrentUser(input: UpdateOwnProfileInput): Promise<User> {
  return request('/api/v1/auth/me', json('PATCH', input))
}

export function fetchAnnouncements(): Promise<Announcement[]> {
  return request('/api/v1/announcements')
}

export function createAnnouncement(
  input: AnnouncementInput,
): Promise<Announcement> {
  return request('/api/v1/announcements', json('POST', input))
}

export function replaceAnnouncement(
  id: number,
  input: AnnouncementInput,
): Promise<Announcement> {
  return request(`/api/v1/announcements/${id}`, json('PUT', input))
}

export function deleteAnnouncement(id: number): Promise<void> {
  return request(`/api/v1/announcements/${id}`, { method: 'DELETE' })
}

export function fetchAdminUsers(): Promise<User[]> {
  return request('/api/v1/admin/users')
}

export function verifyAdminUser(
  id: number,
  input: VerifyUserInput,
): Promise<User> {
  return request(`/api/v1/admin/users/${id}/verify`, json('POST', input))
}

export function updateAdminUser(
  id: number,
  input: AdminUpdateUserInput,
): Promise<User> {
  return request(`/api/v1/admin/users/${id}`, json('PATCH', input))
}

export function revokeAdminUserSessions(id: number): Promise<void> {
  return request(`/api/v1/admin/users/${id}/revoke-sessions`, {
    method: 'POST',
  })
}

export function fetchAlumni(): Promise<Alumni[]> {
  return request('/api/v1/alumni')
}

export function fetchJobPostings(active?: boolean): Promise<JobPosting[]> {
  const query = active === undefined ? '' : `?active=${active}`
  return request(`/api/v1/job-postings${query}`)
}

export function createJobPosting(input: JobPostingInput): Promise<JobPosting> {
  return request('/api/v1/job-postings', json('POST', input))
}

export function replaceJobPosting(
  id: number,
  input: JobPostingInput,
): Promise<JobPosting> {
  return request(`/api/v1/job-postings/${id}`, json('PUT', input))
}

export function setJobPostingActive(
  id: number,
  isActive: boolean,
): Promise<JobPosting> {
  return request(
    `/api/v1/job-postings/${id}`,
    json('PATCH', { is_active: isActive }),
  )
}

export function deleteJobPosting(id: number): Promise<void> {
  return request(`/api/v1/job-postings/${id}`, { method: 'DELETE' })
}
