import { useEffect, useState } from 'react'
import { getGeneralNote, saveGeneralNote } from '../lib/invoke'
import { useUI } from '../contexts/UIContext'

interface GeneralNotesDialogProps {
  isOpen: boolean
  onClose: () => void
}

export function GeneralNotesDialog({ isOpen, onClose }: GeneralNotesDialogProps) {
  const { addToast } = useUI()
  const [content, setContent] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [isSaving, setIsSaving] = useState(false)
  const [error, setError] = useState('')

  useEffect(() => {
    if (!isOpen) return

    let cancelled = false
    setIsLoading(true)
    setError('')

    getGeneralNote()
      .then(note => {
        if (!cancelled) {
          setContent(note)
        }
      })
      .catch(err => {
        if (!cancelled) {
          setError(err instanceof Error ? err.message : String(err))
        }
      })
      .finally(() => {
        if (!cancelled) {
          setIsLoading(false)
        }
      })

    return () => {
      cancelled = true
    }
  }, [isOpen])

  const handleSave = async () => {
    setIsSaving(true)
    setError('')

    try {
      await saveGeneralNote(content)
      addToast('Saved', 'success')
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err)
      setError(message)
      addToast(message, 'error')
    } finally {
      setIsSaving(false)
    }
  }

  if (!isOpen) return null

  return (
    <div className="dialog-overlay" onClick={onClose}>
      <div className="dialog-backdrop" />
      <div className="dialog-content max-w-2xl" onClick={e => e.stopPropagation()}>
        <div className="flex items-start justify-between gap-4">
          <div>
            <h2 className="text-headline-md font-headline text-secondary">General Notes</h2>
            <p className="text-label-sm text-on-surface-variant mt-1">Shared scratchpad for uncategorized ideas</p>
          </div>
          <button onClick={onClose} className="btn-ghost shrink-0">
            Close
          </button>
        </div>

        <div className="mt-4">
          {isLoading ? (
            <p className="text-body-md text-on-surface-variant">Loading notes...</p>
          ) : (
            <textarea
              value={content}
              onChange={e => setContent(e.target.value)}
              onKeyDown={async e => {
                if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
                  e.preventDefault()
                  if (!isSaving) {
                    await handleSave()
                  }
                }
              }}
              className="input-field min-h-[320px] resize-y"
              placeholder="No notes yet — start typing..."
            />
          )}
        </div>

        {error && (
          <p className="mt-3 text-label-sm text-error">{error}</p>
        )}

        <div className="mt-4 flex justify-end gap-2">
          <button onClick={onClose} className="btn-ghost">
            Cancel
          </button>
          <button onClick={handleSave} disabled={isSaving || isLoading} className="btn-primary disabled:opacity-50">
            {isSaving ? 'Saving...' : 'Save'}
          </button>
        </div>
      </div>
    </div>
  )
}
