import { useState } from 'react'
import { ProjectsProvider } from './contexts/ProjectsContext'
import { UIProvider } from './contexts/UIContext'
import { ProjectList } from './components/ProjectList'
import { TopBar } from './components/TopBar'

export type ProjectViewMode = 'flat' | 'ecosystem'

function App() {
  const [viewMode, setViewMode] = useState<ProjectViewMode>('flat')

  return (
    <UIProvider>
      <ProjectsProvider>
        <div className="min-h-screen bg-surface">
          <TopBar viewMode={viewMode} onChangeViewMode={setViewMode} />

          {/* Main content */}
          <main className="p-4">
            <ProjectList viewMode={viewMode} />
          </main>
        </div>
      </ProjectsProvider>
    </UIProvider>
  )
}

export default App
