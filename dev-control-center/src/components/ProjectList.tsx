import { useState, useMemo } from 'react'
import { useProjects } from '../contexts/ProjectsContext'
import { useUI } from '../contexts/UIContext'
import { ProjectCard } from './ProjectCard'
import { AddProjectDialog } from './AddProjectDialog'

export function ProjectList() {
  const { projects, isLoading, error, removeProject } = useProjects()
  const { addToast } = useUI()
  const [isDialogOpen, setIsDialogOpen] = useState(false)
  const [editingProject, setEditingProject] = useState<any>(null)
  const [deleteConfirm, setDeleteConfirm] = useState<string | null>(null)
  const [searchQuery, setSearchQuery] = useState('')

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
        <AddProjectDialog
          isOpen={isDialogOpen}
          onClose={() => {
            setIsDialogOpen(false)
            setEditingProject(null)
          }}
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

        <button
          onClick={() => {
            setEditingProject(null)
            setIsDialogOpen(true)
          }}
          className="btn-primary shrink-0"
        >
          + Add Project
        </button>
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
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
          {filteredProjects.map(project => (
            <ProjectCard
              key={project.id}
              project={project}
              onEdit={() => handleEdit(project)}
              onDelete={() => handleDelete(project.id)}
              onOpenEditor={handleOpenEditor}
              onLaunchAgent={handleLaunchAgent}
              onError={handleError}
            />
          ))}
        </div>
      )}

      <AddProjectDialog
        isOpen={isDialogOpen}
        onClose={() => {
          setIsDialogOpen(false)
          setEditingProject(null)
        }}
        editingProject={editingProject}
      />

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
