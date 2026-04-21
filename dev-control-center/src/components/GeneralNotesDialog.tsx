import { useCallback } from 'react'
import { getGeneralNote, saveGeneralNote } from '../lib/invoke'
import { NotesDialog } from './NotesDialog'

interface GeneralNotesDialogProps {
  isOpen: boolean
  onClose: () => void
}

export function GeneralNotesDialog({ isOpen, onClose }: GeneralNotesDialogProps) {
  const handleLoad = useCallback(() => getGeneralNote(), [])
  const handleSave = useCallback((content: string) => saveGeneralNote(content), [])

  return (
    <NotesDialog
      isOpen={isOpen}
      title="General Notes"
      subtitle="Shared scratchpad for uncategorized ideas"
      onClose={onClose}
      onLoad={handleLoad}
      onSave={handleSave}
    />
  )
}
