import { Navigate, Route, Routes } from 'react-router-dom'
import { AuthProvider } from '@/auth'
import { Layout } from '@/components/Layout'
import { RequireAccess } from '@/components/RequireAccess'
import { Account } from '@/pages/Account'
import { Admin } from '@/pages/Admin'
import { About } from '@/pages/About'
import { Alumni } from '@/pages/Alumni'
import { FacultyWorkspace } from '@/pages/FacultyWorkspace'
import { GraduateWorkspace } from '@/pages/GraduateWorkspace'
import { Home } from '@/pages/Home'
import { Jobs } from '@/pages/Jobs'
import { Login } from '@/pages/Login'
import { NotFound } from '@/pages/NotFound'
import { Register } from '@/pages/Register'
import { StudentPortal } from '@/pages/StudentPortal'
import { Verification } from '@/pages/Verification'

export function App() {
  return (
    <AuthProvider>
      <Routes>
        <Route element={<Layout />}>
          <Route index element={<Home />} />
          <Route
            path="account"
            element={
              <RequireAccess
                access="authenticated"
                sectionLabel="07 / ACCOUNT"
                heading="Personal account ledger"
              >
                <Account />
              </RequireAccess>
            }
          />
          <Route
            path="verification"
            element={
              <RequireAccess
                access="unverified"
                sectionLabel="08 / VERIFICATION"
                heading="Paper verification status"
              >
                <Verification />
              </RequireAccess>
            }
          />
          <Route
            path="student"
            element={
              <RequireAccess
                access="verified"
                roles={['undergrad', 'grad', 'phd']}
                sectionLabel="08 / STUDENT WORKSPACE"
                heading="Student workspace"
              >
                <StudentPortal />
              </RequireAccess>
            }
          />
          <Route
            path="graduate"
            element={
              <RequireAccess
                access="verified"
                roles={['alumni']}
                sectionLabel="08 / GRADUATE WORKSPACE"
                heading="Graduate workspace"
              >
                <GraduateWorkspace />
              </RequireAccess>
            }
          />
          <Route
            path="faculty"
            element={
              <RequireAccess
                access="verified"
                roles={['academic', 'professor', 'admin']}
                sectionLabel="08 / FACULTY WORKSPACE"
                heading="Faculty workspace"
              >
                <FacultyWorkspace />
              </RequireAccess>
            }
          />
          <Route
            path="alumni"
            element={
              <RequireAccess
                access="verified"
                sectionLabel="02 / DIRECTORY"
                heading="The alumni directory"
              >
                <Alumni />
              </RequireAccess>
            }
          />
          <Route
            path="jobs"
            element={
              <RequireAccess
                access="verified"
                sectionLabel="03 / OFFERS"
                heading="Job postings"
              >
                <Jobs />
              </RequireAccess>
            }
          />
          <Route path="about" element={<About />} />
          <Route path="register" element={<Register />} />
          <Route path="login" element={<Login />} />
          <Route path="admin" element={<Admin />} />
          <Route path="404" element={<NotFound />} />
          <Route path="*" element={<Navigate to="/404" replace />} />
        </Route>
      </Routes>
    </AuthProvider>
  )
}
