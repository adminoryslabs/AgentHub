import { useEffect, useState } from 'react'
import { type Project } from '../lib/invoke'
import { openEditor, launchAgent, openTerminal } from '../lib/invoke'
import { SessionHistory } from './SessionHistory'
import { ProjectNotesDialog } from './ProjectNotesDialog'

const EDITOR_OPTIONS = [
  { value: 'vscode', label: 'VSCode' },
  { value: 'cursor', label: 'Cursor' },
]

const CLI_OPTIONS = [
  { value: 'claude', label: 'Claude Code' },
  { value: 'opencode', label: 'OpenCode' },
  { value: 'qwen', label: 'QwenCode' },
]

interface ProjectCardProps {
  project: Project
  ecosystemName?: string | null
  onEdit: () => void
  onDelete: () => void
  onOpenEditor?: (editor: string) => void
  onLaunchAgent?: (agent: string) => void
  onError?: (message: string) => void
}

export function ProjectCard({ project, ecosystemName, onEdit, onDelete, onOpenEditor, onLaunchAgent, onError }: ProjectCardProps) {
  const [isNotesOpen, setIsNotesOpen] = useState(false)
  const [selectedEditor, setSelectedEditor] = useState(project.preferredEditor || 'vscode')
  const [selectedCli, setSelectedCli] = useState(project.defaultAgent || 'claude')

  useEffect(() => {
    setSelectedEditor(project.preferredEditor || 'vscode')
    setSelectedCli(project.defaultAgent || 'claude')
  }, [project.defaultAgent, project.preferredEditor])

  const handleOpenEditor = async (editor: string) => {
    try {
      await openEditor(project.id, editor)
      onOpenEditor?.(editor)
    } catch (err) {
      onError?.(err instanceof Error ? err.message : String(err))
    }
  }

  const handleLaunchAgent = async (agent: string) => {
    try {
      await launchAgent(project.id, agent)
      onLaunchAgent?.(agent)
    } catch (err) {
      onError?.(err instanceof Error ? err.message : String(err))
    }
  }

  const handleOpenTerminal = async () => {
    try {
      await openTerminal(project.id)
    } catch (err) {
      onError?.(err instanceof Error ? err.message : String(err))
    }
  }

  return (
    <div className="card">
      <div className="flex items-start justify-between">
        <div className="min-w-0 flex-1">
          <h3 className="text-sm font-headline text-secondary truncate">{project.name}</h3>
          <p className="text-label-sm text-outline font-mono mt-0.5 truncate">{project.path}</p>
          {ecosystemName && (
            <p className="text-label-sm text-tertiary font-mono mt-1 truncate">
              ecosystem: {ecosystemName}
            </p>
          )}
        </div>
        <div className="flex gap-1 ml-2 shrink-0">
          <span className="glow-tag glow-tag--success text-label-sm">
            {project.env}
          </span>
        </div>
      </div>

      <div className="mt-3 space-y-1.5">
        <button
          onClick={() => handleLaunchAgent(project.defaultAgent)}
          className="btn-primary w-full text-center"
        >
          Continue with {project.defaultAgent}
        </button>

        <div className="rounded border border-outline/15 px-2 py-2">
          <div className="flex items-center gap-2">
            <span className="w-12 shrink-0 text-label-sm text-outline">IDE</span>
            <select
              value={selectedEditor}
              onChange={event => setSelectedEditor(event.target.value)}
              className="input-field h-8 flex-1 py-1 text-xs"
            >
              {EDITOR_OPTIONS.map(option => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
            <button onClick={() => handleOpenEditor(selectedEditor)} className="btn-ghost shrink-0 px-3 py-1 text-xs">
              Open
            </button>
          </div>
        </div>

        <div className="rounded border border-outline/15 px-2 py-2">
          <div className="flex items-center gap-2">
            <span className="w-12 shrink-0 text-label-sm text-outline">CLI</span>
            <select
              value={selectedCli}
              onChange={event => setSelectedCli(event.target.value)}
              className="input-field h-8 flex-1 py-1 text-xs"
            >
              {CLI_OPTIONS.map(option => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
            <button onClick={() => handleLaunchAgent(selectedCli)} className="btn-ghost shrink-0 px-3 py-1 text-xs">
              Launch
            </button>
          </div>
        </div>

        <div className="grid grid-cols-2 gap-1.5">
          <button onClick={handleOpenTerminal} className="btn-ghost flex-1">
            Terminal
          </button>
          <button onClick={() => setIsNotesOpen(true)} className="btn-ghost flex-1">
            Notes
          </button>
        </div>

        <div className="grid grid-cols-2 gap-1.5 pt-0.5">
          <button onClick={onEdit} className="btn-ghost flex-1">
            Edit
          </button>
          <button onClick={onDelete} className="btn-danger flex-1">
            Delete
          </button>
        </div>
      </div>

      {/* Session history */}
      <SessionHistory projectPath={project.path} projectId={project.id} />

      <ProjectNotesDialog
        isOpen={isNotesOpen}
        projectId={project.id}
        projectName={project.name}
        onClose={() => setIsNotesOpen(false)}
      />
    </div>
  )
}
