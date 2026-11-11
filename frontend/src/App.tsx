import { Navigate, Route, Routes } from 'react-router-dom'
import { Layout } from '@/components/Layout'
import { Home } from '@/pages/Home'
import { Alumni } from '@/pages/Alumni'
import { Jobs } from '@/pages/Jobs'
import { About } from '@/pages/About'
import { NotFound } from '@/pages/NotFound'

export function App() {
  return (
    <Routes>
      <Route element={<Layout />}>
        <Route index element={<Home />} />
        <Route path="alumni" element={<Alumni />} />
        <Route path="jobs" element={<Jobs />} />
        <Route path="about" element={<About />} />
        <Route path="404" element={<NotFound />} />
        <Route path="*" element={<Navigate to="/404" replace />} />
      </Route>
    </Routes>
  )
}
