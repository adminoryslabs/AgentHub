import { useEffect, useMemo, useState } from 'react'
import type { ProjectViewMode } from '../App'
import { useProjects } from '../contexts/ProjectsContext'
import { useUI } from '../contexts/UIContext'
import { ProjectCard } from './ProjectCard'
import { AddProjectDialog } from './AddProjectDialog'
import { AddEcosystemFolderDialog } from './AddEcosystemFolderDialog'
import { EcosystemNotesDialog } from './EcosystemNotesDialog'
import { SessionHistory } from './SessionHistory'
import { getEcosystems, launchEcosystemAgent, openEcosystemEditor, type Ecosystem, type Project } from '../lib/invoke'

interface ProjectListProps {
  viewMode: ProjectViewMode
}

type EcosystemGroup = {
  key: string
  label: string
  ecosystemId: string | null
  preferredEditor: string | null
  defaultAgent: string | null
  rootPath: string | null
  projects: Project[]
}

const EDITOR_OPTIONS = [
  { value: 'vscode', label: 'VSCode' },
  { value: 'cursor', label: 'Cursor' },
]

const CLI_OPTIONS = [
  { value: 'claude', label: 'Claude Code' },
  { value: 'opencode', label: 'OpenCode' },
  { value: 'qwen', label: 'QwenCode' },
]

export function ProjectList({ viewMode }: ProjectListProps) {
  const { projects, isLoading, error, removeProject } = useProjects()
  const { addToast } = useUI()
  const [isDialogOpen, setIsDialogOpen] = useState(false)
  const [isEcosystemDialogOpen, setIsEcosystemDialogOpen] = useState(false)
  const [editingProject, setEditingProject] = useState<any>(null)
  const [deleteConfirm, setDeleteConfirm] = useState<string | null>(null)
  const [searchQuery, setSearchQuery] = useState('')
  const [expandedGroups, setExpandedGroups] = useState<Record<string, boolean>>({})
  const [ecosystems, setEcosystems] = useState<Ecosystem[]>([])
  const [notesTarget, setNotesTarget] = useState<{ id: string; name: string } | null>(null)
  const [ecosystemEditors, setEcosystemEditors] = useState<Record<string, string>>({})
  const [ecosystemAgents, setEcosystemAgents] = useState<Record<string, string>>({})

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
  }, [projects])

  const ecosystemsById = useMemo(() => {
    return new Map(ecosystems.map(ecosystem => [ecosystem.id, ecosystem]))
  }, [ecosystems])

  const handleEdit = (project: any) => {
    setEditingProject(project)
    setIsDialogOpen(true)
  }

  const handleDelete = (id: string) => {
    setDeleteConfirm(id)
  }

  const confirmDelete = async () => {
    if (deleteConfirm) {
      await removeProject(deleteConfirm)
      setDeleteConfirm(null)
    }
  }

  const handleOpenEditor = (editor: string) => {
    addToast(`${editor} opened`, 'success')
  }

  const handleLaunchAgent = (agent: string) => {
    addToast(`${agent} launched`, 'success')
  }

  const handleError = (message: string) => {
    addToast(message, 'error')
  }

  const handleLaunchEcosystem = async (ecosystemId: string, ecosystemName: string, agent: string) => {
    try {
      await launchEcosystemAgent(ecosystemId, agent)
      addToast(`${agent} launched for ecosystem ${ecosystemName}`, 'success')
    } catch (err) {
      addToast(err instanceof Error ? err.message : String(err), 'error')
    }
  }

  const handleOpenEcosystemEditor = async (ecosystemId: string, ecosystemName: string, editor: string) => {
    try {
      await openEcosystemEditor(ecosystemId, editor)
      addToast(`${editor} opened for ecosystem ${ecosystemName}`, 'success')
    } catch (err) {
      addToast(err instanceof Error ? err.message : String(err), 'error')
    }
  }

  // Sort by lastOpenedAt DESC, nulls last
  const sortedProjects = useMemo(() => {
    return [...projects].sort((a, b) => {
      const aTime = a.lastOpenedAt ? new Date(a.lastOpenedAt).getTime() : 0
      const bTime = b.lastOpenedAt ? new Date(b.lastOpenedAt).getTime() : 0
      return bTime - aTime
    })
  }, [projects])

  // Filter by search query (name, path, tags)
  const filteredProjects = useMemo(() => {
    if (!searchQuery.trim()) return sortedProjects
    const q = searchQuery.toLowerCase()
    return sortedProjects.filter(p =>
      p.name.toLowerCase().includes(q) ||
      p.path.toLowerCase().includes(q) ||
      p.tags.some(t => t.toLowerCase().includes(q))
    )
  }, [sortedProjects, searchQuery])

  const ecosystemGroups = useMemo<EcosystemGroup[]>(() => {
    const groups = new Map<string, EcosystemGroup>()

    for (const project of filteredProjects) {
      const ecosystem = project.ecosystemId ? ecosystemsById.get(project.ecosystemId) : null
      const label = ecosystem?.name || 'Ungrouped'
      const key = ecosystem?.id || '__ungrouped__'
      const group = groups.get(key)

      if (group) {
        group.projects.push(project)
        continue
      }

        groups.set(key, {
          key,
          label,
          ecosystemId: ecosystem?.id ?? null,
          preferredEditor: ecosystem?.preferredEditor ?? null,
          defaultAgent: ecosystem?.defaultAgent ?? null,
          rootPath: ecosystem?.rootPath ?? null,
          projects: [project],
        })
    }

    return Array.from(groups.values()).sort((a, b) => {
      if (a.key === '__ungrouped__') return 1
      if (b.key === '__ungrouped__') return -1
      return a.label.localeCompare(b.label)
    })
  }, [ecosystemsById, filteredProjects])

  useEffect(() => {
    if (viewMode !== 'ecosystem') {
      return
    }

    setExpandedGroups(current => {
      const next = { ...current }

      for (const group of ecosystemGroups) {
        if (!(group.key in next)) {
          next[group.key] = true
        }
      }

      return next
    })
  }, [ecosystemGroups, viewMode])

  useEffect(() => {
    setEcosystemEditors(current => {
      const next = { ...current }
      for (const group of ecosystemGroups) {
        if (group.ecosystemId && !next[group.ecosystemId]) {
          next[group.ecosystemId] = group.preferredEditor ?? 'vscode'
        }
      }
      return next
    })

    setEcosystemAgents(current => {
      const next = { ...current }
      for (const group of ecosystemGroups) {
        if (group.ecosystemId && !next[group.ecosystemId]) {
          next[group.ecosystemId] = group.defaultAgent ?? 'opencode'
        }
      }
      return next
    })
  }, [ecosystemGroups])

  const toggleGroup = (key: string) => {
    setExpandedGroups(current => ({
      ...current,
      [key]: !current[key],
    }))
  }

  const renderProjectGrid = (items: Project[]) => (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
      {items.map(project => (
        <ProjectCard
          key={project.id}
          project={project}
          ecosystemName={project.ecosystemId ? ecosystemsById.get(project.ecosystemId)?.name ?? null : null}
          onEdit={() => handleEdit(project)}
          onDelete={() => handleDelete(project.id)}
          onOpenEditor={handleOpenEditor}
          onLaunchAgent={handleLaunchAgent}
          onError={handleError}
        />
      ))}
    </div>
  )

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-20">
        <p className="text-body-md text-on-surface-variant">Loading projects...</p>
      </div>
    )
  }

  if (error) {
    return (
      <div className="flex items-center justify-center py-20">
        <p className="text-body-md text-error">Error loading projects: {error}</p>
      </div>
    )
  }

  if (projects.length === 0) {
    return (
      <div className="text-center py-20">
        <p className="text-body-md text-on-surface-variant mb-3">No projects yet</p>
        <button
          onClick={() => {
            setEditingProject(null)
            setIsDialogOpen(true)
          }}
          className="btn-primary px-4 py-2"
        >
          Add Project to get started
        </button>
        <button
          onClick={() => setIsEcosystemDialogOpen(true)}
          className="btn-ghost px-4 py-2 ml-2"
        >
          Add Ecosystem Folder
        </button>
        <AddProjectDialog
          isOpen={isDialogOpen}
          onClose={() => {
            setIsDialogOpen(false)
            setEditingProject(null)
          }}
        />
        <AddEcosystemFolderDialog
          isOpen={isEcosystemDialogOpen}
          onClose={() => setIsEcosystemDialogOpen(false)}
        />
      </div>
    )
  }

  return (
    <>
      {/* Toolbar */}
      <div className="flex items-center justify-between mb-3 gap-3">
        <span className="text-label-sm text-on-surface-variant shrink-0">
          {filteredProjects.length} of {projects.length} project{projects.length !== 1 ? 's' : ''}
        </span>

        {/* Search input */}
        <div className="relative flex-1 max-w-xs">
          <input
            type="text"
            value={searchQuery}
            onChange={e => setSearchQuery(e.target.value)}
            placeholder="Search by name, path, tags..."
            className="input-field pr-8 text-xs py-1"
          />
          {searchQuery && (
            <button
              onClick={() => setSearchQuery('')}
              className="absolute right-2 top-1/2 -translate-y-1/2 text-on-surface-variant hover:text-secondary"
            >
              ✕
            </button>
          )}
        </div>

        <span className="text-label-sm text-on-surface-variant shrink-0">
          {viewMode === 'flat' ? 'Flat' : 'By Ecosystem'}
        </span>

        <div className="flex gap-2 shrink-0">
          <button
            onClick={() => setIsEcosystemDialogOpen(true)}
            className="btn-ghost"
          >
            + Add Ecosystem Folder
          </button>
          <button
            onClick={() => {
              setEditingProject(null)
              setIsDialogOpen(true)
            }}
            className="btn-primary"
          >
            + Add Project
          </button>
        </div>
      </div>

      {/* Project grid */}
      {filteredProjects.length === 0 ? (
        <div className="text-center py-12">
          <p className="text-body-md text-on-surface-variant">No projects match &quot;{searchQuery}&quot;</p>
          <button
            onClick={() => setSearchQuery('')}
            className="btn-ghost mt-2"
          >
            Clear search
          </button>
        </div>
      ) : (
        viewMode === 'flat' ? (
          renderProjectGrid(filteredProjects)
        ) : (
          <div className="space-y-4">
            {ecosystemGroups.map(group => {
              const isExpanded = expandedGroups[group.key] ?? true
              const groupEditor = group.ecosystemId ? (ecosystemEditors[group.ecosystemId] ?? group.preferredEditor ?? 'vscode') : 'vscode'
              const groupAgent = group.defaultAgent ?? 'opencode'
              const selectedAgent = group.ecosystemId ? (ecosystemAgents[group.ecosystemId] ?? groupAgent) : groupAgent
              const canOpenAll = group.ecosystemId !== null

              return (
                <section key={group.key} className="card space-y-3">
                  <div className="relative flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
                    <div className="min-w-0 flex-1">
                      <h2 className="text-sm font-headline text-secondary">{group.label}</h2>
                      <p className="text-label-sm text-on-surface-variant mt-1">
                        {group.projects.length} project{group.projects.length !== 1 ? 's' : ''}
                      </p>
                      {group.rootPath && (
                        <p className="text-label-sm text-outline font-mono mt-1 break-all">
                          {group.rootPath}
                        </p>
                      )}
                    </div>

                    <div className="flex flex-wrap items-center justify-end gap-2 shrink-0 lg:max-w-[58%] lg:pr-20">
                      {canOpenAll && (
                        <>
                          <div className="rounded border border-outline/15 px-2 py-2 min-w-[280px] flex-1 lg:flex-none lg:w-[320px]">
                            <div className="flex items-center gap-2">
                              <span className="w-12 shrink-0 text-label-sm text-outline">IDE</span>
                              <select
                                value={groupEditor}
                                onChange={event =>
                                  setEcosystemEditors(current => ({
                                    ...current,
                                    [group.ecosystemId!]: event.target.value,
                                  }))
                                }
                                className="input-field h-8 flex-1 py-1 text-xs"
                              >
                                {EDITOR_OPTIONS.map(option => (
                                  <option key={option.value} value={option.value}>
                                    {option.label}
                                  </option>
                                ))}
                              </select>
                              <button
                                type="button"
                                onClick={() => handleOpenEcosystemEditor(group.ecosystemId!, group.label, groupEditor)}
                                className="btn-ghost shrink-0 px-3 py-1 text-xs"
                              >
                                Open
                              </button>
                            </div>
                          </div>

                          <div className="rounded border border-outline/15 px-2 py-2 min-w-[280px] flex-1 lg:flex-none lg:w-[320px]">
                            <div className="flex items-center gap-2">
                              <span className="w-12 shrink-0 text-label-sm text-outline">CLI</span>
                              <select
                                value={selectedAgent}
                                onChange={event =>
                                  setEcosystemAgents(current => ({
                                    ...current,
                                    [group.ecosystemId!]: event.target.value,
                                  }))
                                }
                                className="input-field h-8 flex-1 py-1 text-xs"
                              >
                                {CLI_OPTIONS.map(option => (
                                  <option key={option.value} value={option.value}>
                                    {option.label}
                                  </option>
                                ))}
                              </select>
                              <button
                                type="button"
                                onClick={() => handleLaunchEcosystem(group.ecosystemId!, group.label, selectedAgent)}
                                className="btn-ghost shrink-0 px-3 py-1 text-xs"
                              >
                                Launch
                              </button>
                            </div>
                          </div>

                          <button
                            type="button"
                            onClick={() => setNotesTarget({ id: group.ecosystemId!, name: group.label })}
                            className="btn-ghost inline-flex h-8 shrink-0 items-center px-3 py-1 text-xs"
                          >
                            Notes
                          </button>
                        </>
                      )}
                    </div>

                    <button
                      type="button"
                      onClick={() => toggleGroup(group.key)}
                      className="btn-ghost shrink-0 self-start px-1.5 py-0.5 text-[10px] leading-none lg:absolute lg:right-0 lg:top-0"
                    >
                      {isExpanded ? 'Collapse' : 'Expand'}
                    </button>
                  </div>

                  {canOpenAll && group.rootPath && (
                    <SessionHistory
                      projectPath={group.rootPath}
                      ecosystemId={group.ecosystemId!}
                      label="Ecosystem Sessions"
                    />
                  )}

                  {isExpanded && renderProjectGrid(group.projects)}
                </section>
              )
            })}
          </div>
        )
      )}

      <AddProjectDialog
        isOpen={isDialogOpen}
        onClose={() => {
          setIsDialogOpen(false)
          setEditingProject(null)
        }}
        editingProject={editingProject}
      />

      <AddEcosystemFolderDialog
        isOpen={isEcosystemDialogOpen}
        onClose={() => setIsEcosystemDialogOpen(false)}
      />

      {notesTarget && (
        <EcosystemNotesDialog
          isOpen={true}
          ecosystemId={notesTarget.id}
          ecosystemName={notesTarget.name}
          onClose={() => setNotesTarget(null)}
        />
      )}

      {deleteConfirm && (
        <div className="dialog-overlay" onClick={() => setDeleteConfirm(null)}>
          <div className="dialog-backdrop" />
          <div
            className="dialog-content border-error/30 max-w-sm"
            onClick={e => e.stopPropagation()}
          >
            <p className="text-body-md text-secondary">Are you sure you want to delete this project?</p>
            <div className="flex justify-end gap-2 mt-4">
              <button
                onClick={() => setDeleteConfirm(null)}
                className="btn-ghost"
              >
                Cancel
              </button>
              <button
                onClick={confirmDelete}
                className="btn-danger"
              >
                Delete
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  )
}
