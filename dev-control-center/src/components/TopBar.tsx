import { useState } from 'react'
import { openGlobalTerminal } from '../lib/invoke'
import { useUI } from '../contexts/UIContext'
import { GeneralNotesDialog } from './GeneralNotesDialog'
import type { ProjectViewMode } from '../App'

interface TopBarProps {
  viewMode: ProjectViewMode
  onChangeViewMode: (mode: ProjectViewMode) => void
}

export function TopBar({ viewMode, onChangeViewMode }: TopBarProps) {
  const { addToast } = useUI()
  const [isGeneralNotesOpen, setIsGeneralNotesOpen] = useState(false)

  const handleOpenTerminal = async (shell: 'wsl' | 'powershell') => {
    try {
      await openGlobalTerminal(shell)
    } catch (err) {
      addToast(err instanceof Error ? err.message : String(err), 'error')
    }
  }

  return (
    <header className="border-b border-outline/15 px-4 py-2 flex items-center justify-between">
      <h1 className="text-headline-md font-headline text-secondary">Dev Control Center</h1>

      <div className="flex gap-1.5">
        <div className="flex gap-1 rounded-sm bg-surface-container-low p-1">
          <button
            onClick={() => onChangeViewMode('flat')}
            className={viewMode === 'flat' ? 'btn-primary text-sm px-3 py-1' : 'btn-ghost text-sm px-3 py-1'}
          >
            Flat
          </button>
          <button
            onClick={() => onChangeViewMode('ecosystem')}
            className={viewMode === 'ecosystem' ? 'btn-primary text-sm px-3 py-1' : 'btn-ghost text-sm px-3 py-1'}
          >
            By Ecosystem
          </button>
        </div>
        <button onClick={() => setIsGeneralNotesOpen(true)} className="btn-ghost text-sm px-3 py-1">
          Notes
        </button>
        <button onClick={() => handleOpenTerminal('wsl')} className="btn-ghost text-sm px-3 py-1">
          Terminal WSL
        </button>
        <button onClick={() => handleOpenTerminal('powershell')} className="btn-ghost text-sm px-3 py-1">
          Terminal PS
        </button>
      </div>

      <GeneralNotesDialog
        isOpen={isGeneralNotesOpen}
        onClose={() => setIsGeneralNotesOpen(false)}
      />
    </header>
  )
}
