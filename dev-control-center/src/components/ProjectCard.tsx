import { type Project } from '../lib/invoke'
import { openEditor, launchAgent } from '../lib/invoke'

interface ProjectCardProps {
  project: Project
  onEdit: () => void
  onDelete: () => void
  onOpenEditor?: (editor: string) => void
  onLaunchAgent?: (agent: string) => void
  onError?: (message: string) => void
}

export function ProjectCard({ project, onEdit, onDelete, onOpenEditor, onLaunchAgent, onError }: ProjectCardProps) {
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

  return (
    <div className="card">
      <div className="flex items-start justify-between">
        <div className="min-w-0 flex-1">
          <h3 className="text-sm font-headline text-secondary truncate">{project.name}</h3>
          <p className="text-label-sm text-outline font-mono mt-0.5 truncate">{project.path}</p>
        </div>
        <div className="flex gap-1 ml-2 shrink-0">
          <span className="glow-tag glow-tag--success text-label-sm">
            {project.env}
          </span>
        </div>
      </div>

      <div className="mt-3 space-y-1.5">
        {/* Action row: Continue (primary) */}
        <button
          onClick={() => handleLaunchAgent(project.defaultAgent)}
          className="btn-primary w-full text-center"
        >
          Continue with {project.defaultAgent}
        </button>

        {/* Editors row */}
        <div className="flex gap-1.5">
          <button onClick={() => handleOpenEditor('vscode')} className="btn-ghost flex-1">
            VSCode
          </button>
          <button onClick={() => handleOpenEditor('cursor')} className="btn-ghost flex-1">
            Cursor
          </button>
        </div>

        {/* Agents row */}
        <div className="flex gap-1.5">
          <button onClick={() => handleLaunchAgent('claude')} className="btn-ghost flex-1">
            Claude Code
          </button>
          <button onClick={() => handleLaunchAgent('opencode')} className="btn-ghost flex-1">
            OpenCode
          </button>
          <button onClick={() => handleLaunchAgent('qwen')} className="btn-ghost flex-1">
            QwenCode
          </button>
        </div>

        {/* Edit/Delete row */}
        <div className="flex gap-1.5 pt-0.5">
          <button onClick={onEdit} className="btn-ghost flex-1">
            Edit
          </button>
          <button onClick={onDelete} className="btn-danger flex-1">
            Delete
          </button>
        </div>
      </div>
    </div>
  )
}
