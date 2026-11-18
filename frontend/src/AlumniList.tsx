import type { Alumni } from './types'
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

export function AlumniList({ alumni }: { alumni: Alumni[] }) {
  if (alumni.length === 0) {
    return (
      <div className="m-6 border border-dashed border-line-strong p-6 text-center">
        <p className="type-label text-ink-muted">No records</p>
        <p className="mt-3 text-sm text-ink-muted">No alumni are loaded.</p>
      </div>
    )
  }

  return (
    <div className="px-4 pb-6 md:px-6">
      <Table className="min-w-[640px]">
        <TableCaption>Alumni / Web Development · Fall 2026–2027</TableCaption>
        <TableHeader>
          <TableRow className="h-12">
            <TableHead scope="col" className="w-16">
              No.
            </TableHead>
            <TableHead scope="col">Name</TableHead>
            <TableHead scope="col">Degree</TableHead>
            <TableHead scope="col" className="text-right">
              Graduation year
            </TableHead>
            <TableHead scope="col">Email</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {alumni.map((a, index) => (
            <TableRow key={a._id} className="h-14">
              <TableCell className="type-data text-ink-muted">
                {String(index + 1).padStart(2, '0')}
              </TableCell>
              <TableCell className="type-ui text-ink">{a.full_name}</TableCell>
              <TableCell className="type-ui text-ink">
                {a.degree?.trim() ? (
                  a.degree.trim()
                ) : (
                  <span className="text-ink-muted">Not recorded.</span>
                )}
              </TableCell>
              <TableCell className="type-data text-right text-ink">
                {a.graduation_year}
              </TableCell>
              <TableCell className="type-ui text-ink">
                {a.email?.trim() ? (
                  <a
                    href={`mailto:${a.email.trim()}`}
                    className="text-mulberry underline decoration-1 underline-offset-4 hover:decoration-rose"
                  >
                    {a.email.trim()}
                  </a>
                ) : (
                  <span className="text-ink-muted">Not recorded.</span>
                )}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  )
}
