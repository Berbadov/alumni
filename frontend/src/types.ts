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
