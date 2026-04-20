import { useState, useEffect } from 'react'
import {
  getSessions,
  resumeAgentSession,
  resumeEcosystemAgentSession,
  type SessionEntry,
} from '../lib/invoke'
import { useUI } from '../contexts/UIContext'

interface SessionHistoryProps {
  projectPath: string
  projectId?: string
  ecosystemId?: string
  label?: string
}

function formatDateRelative(isoString: string): string {
  const date = new Date(isoString)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60))
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

  if (diffHours < 1) return 'just now'
  if (diffHours < 24) return `${diffHours}h ago`
  if (diffDays === 1) return 'yesterday'
  if (diffDays < 7) return `${diffDays}d ago`
  return date.toLocaleDateString()
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes}B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)}KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)}MB`
}

const AGENT_LABELS: Record<string, string> = {
  claude: 'Claude',
  qwen: 'Qwen',
  opencode: 'OpenCode',
}

const AGENT_COLORS: Record<string, string> = {
  claude: 'text-[#a78bfa]',
  qwen: 'text-[#00e475]',
  opencode: 'text-[#f97316]',
}

export function SessionHistory({ projectPath, projectId, ecosystemId, label = 'Sessions' }: SessionHistoryProps) {
  const [sessions, setSessions] = useState<SessionEntry[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [isExpanded, setIsExpanded] = useState(false)
  const { addToast } = useUI()

  useEffect(() => {
    if (!isExpanded) return
    let cancelled = false
    setIsLoading(true)
    getSessions(projectPath)
      .then(data => { if (!cancelled) setSessions(data) })
      .catch(() => { /* silently fail */ })
      .finally(() => { if (!cancelled) setIsLoading(false) })
    return () => { cancelled = true }
  }, [projectPath, isExpanded])

  const handleOpenSession = async (session: SessionEntry) => {
    try {
      if (ecosystemId) {
        await resumeEcosystemAgentSession(ecosystemId, session.agent, session.sessionId)
      } else if (projectId) {
        await resumeAgentSession(projectId, session.agent, session.sessionId)
      } else {
        throw new Error('No session target configured')
      }
      addToast(
        `${AGENT_LABELS[session.agent] || session.agent} session opened`,
        'success'
      )
    } catch (err) {
      addToast(err instanceof Error ? err.message : String(err), 'error')
    }
  }

  if (!isExpanded && sessions.length === 0 && !isLoading) {
    return (
      <button
        onClick={() => setIsExpanded(true)}
        className="w-full text-left text-label-sm text-outline hover:text-secondary transition-colors py-1"
      >
        ▸ {label}
      </button>
    )
  }

  return (
    <div className="mt-2 border-t border-outline/15 pt-2">
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="w-full text-left text-label-sm text-outline hover:text-secondary transition-colors flex items-center justify-between"
      >
          <span>{label} ({sessions.length})</span>
        <span>{isExpanded ? '▾' : '▸'}</span>
      </button>

      {isExpanded && (
        <div className="mt-2 space-y-2 max-h-56 overflow-y-auto">
          {isLoading ? (
            <p className="text-label-sm text-outline">Loading sessions...</p>
          ) : sessions.length === 0 ? (
            <p className="text-label-sm text-outline">No agent sessions found</p>
          ) : (
            <>
              <div className="space-y-1">
                {sessions.map(session => (
                  <button
                    key={`${session.agent}-${session.sessionId}`}
                    onClick={() => handleOpenSession(session)}
                    className="w-full rounded px-2 py-2 text-left hover:bg-surface-active transition-colors"
                  >
                    <div className="flex items-start justify-between gap-2">
                      <div className="min-w-0 flex-1">
                        <div className="flex items-center gap-2">
                          <span className={`text-[11px] font-medium ${AGENT_COLORS[session.agent] || 'text-secondary'}`}>
                            {AGENT_LABELS[session.agent] || session.agent}
                          </span>
                        </div>
                        <p className="mt-0.5 truncate text-label-sm text-on-surface-variant">
                          {session.title}
                        </p>
                      </div>
                      <span className="shrink-0 text-[11px] text-outline">
                        {formatDateRelative(session.modifiedAt)}
                      </span>
                    </div>
                    <p className="mt-1 text-[11px] text-outline">
                      {formatSize(session.sizeBytes)}
                    </p>
                  </button>
                ))}
              </div>
            </>
          )}
        </div>
      )}
    </div>
  )
}
