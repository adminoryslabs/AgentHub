import { useCallback } from 'react'
import { getEcosystemNote, saveEcosystemNote } from '../lib/invoke'
import { NotesDialog } from './NotesDialog'

interface EcosystemNotesDialogProps {
  isOpen: boolean
  ecosystemId: string
  ecosystemName: string
  onClose: () => void
}

export function EcosystemNotesDialog({ isOpen, ecosystemId, ecosystemName, onClose }: EcosystemNotesDialogProps) {
  const handleLoad = useCallback(() => getEcosystemNote(ecosystemId), [ecosystemId])
  const handleSave = useCallback((content: string) => saveEcosystemNote(ecosystemId, content), [ecosystemId])

  return (
    <NotesDialog
      isOpen={isOpen}
      title="Ecosystem Notes"
      subtitle={ecosystemName}
      placeholder="No ecosystem notes yet — start typing..."
      onClose={onClose}
      onLoad={handleLoad}
      onSave={handleSave}
    />
  )
}
