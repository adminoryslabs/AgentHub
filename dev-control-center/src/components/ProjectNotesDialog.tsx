import { useCallback } from 'react'
import { getProjectNote, saveProjectNote } from '../lib/invoke'
import { NotesDialog } from './NotesDialog'

interface ProjectNotesDialogProps {
  isOpen: boolean
  projectId: string
  projectName: string
  onClose: () => void
}

export function ProjectNotesDialog({ isOpen, projectId, projectName, onClose }: ProjectNotesDialogProps) {
  const handleLoad = useCallback(() => getProjectNote(projectId), [projectId])
  const handleSave = useCallback((content: string) => saveProjectNote(projectId, content), [projectId])

  return (
    <NotesDialog
      isOpen={isOpen}
      title="Project Notes"
      subtitle={projectName}
      onClose={onClose}
      onLoad={handleLoad}
      onSave={handleSave}
    />
  )
}
