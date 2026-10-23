import type { Alumni, JobPosting, JobPostingInput } from './types'

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
  const res = await fetch(url, init)
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
