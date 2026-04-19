import { useState, useEffect } from 'react'
import { getSessions, resumeAgentSession, openAgentSettings, type SessionEntry } from '../lib/invoke'
import { useUI } from '../contexts/UIContext'

interface SessionHistoryProps {
  projectPath: string
  projectId: string
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
}

const AGENT_COLORS: Record<string, string> = {
  claude: 'text-[#a78bfa]',
  qwen: 'text-[#00e475]',
}

export function SessionHistory({ projectPath, projectId }: SessionHistoryProps) {
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
      await resumeAgentSession(projectId, session.agent, session.sessionId)
      addToast(
        `${AGENT_LABELS[session.agent] || session.agent} session ${session.sessionId.slice(0, 8)}... opened`,
        'success'
      )
    } catch (err) {
      addToast(err instanceof Error ? err.message : String(err), 'error')
    }
  }

  const handleOpenSettings = async (agent: string) => {
    try {
      await openAgentSettings(agent)
      addToast(
        `${AGENT_LABELS[agent] || agent} settings opened`,
        'success'
      )
    } catch (err) {
      addToast(err instanceof Error ? err.message : String(err), 'error')
    }
  }

  // Group sessions by agent
  const grouped = sessions.reduce<Record<string, SessionEntry[]>>((acc, s) => {
    if (!acc[s.agent]) acc[s.agent] = []
    acc[s.agent].push(s)
    return acc
  }, {})

  if (!isExpanded && sessions.length === 0 && !isLoading) {
    return (
      <button
        onClick={() => setIsExpanded(true)}
        className="w-full text-left text-label-sm text-outline hover:text-secondary transition-colors py-1"
      >
        ▸ Sessions
      </button>
    )
  }

  return (
    <div className="mt-2 border-t border-outline/15 pt-2">
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="w-full text-left text-label-sm text-outline hover:text-secondary transition-colors flex items-center justify-between"
      >
        <span>Sessions ({sessions.length})</span>
        <span>{isExpanded ? '▾' : '▸'}</span>
      </button>

      {isExpanded && (
        <div className="mt-2 space-y-3 max-h-48 overflow-y-auto">
          {isLoading ? (
            <p className="text-label-sm text-outline">Loading sessions...</p>
          ) : sessions.length === 0 ? (
            <p className="text-label-sm text-outline">No agent sessions found</p>
          ) : (
            Object.entries(grouped).map(([agent, agentSessions]) => (
              <div key={agent}>
                <div className="flex items-center justify-between">
                  <p className={`text-label-sm font-medium ${AGENT_COLORS[agent] || 'text-secondary'}`}>
                    {AGENT_LABELS[agent] || agent} ({agentSessions.length})
                  </p>
                  <button
                    onClick={() => handleOpenSettings(agent)}
                    className="text-label-sm text-outline hover:text-secondary transition-colors"
                    title={`Open ${AGENT_LABELS[agent] || agent} settings`}
                  >
                    ⚙ Settings
                  </button>
                </div>
                <div className="mt-1 space-y-0.5">
                  {agentSessions.map((session, i) => (
                    <button
                      key={i}
                      onClick={() => handleOpenSession(session)}
                      className="w-full text-left px-2 py-1 rounded text-label-sm text-on-surface-variant hover:bg-surface-active transition-colors flex items-center justify-between"
                    >
                      <span className="truncate font-mono text-[10px]">{session.sessionId.slice(0, 8)}...</span>
                      <span className="shrink-0 text-on-surface-variant ml-2">
                        {formatDateRelative(session.modifiedAt)} · {formatSize(session.sizeBytes)}
                      </span>
                    </button>
                  ))}
                </div>
              </div>
            ))
          )}
        </div>
      )}
    </div>
  )
}
