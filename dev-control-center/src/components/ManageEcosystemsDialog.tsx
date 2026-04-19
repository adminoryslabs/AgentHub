import { useEffect, useMemo, useState } from 'react'
import {
  createEcosystem,
  deleteEcosystem,
  getEcosystems,
  pickDirectory,
  updateEcosystem,
  type Ecosystem,
} from '../lib/invoke'
import { useProjects } from '../contexts/ProjectsContext'
import { useUI } from '../contexts/UIContext'

interface ManageEcosystemsDialogProps {
  isOpen: boolean
  onClose: () => void
}

const ENVIRONMENTS = [
  { value: 'wsl', label: 'WSL' },
  { value: 'windows', label: 'Windows' },
  { value: 'mac', label: 'Mac' },
]

const AGENTS = [
  { value: 'qwencode', label: 'QwenCode' },
  { value: 'claude', label: 'Claude Code' },
  { value: 'opencode', label: 'OpenCode' },
]

function normalizePathForComparison(path: string, env: string) {
  const separator = env === 'windows' ? '\\' : '/'
  let normalized = path.trim().replace(/[\\/]/g, separator)

  while (normalized.length > 1 && normalized.endsWith(separator)) {
    normalized = normalized.slice(0, -1)
  }

  return env === 'windows' ? normalized.toLowerCase() : normalized
}

function pathBelongsToRoot(projectPath: string, rootPath: string, env: string) {
  const project = normalizePathForComparison(projectPath, env)
  const root = normalizePathForComparison(rootPath, env)

  if (!project || !root) {
    return false
  }

  if (project === root) {
    return true
  }

  return project.startsWith(`${root}${env === 'windows' ? '\\' : '/'}`)
}

export function ManageEcosystemsDialog({ isOpen, onClose }: ManageEcosystemsDialogProps) {
  const { projects, editProject, refreshProjects } = useProjects()
  const { addToast } = useUI()
  const [ecosystems, setEcosystems] = useState<Ecosystem[]>([])
  const [selectedId, setSelectedId] = useState<string | 'new' | null>(null)
  const [name, setName] = useState('')
  const [rootPath, setRootPath] = useState('')
  const [env, setEnv] = useState('wsl')
  const [defaultAgent, setDefaultAgent] = useState('qwencode')
  const [assignedProjectIds, setAssignedProjectIds] = useState<Record<string, boolean>>({})
  const [error, setError] = useState('')
  const [isSaving, setIsSaving] = useState(false)
  const [isDeleting, setIsDeleting] = useState(false)

  const loadEcosystems = async () => {
    const data = await getEcosystems()
    setEcosystems(data)
    return data
  }

  useEffect(() => {
    if (!isOpen) return

    let cancelled = false
    setError('')

    loadEcosystems()
      .then(data => {
        if (cancelled) return
        const first = data[0]
        setSelectedId(first?.id ?? 'new')
      })
      .catch(err => {
        if (!cancelled) {
          setError(err instanceof Error ? err.message : String(err))
        }
      })

    return () => {
      cancelled = true
    }
  }, [isOpen])

  const selectedEcosystem = useMemo(() => {
    return selectedId && selectedId !== 'new'
      ? ecosystems.find(ecosystem => ecosystem.id === selectedId) ?? null
      : null
  }, [ecosystems, selectedId])

  useEffect(() => {
    if (!isOpen) return

    if (selectedEcosystem) {
      setName(selectedEcosystem.name)
      setRootPath(selectedEcosystem.rootPath)
      setEnv(selectedEcosystem.env)
      setDefaultAgent(selectedEcosystem.defaultAgent)
      setAssignedProjectIds(
        Object.fromEntries(
          projects
            .filter(project => project.ecosystemId === selectedEcosystem.id)
            .map(project => [project.id, true]),
        ),
      )
    } else {
      setName('')
      setRootPath('')
      setEnv('wsl')
      setDefaultAgent('qwencode')
      setAssignedProjectIds({})
    }

    setError('')
  }, [isOpen, projects, selectedEcosystem])

  const eligibleProjects = useMemo(() => {
    return projects.filter(project => project.env === env || assignedProjectIds[project.id])
  }, [assignedProjectIds, env, projects])

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!name.trim()) {
      setError("El campo 'name' es requerido")
      return
    }

    if (!rootPath.trim()) {
      setError("El campo 'rootPath' es requerido")
      return
    }

    setIsSaving(true)
    setError('')

    try {
      const saved = selectedEcosystem
        ? await updateEcosystem({
            id: selectedEcosystem.id,
            name: name.trim(),
            rootPath: rootPath.trim(),
            env,
            defaultAgent,
          })
        : await createEcosystem({
            name: name.trim(),
            rootPath: rootPath.trim(),
            env,
            defaultAgent,
          })

      const updates = projects.filter(project => {
        const shouldBeAssigned = assignedProjectIds[project.id] ?? false
        const isAssigned = project.ecosystemId === saved.id
        return shouldBeAssigned !== isAssigned
      })

      for (const project of updates) {
        const shouldBeAssigned = assignedProjectIds[project.id] ?? false
        await editProject({
          id: project.id,
          name: project.name,
          path: project.path,
          env: project.env,
          preferredEditor: project.preferredEditor,
          defaultAgent: project.defaultAgent,
          tags: project.tags,
          ecosystemId: shouldBeAssigned ? saved.id : null,
        })
      }

      await refreshProjects()
      const next = await loadEcosystems()
      setSelectedId(saved.id)
      setEcosystems(next)
      addToast(selectedEcosystem ? 'Ecosystem updated' : 'Ecosystem created', 'success')
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err))
    } finally {
      setIsSaving(false)
    }
  }

  const handleDelete = async () => {
    if (!selectedEcosystem) return

    setIsDeleting(true)
    setError('')
    try {
      await deleteEcosystem(selectedEcosystem.id)
      await refreshProjects()
      const next = await loadEcosystems()
      setEcosystems(next)
      setSelectedId(next[0]?.id ?? 'new')
      addToast('Ecosystem deleted', 'success')
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err))
    } finally {
      setIsDeleting(false)
    }
  }

  if (!isOpen) return null

  return (
    <div className="dialog-overlay" onClick={onClose}>
      <div className="dialog-backdrop" />
      <div className="dialog-content max-w-5xl" onClick={e => e.stopPropagation()}>
        <div className="flex items-start justify-between gap-4">
          <div>
            <h2 className="text-headline-md font-headline text-secondary">Manage Ecosystems</h2>
            <p className="text-label-sm text-on-surface-variant mt-1">
              Edit root path, default agent and project assignments.
            </p>
          </div>
          <button onClick={onClose} className="btn-ghost shrink-0">
            Close
          </button>
        </div>

        <div className="mt-4 grid grid-cols-1 lg:grid-cols-[220px,1fr] gap-4 min-h-[520px]">
          <aside className="rounded-sm border border-outline/15 p-3 space-y-2">
            <button
              type="button"
              onClick={() => setSelectedId('new')}
              className={selectedId === 'new' ? 'btn-primary w-full justify-center' : 'btn-ghost w-full justify-center'}
            >
              + New Ecosystem
            </button>
            <div className="space-y-2 pt-2">
              {ecosystems.map(ecosystem => (
                <button
                  key={ecosystem.id}
                  type="button"
                  onClick={() => setSelectedId(ecosystem.id)}
                  className={selectedId === ecosystem.id ? 'btn-primary w-full text-left' : 'btn-ghost w-full text-left'}
                >
                  {ecosystem.name}
                </button>
              ))}
            </div>
          </aside>

          <form onSubmit={handleSave} className="rounded-sm border border-outline/15 p-4 space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <div>
                <label className="block text-label-sm text-secondary mb-1">Name *</label>
                <input value={name} onChange={e => setName(e.target.value)} className="input-field" placeholder="cosnautas" />
              </div>
              <div>
                <label className="block text-label-sm text-secondary mb-1">Default Agent</label>
                <select value={defaultAgent} onChange={e => setDefaultAgent(e.target.value)} className="input-field">
                  {AGENTS.map(agent => (
                    <option key={agent.value} value={agent.value}>{agent.label}</option>
                  ))}
                </select>
              </div>
            </div>

            <div className="flex items-center gap-2">
              <div className="flex-1">
                <label className="block text-label-sm text-secondary mb-1">Root Path *</label>
                <input
                  value={rootPath}
                  onChange={e => setRootPath(e.target.value)}
                  className="input-field"
                  placeholder="/home/user/dev/cosnautas"
                />
              </div>
              <button
                type="button"
                onClick={async () => {
                  try {
                    const dir = await pickDirectory()
                    if (dir) setRootPath(dir)
                  } catch {
                    // ignore
                  }
                }}
                className="btn-ghost mt-6"
              >
                Browse
              </button>
            </div>

            <div className="w-full md:w-48">
              <label className="block text-label-sm text-secondary mb-1">Environment</label>
              <select value={env} onChange={e => setEnv(e.target.value)} className="input-field">
                {ENVIRONMENTS.map(environment => (
                  <option key={environment.value} value={environment.value}>{environment.label}</option>
                ))}
              </select>
            </div>

            <div className="space-y-3 rounded-sm border border-outline/15 p-3">
              <div>
                <p className="text-sm text-secondary">Assign existing projects</p>
                <p className="text-label-sm text-on-surface-variant mt-1">
                  Only projects inside the ecosystem root can be newly assigned.
                </p>
              </div>

              <div className="max-h-72 overflow-auto space-y-2">
                {eligibleProjects.map(project => {
                  const isAssigned = assignedProjectIds[project.id] ?? false
                  const belongsToRoot = pathBelongsToRoot(project.path, rootPath, env)
                  const disabled = !isAssigned && (!belongsToRoot || project.env !== env)

                  return (
                    <label key={project.id} className={`flex items-start gap-3 rounded-sm border p-2 ${disabled ? 'opacity-60 border-outline/10' : 'border-outline/15'}`}>
                      <input
                        type="checkbox"
                        checked={isAssigned}
                        disabled={disabled}
                        onChange={e => {
                          setAssignedProjectIds(current => ({
                            ...current,
                            [project.id]: e.target.checked,
                          }))
                        }}
                      />
                      <div className="min-w-0">
                        <p className="text-sm text-secondary truncate">{project.name}</p>
                        <p className="text-label-sm text-outline font-mono truncate mt-1">{project.path}</p>
                        {project.env !== env && (
                          <p className="text-label-sm text-error mt-1">Env mismatch: {project.env}</p>
                        )}
                        {project.env === env && !belongsToRoot && !isAssigned && (
                          <p className="text-label-sm text-error mt-1">Outside ecosystem root</p>
                        )}
                      </div>
                    </label>
                  )
                })}
              </div>
            </div>

            {error && <p className="text-label-sm text-error">{error}</p>}

            <div className="flex justify-between gap-2 pt-2">
              <div>
                {selectedEcosystem && (
                  <button type="button" onClick={handleDelete} disabled={isDeleting} className="btn-danger disabled:opacity-50">
                    {isDeleting ? 'Deleting...' : 'Delete Ecosystem'}
                  </button>
                )}
              </div>
              <div className="flex gap-2">
                <button type="button" onClick={onClose} className="btn-ghost">
                  Cancel
                </button>
                <button type="submit" disabled={isSaving} className="btn-primary disabled:opacity-50">
                  {isSaving ? 'Saving...' : selectedEcosystem ? 'Save Changes' : 'Create Ecosystem'}
                </button>
              </div>
            </div>
          </form>
        </div>
      </div>
    </div>
  )
}
