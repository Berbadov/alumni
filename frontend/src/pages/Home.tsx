import { Link } from 'react-router-dom'
import { Button } from '@/components/ui/button'

export function Home() {
  return (
    <div className="space-y-6 py-16 text-center">
      <h1 className="text-4xl font-bold tracking-tight">
        Alumni Tracking System
      </h1>
      <p className="mx-auto max-w-xl text-lg text-muted-foreground">
        Web Development, Fall 2026&ndash;2027. Browse classmates, their degrees,
        and graduation years.
      </p>
      <Button asChild size="lg">
        <Link to="/alumni">Browse alumni</Link>
      </Button>
    </div>
  )
}
