import type { Alumni } from './types'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

export function AlumniList({ alumni }: { alumni: Alumni[] }) {
  if (alumni.length === 0) {
    return <p className="text-muted-foreground">No alumni yet.</p>
  }
  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Name</TableHead>
          <TableHead>Degree</TableHead>
          <TableHead>Graduation year</TableHead>
          <TableHead>Email</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {alumni.map((a) => (
          <TableRow key={a._id}>
            <TableCell className="font-medium">{a.full_name}</TableCell>
            <TableCell>{a.degree}</TableCell>
            <TableCell>{a.graduation_year}</TableCell>
            <TableCell>{a.email}</TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}
