import type { Alumni } from './types'

export function AlumniList({ alumni }: { alumni: Alumni[] }) {
  if (alumni.length === 0) {
    return <p>No alumni yet.</p>
  }
  return (
    <ul>
      {alumni.map((a) => (
        <li key={a._id}>
          <strong>{a.full_name}</strong> &mdash; {a.degree}, {a.graduation_year} ({a.email})
        </li>
      ))}
    </ul>
  )
}
