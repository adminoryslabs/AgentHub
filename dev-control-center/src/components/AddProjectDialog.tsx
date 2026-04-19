import { useState, useEffect } from 'react'
import { getEcosystems, type CreateProjectRequest, type Ecosystem, type Project, pickDirectory } from '../lib/invoke'
import { useProjects } from '../contexts/ProjectsContext'

interface AddProjectDialogProps {
  isOpen: boolean
  onClose: () => void
  editingProject?: Project | null
}

const ENVIRONMENTS = [
  { value: 'wsl', label: 'WSL' },
  { value: 'windows', label: 'Windows' },
  { value: 'mac', label: 'Mac' },
]

const EDITORS = [
  { value: 'vscode', label: 'VS Code' },
  { value: 'cursor', label: 'Cursor' },
]

const AGENTS = [
  { value: 'qwencode', label: 'QwenCode' },
  { value: 'claude', label: 'Claude Code' },
  { value: 'opencode', label: 'OpenCode' },
]

export function AddProjectDialog({ isOpen, onClose, editingProject }: AddProjectDialogProps) {
  const { addProject, editProject } = useProjects()
  const [name, setName] = useState('')
  const [path, setPath] = useState('')
  const [env, setEnv] = useState('wsl')
  const [preferredEditor, setPreferredEditor] = useState('vscode')
  const [defaultAgent, setDefaultAgent] = useState('qwencode')
  const [tags, setTags] = useState('')
  const [ecosystemId, setEcosystemId] = useState('')
  const [ecosystems, setEcosystems] = useState<Ecosystem[]>([])
  const [error, setError] = useState('')
  const [isSaving, setIsSaving] = useState(false)

  useEffect(() => {
    let cancelled = false

    const loadData = async () => {
      try {
        const data = await getEcosystems()
        if (!cancelled) {
          setEcosystems(data)
        }
      } catch {
        if (!cancelled) {
          setEcosystems([])
        }
      }
    }

    loadData()

    return () => {
      cancelled = true
    }
  }, [isOpen])

  useEffect(() => {
    if (editingProject) {
      setName(editingProject.name)
      setPath(editingProject.path)
      setEnv(editingProject.env)
      setPreferredEditor(editingProject.preferredEditor)
      setDefaultAgent(editingProject.defaultAgent)
      setTags(editingProject.tags.join(', '))
      setEcosystemId(editingProject.ecosystemId ?? '')
    } else {
      setName('')
      setPath('')
      setEnv('wsl')
      setPreferredEditor('vscode')
      setDefaultAgent('qwencode')
      setTags('')
      setEcosystemId('')
    }
    setError('')
  }, [editingProject, isOpen])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!name.trim()) {
      setError("El campo 'name' es requerido")
      return
    }
    setIsSaving(true)
    setError('')
    try {
      const req = {
        name: name.trim(),
        path: path.trim(),
        env,
        preferredEditor,
        defaultAgent,
        tags: tags.split(',').map(t => t.trim()).filter(Boolean),
        ecosystemId: ecosystemId || null,
      }
      if (editingProject) {
        await editProject({ ...req, id: editingProject.id })
      } else {
        await addProject(req as CreateProjectRequest)
      }
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err))
    } finally {
      setIsSaving(false)
    }
  }

  if (!isOpen) return null

  return (
    <div className="dialog-overlay" onClick={onClose}>
      <div className="dialog-backdrop" />
      <div className="dialog-content" onClick={e => e.stopPropagation()}>
        <h2 className="text-headline-md font-headline text-secondary">
          {editingProject ? 'Edit Project' : 'Add Project'}
        </h2>

        <form onSubmit={handleSubmit} className="mt-4 space-y-3">
          <div>
            <label className="block text-label-sm text-secondary mb-1">Name *</label>
            <input
              type="text"
              value={name}
              onChange={e => setName(e.target.value)}
              className="input-field"
              placeholder="my-project"
            />
          </div>

          <div className="flex items-center">
            <label className="block text-label-sm text-secondary mb-1 mr-2">Path</label>
            <input
              type="text"
              value={path}
              onChange={e => setPath(e.target.value)}
              className="input-field flex-1"
              placeholder="/home/user/dev/my-project"
            />
            <button
              type="button"
              onClick={async () => {
                try {
                  const dir = await pickDirectory()
                  if (dir) setPath(dir)
                } catch (e) {
                  // ignore, toast could be added later
                }
              }}
              className="btn-ghost ml-2"
            >
              Browse
            </button>
          </div>

          <div className="grid grid-cols-3 gap-3">
            <div>
              <label className="block text-label-sm text-secondary mb-1">Environment</label>
              <select value={env} onChange={e => setEnv(e.target.value)} className="input-field">
                {ENVIRONMENTS.map(e => (
                  <option key={e.value} value={e.value}>{e.label}</option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-label-sm text-secondary mb-1">Editor</label>
              <select value={preferredEditor} onChange={e => setPreferredEditor(e.target.value)} className="input-field">
                {EDITORS.map(e => (
                  <option key={e.value} value={e.value}>{e.label}</option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-label-sm text-secondary mb-1">Agent</label>
              <select value={defaultAgent} onChange={e => setDefaultAgent(e.target.value)} className="input-field">
                {AGENTS.map(e => (
                  <option key={e.value} value={e.value}>{e.label}</option>
                ))}
              </select>
            </div>
          </div>

          <div>
            <label className="block text-label-sm text-secondary mb-1">Tags (comma separated)</label>
            <input
              type="text"
              value={tags}
              onChange={e => setTags(e.target.value)}
              className="input-field"
              placeholder="backend, api"
            />
          </div>

          <div>
            <label className="block text-label-sm text-secondary mb-1">Ecosystem (optional)</label>
            <select value={ecosystemId} onChange={e => setEcosystemId(e.target.value)} className="input-field">
              <option value="">Ungrouped</option>
              {ecosystems.map(ecosystem => (
                <option key={ecosystem.id} value={ecosystem.id}>
                  {ecosystem.name}
                </option>
              ))}
            </select>
          </div>

          {error && (
            <p className="text-label-sm text-error">{error}</p>
          )}

          <div className="flex justify-end gap-2 pt-2">
            <button
              type="button"
              onClick={onClose}
              className="btn-ghost"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={isSaving}
              className="btn-primary disabled:opacity-50"
            >
              {isSaving ? 'Saving...' : editingProject ? 'Update' : 'Save'}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
