import { useEffect, useMemo, useState } from 'react'
import {
  importEcosystemFolder,
  pickDirectory,
  scanEcosystemFolder,
  type ScanEcosystemFolderCandidate,
} from '../lib/invoke'
import { useProjects } from '../contexts/ProjectsContext'

interface AddEcosystemFolderDialogProps {
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

export function AddEcosystemFolderDialog({ isOpen, onClose }: AddEcosystemFolderDialogProps) {
  const { refreshProjects } = useProjects()
  const [name, setName] = useState('')
  const [rootPath, setRootPath] = useState('')
  const [env, setEnv] = useState('wsl')
  const [defaultAgent, setDefaultAgent] = useState('qwencode')
  const [candidates, setCandidates] = useState<ScanEcosystemFolderCandidate[]>([])
  const [selectedPaths, setSelectedPaths] = useState<Record<string, boolean>>({})
  const [error, setError] = useState('')
  const [isScanning, setIsScanning] = useState(false)
  const [isImporting, setIsImporting] = useState(false)

  useEffect(() => {
    if (!isOpen) {
      return
    }

    setName('')
    setRootPath('')
    setEnv('wsl')
    setDefaultAgent('qwencode')
    setCandidates([])
    setSelectedPaths({})
    setError('')
    setIsScanning(false)
    setIsImporting(false)
  }, [isOpen])

  const selectableCount = useMemo(() => {
    return candidates.filter(candidate => !candidate.isAlreadyRegistered).length
  }, [candidates])

  const selectedCount = useMemo(() => {
    return Object.values(selectedPaths).filter(Boolean).length
  }, [selectedPaths])

  const handleScan = async () => {
    if (!rootPath.trim()) {
      setError("El campo 'rootPath' es requerido")
      return
    }

    setIsScanning(true)
    setError('')
    try {
      const result = await scanEcosystemFolder({ rootPath: rootPath.trim(), env })
      setCandidates(result.candidates)
      setSelectedPaths(
        Object.fromEntries(
          result.candidates
            .filter(candidate => !candidate.isAlreadyRegistered)
            .map(candidate => [candidate.path, true]),
        ),
      )

      if (!name.trim()) {
        const folderName = rootPath.trim().split(/[/\\]/).filter(Boolean).at(-1)
        if (folderName) {
          setName(folderName)
        }
      }
    } catch (err) {
      setCandidates([])
      setSelectedPaths({})
      setError(err instanceof Error ? err.message : String(err))
    } finally {
      setIsScanning(false)
    }
  }

  const handleImport = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!name.trim()) {
      setError("El campo 'name' es requerido")
      return
    }

    if (!rootPath.trim()) {
      setError("El campo 'rootPath' es requerido")
      return
    }

    const paths = Object.entries(selectedPaths)
      .filter(([, selected]) => selected)
      .map(([path]) => path)

    if (paths.length === 0) {
      setError('Debes seleccionar al menos una carpeta para importar')
      return
    }

    setIsImporting(true)
    setError('')
    try {
      await importEcosystemFolder({
        name: name.trim(),
        rootPath: rootPath.trim(),
        env,
        defaultAgent,
        selectedPaths: paths,
      })
      await refreshProjects()
      onClose()
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err))
    } finally {
      setIsImporting(false)
    }
  }

  if (!isOpen) return null

  return (
    <div className="dialog-overlay" onClick={onClose}>
      <div className="dialog-backdrop" />
      <div className="dialog-content max-w-3xl" onClick={e => e.stopPropagation()}>
        <h2 className="text-headline-md font-headline text-secondary">Add Ecosystem Folder</h2>

        <form onSubmit={handleImport} className="mt-4 space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            <div>
              <label className="block text-label-sm text-secondary mb-1">Name *</label>
              <input
                type="text"
                value={name}
                onChange={e => setName(e.target.value)}
                className="input-field"
                placeholder="cosnautas"
              />
            </div>
            <div>
              <label className="block text-label-sm text-secondary mb-1">Default Agent</label>
              <select
                value={defaultAgent}
                onChange={e => setDefaultAgent(e.target.value)}
                className="input-field"
              >
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
                type="text"
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

          <div className="flex items-center justify-between gap-3 rounded-sm border border-outline/15 p-3">
            <div>
              <p className="text-sm text-secondary">Scan direct subfolders before importing</p>
              <p className="text-label-sm text-on-surface-variant mt-1">
                Duplicates are shown but cannot be selected.
              </p>
            </div>
            <button
              type="button"
              onClick={handleScan}
              disabled={isScanning}
              className="btn-ghost disabled:opacity-50"
            >
              {isScanning ? 'Scanning...' : 'Scan'}
            </button>
          </div>

          {candidates.length > 0 && (
            <div className="space-y-3 rounded-sm border border-outline/15 p-3">
              <div className="flex items-center justify-between gap-3">
                <div>
                  <p className="text-sm text-secondary">Detected child folders</p>
                  <p className="text-label-sm text-on-surface-variant mt-1">
                    {selectedCount} selected of {selectableCount} importable
                  </p>
                </div>
              </div>

              <div className="max-h-80 overflow-auto space-y-2">
                {candidates.map(candidate => (
                  <label
                    key={candidate.path}
                    className={`flex items-start gap-3 rounded-sm border p-2 ${candidate.isAlreadyRegistered ? 'border-error/30 opacity-70' : 'border-outline/15'}`}
                  >
                    <input
                      type="checkbox"
                      checked={selectedPaths[candidate.path] ?? false}
                      disabled={candidate.isAlreadyRegistered}
                      onChange={e => {
                        setSelectedPaths(current => ({
                          ...current,
                          [candidate.path]: e.target.checked,
                        }))
                      }}
                    />
                    <div className="min-w-0">
                      <p className="text-sm text-secondary truncate">{candidate.name}</p>
                      <p className="text-label-sm text-outline font-mono truncate mt-1">{candidate.path}</p>
                      {candidate.isAlreadyRegistered && (
                        <p className="text-label-sm text-error mt-1">
                          Already registered{candidate.existingProjectName ? ` as ${candidate.existingProjectName}` : ''}
                        </p>
                      )}
                    </div>
                  </label>
                ))}
              </div>
            </div>
          )}

          {error && <p className="text-label-sm text-error">{error}</p>}

          <div className="flex justify-end gap-2 pt-2">
            <button type="button" onClick={onClose} className="btn-ghost">
              Cancel
            </button>
            <button type="submit" disabled={isImporting} className="btn-primary disabled:opacity-50">
              {isImporting ? 'Importing...' : 'Create Ecosystem & Import'}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
