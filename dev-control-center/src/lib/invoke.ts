import { invoke } from '@tauri-apps/api/core'

export type Project = {
  id: string
  name: string
  path: string
  env: string
  preferredEditor: string
  defaultAgent: string
  tags: string[]
  lastOpenedAt: string | null
  createdAt: string
}

export async function getProjects(): Promise<Project[]> {
  return invoke<Project[]>('get_projects')
}

export type CreateProjectRequest = {
  name: string
  path: string
  env: string
  preferredEditor: string
  defaultAgent: string
  tags: string[]
}

export async function createProject(req: CreateProjectRequest): Promise<Project> {
  return invoke<Project>('create_project', { req })
}

export type UpdateProjectRequest = {
  id: string
  name: string
  path: string
  env: string
  preferredEditor: string
  defaultAgent: string
  tags: string[]
}

export async function updateProject(req: UpdateProjectRequest): Promise<Project> {
  return invoke<Project>('update_project', { req })
}

export async function deleteProject(id: string): Promise<void> {
  return invoke<void>('delete_project', { id })
}

export async function openEditor(projectId: string, editor: string): Promise<string> {
  return invoke<string>('open_editor', { req: { projectId, editor } })
}

export async function launchAgent(projectId: string, agent: string): Promise<string> {
  return invoke<string>('launch_agent', { req: { projectId, agent } })
}
