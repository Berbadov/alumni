import { useEffect, useState } from 'react'
import { fetchAlumni } from './api'
import { AlumniList } from './AlumniList'
import type { Alumni } from './types'

export function App() {
  const [alumni, setAlumni] = useState<Alumni[]>([])
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    fetchAlumni().then(setAlumni).catch((e: unknown) => setError(String(e)))
  }, [])

  return (
    <div style={{ fontFamily: 'system-ui, sans-serif', maxWidth: 720, margin: '0 auto', padding: 16 }}>
      <h1>Alumni &mdash; Web Development, Fall 2026&ndash;2027</h1>
      {error ? <p role="alert">Error: {error}</p> : <AlumniList alumni={alumni} />}
    </div>
  )
}
