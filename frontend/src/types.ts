export type AcademicStatus =
  | 'unverified'
  | 'undergrad'
  | 'grad'
  | 'phd'
  | 'academic'
  | 'professor'
  | 'alumni'
  | 'admin'

export type AssignableRole = Exclude<AcademicStatus, 'unverified'>

export type VerificationStatus = 'pending' | 'verified' | 'rejected'

export interface User {
  _id: number
  first_name: string
  last_name: string
  full_name: string
  age: number
  email: string
  roles: AcademicStatus[]
  requested_role?: AcademicStatus
  graduation_date?: string
  verification_status: VerificationStatus
  verification_method?: string
  phone_number?: string
  email_verified: boolean
  phone_verified: boolean
  is_active: boolean
  created_at: string
  updated_at: string
}

export interface UpdateOwnProfileInput {
  first_name?: string
  last_name?: string
  phone_number?: string
  graduation_date?: string
  current_password?: string
  new_password?: string
}

export interface Announcement {
  _id: number
  title: string
  body: string
  author_id: number
  created_at: string
  updated_at: string
}

export interface AnnouncementInput {
  title: string
  body: string
  author_id: number
}

export interface RegisterInput {
  first_name: string
  last_name: string
  age: number
  email: string
  academic_status?: AcademicStatus
  graduation_date?: string
  password: string
}

export interface LoginInput {
  email: string
  password: string
}

export interface AuthSession {
  user: User
  expires_at: string
}

export interface VerifyUserInput {
  verification_status: 'verified' | 'rejected'
  roles?: AssignableRole[]
  verification_method?: string
}

export interface AdminUpdateUserInput {
  roles?: AssignableRole[]
  is_active?: boolean
}

export interface Alumni {
  _id: number
  full_name: string
  graduation_year: number
  email: string
  degree: string
  created_at: string
  updated_at: string
}

export interface JobPosting {
  _id: number
  title: string
  company: string
  description: string
  location: string
  employment_type: string
  apply_url: string
  author_id: number
  is_active: boolean
  created_at: string
  updated_at: string
}

export interface JobPostingInput {
  title: string
  company: string
  description: string
  location: string
  employment_type: string
  apply_url: string
  author_id: number
  is_active: boolean
}
