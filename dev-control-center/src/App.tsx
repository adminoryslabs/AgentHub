import { ProjectsProvider } from './contexts/ProjectsContext'
import { UIProvider } from './contexts/UIContext'
import { ProjectList } from './components/ProjectList'

function App() {
  return (
    <UIProvider>
      <ProjectsProvider>
        <div className="min-h-screen bg-surface">
          {/* Top bar */}
          <header className="border-b border-outline/15 px-4 py-2 flex items-center justify-between">
            <h1 className="text-headline-md font-headline text-secondary">Dev Control Center</h1>
          </header>

          {/* Main content */}
          <main className="p-4">
            <ProjectList />
          </main>
        </div>
      </ProjectsProvider>
    </UIProvider>
  )
}

export default App
