import { invoke } from '@tauri-apps/api/core'

export type Project = {
  id: string
  name: string
  path: string
  env: string
  preferredEditor: string
  defaultAgent: string
  tags: string[]
  ecosystemId: string | null
  lastOpenedAt: string | null
  createdAt: string
}

export type Ecosystem = {
  id: string
  name: string
  rootPath: string
  env: string
  defaultAgent: string
  createdAt: string
}

export async function getEcosystems(): Promise<Ecosystem[]> {
  return invoke<Ecosystem[]>('get_ecosystems')
}

export type ScanEcosystemFolderCandidate = {
  name: string
  path: string
  isAlreadyRegistered: boolean
  existingProjectName: string | null
}

export type ScanEcosystemFolderResponse = {
  candidates: ScanEcosystemFolderCandidate[]
}

export type ScanEcosystemFolderRequest = {
  rootPath: string
  env: string
}

export async function scanEcosystemFolder(
  req: ScanEcosystemFolderRequest,
): Promise<ScanEcosystemFolderResponse> {
  return invoke<ScanEcosystemFolderResponse>('scan_ecosystem_folder', { req })
}

export type ImportEcosystemFolderRequest = {
  name: string
  rootPath: string
  env: string
  defaultAgent: string
  selectedPaths: string[]
}

export type ImportEcosystemFolderResponse = {
  ecosystem: Ecosystem
  importedProjects: Project[]
}

export async function importEcosystemFolder(
  req: ImportEcosystemFolderRequest,
): Promise<ImportEcosystemFolderResponse> {
  return invoke<ImportEcosystemFolderResponse>('import_ecosystem_folder', { req })
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
  ecosystemId: string | null
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
  ecosystemId: string | null
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

export async function launchEcosystemAgent(ecosystemId: string): Promise<string> {
  return invoke<string>('launch_ecosystem_agent', { req: { ecosystemId } })
}

export type SessionEntry = {
  agent: string
  sessionId: string
  modifiedAt: string
  sizeBytes: number
}

export async function getSessions(projectPath: string): Promise<SessionEntry[]> {
  return invoke<SessionEntry[]>('get_sessions', { req: { projectPath } })
}

export async function resumeAgentSession(
  projectId: string,
  agent: string,
  sessionId: string,
): Promise<string> {
  return invoke<string>('resume_agent_session', {
    req: { projectId, agent, sessionId },
  })
}

export async function openTerminal(projectId: string): Promise<string> {
  return invoke<string>('open_terminal', { req: { projectId } })
}

export async function openGlobalTerminal(shell: string): Promise<string> {
  return invoke<string>('open_global_terminal', { req: { shell } })
}

export async function openAgentSettings(agent: string): Promise<string> {
  return invoke<string>('open_agent_settings', { req: { agent } })
}

export async function pickDirectory(): Promise<string | null> {
  return invoke<string | null>('pick_directory')
}

export async function getProjectNote(projectId: string): Promise<string> {
  return invoke<string>('get_project_note', { req: { projectId } })
}

export async function saveProjectNote(projectId: string, content: string): Promise<void> {
  return invoke<void>('save_project_note', { req: { projectId, content } })
}

export async function getGeneralNote(): Promise<string> {
  return invoke<string>('get_general_note')
}

export async function saveGeneralNote(content: string): Promise<void> {
  return invoke<void>('save_general_note', { req: { content } })
}
