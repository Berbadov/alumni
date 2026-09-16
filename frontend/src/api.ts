import type { Alumni } from './types'

export async function fetchAlumni(): Promise<Alumni[]> {
  const res = await fetch('/api/v1/alumni')
  if (!res.ok) {
    throw new Error(`Failed to load alumni (${res.status})`)
  }
  return res.json()
}
