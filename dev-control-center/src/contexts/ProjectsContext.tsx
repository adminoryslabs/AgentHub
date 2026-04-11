import { createContext, useContext, useState, useEffect, useCallback, type ReactNode } from 'react'
import { type Project, getProjects, createProject, updateProject, deleteProject, type CreateProjectRequest, type UpdateProjectRequest } from '../lib/invoke'

interface ProjectsContextType {
  projects: Project[]
  isLoading: boolean
  error: string | null
  addProject: (req: CreateProjectRequest) => Promise<void>
  editProject: (req: UpdateProjectRequest) => Promise<void>
  removeProject: (id: string) => Promise<void>
  refreshProjects: () => Promise<void>
}

const ProjectsContext = createContext<ProjectsContextType | undefined>(undefined)

export function ProjectsProvider({ children }: { children: ReactNode }) {
  const [projects, setProjects] = useState<Project[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const refreshProjects = useCallback(async () => {
    try {
      setIsLoading(true)
      setError(null)
      const data = await getProjects()
      setProjects(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err))
    } finally {
      setIsLoading(false)
    }
  }, [])

  useEffect(() => {
    refreshProjects()
  }, [refreshProjects])

  const addProject = useCallback(async (req: CreateProjectRequest) => {
    await createProject(req)
    await refreshProjects()
  }, [refreshProjects])

  const editProject = useCallback(async (req: UpdateProjectRequest) => {
    await updateProject(req)
    await refreshProjects()
  }, [refreshProjects])

  const removeProject = useCallback(async (id: string) => {
    await deleteProject(id)
    await refreshProjects()
  }, [refreshProjects])

  return (
    <ProjectsContext.Provider value={{ projects, isLoading, error, addProject, editProject, removeProject, refreshProjects }}>
      {children}
    </ProjectsContext.Provider>
  )
}

export function useProjects() {
  const context = useContext(ProjectsContext)
  if (context === undefined) {
    throw new Error('useProjects must be used within a ProjectsProvider')
  }
  return context
}
