import { useEffect, useState } from 'react'
import { fetchAlumni } from '@/api'
import { AlumniList } from '@/AlumniList'
import type { Alumni } from '@/types'
import { Card, CardContent } from '@/components/ui/card'

export function Alumni() {
  const [alumni, setAlumni] = useState<Alumni[]>([])
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    fetchAlumni().then(setAlumni).catch((e: unknown) => setError(String(e)))
  }, [])

  return (
    <div className="space-y-4">
      <h1 className="text-2xl font-semibold tracking-tight">
        Alumni &mdash; Web Development, Fall 2026&ndash;2027
      </h1>
      {error ? (
        <p role="alert" className="text-destructive">
          Error: {error}
        </p>
      ) : (
        <Card>
          <CardContent>
            <AlumniList alumni={alumni} />
          </CardContent>
        </Card>
      )}
    </div>
  )
}
