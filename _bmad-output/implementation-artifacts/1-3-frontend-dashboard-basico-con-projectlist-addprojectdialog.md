---
story_key: 1-3-frontend-dashboard-basico-con-projectlist-addprojectdialog
status: done
epic: 1
epic_title: Setup & Gestión de Proyectos
---

# Story 1.3: Frontend — Dashboard Básico con ProjectList + AddProjectDialog

## Story

As a user,
I want to see my projects in a dashboard and add new ones,
So that I can visually manage my project list.

## Acceptance Criteria

**Given** no hay proyectos
**When** el usuario abre la app
**Then** ve un mensaje "No projects yet — click 'Add Project' to start"
**And** un botón "+ Add Project" en la barra superior

**Given** el usuario hace clic en "Add Project"
**When** se abre el AddProjectDialog
**Then** ve un formulario con campos: name, path, environment (dropdown: windows/wsl/mac), preferredEditor (dropdown: vscode/cursor), defaultAgent (dropdown: qwencode/claude/opencode), tags (texto libre opcional)
**And** los inputs usan fuente JetBrains Mono con background surface-container-lowest

**Given** el formulario de proyecto está completo
**When** el usuario hace clic en "Save"
**Then** el proyecto se crea vía `invoke('create_project', {...})`
**And** el dashboard se actualiza mostrando el nuevo proyecto
**And** el dialog se cierra

**Given** hay proyectos en la lista
**When** el usuario hace clic en "Edit" en un proyecto
**Then** se abre el AddProjectDialog con los datos pre-llenados
**And** al guardar, el proyecto se actualiza

**Given** hay proyectos en la lista
**When** el usuario hace clic en "Delete" en un proyecto
**Then** se muestra un confirm dialog
**And** si confirma, el proyecto se elimina y la lista se actualiza

## Tasks/Subtasks

- [x] Task 1: Crear capa de invocación Tauri
  - [x] Subtask 1.1: `src/lib/invoke.ts` con types Project, CreateProjectRequest, UpdateProjectRequest
  - [x] Subtask 1.2: Funciones getProjects, createProject, updateProject, deleteProject
- [x] Task 2: Crear ProjectsContext
  - [x] Subtask 2.1: Context + Provider con useState/useEffect/useCallback
  - [x] Subtask 2.2: Auto-load projects al montar
  - [x] Subtask 2.3: addProject, editProject, removeProject con auto-refresh
- [x] Task 3: Crear componentes de UI
  - [x] Subtask 3.1: `ProjectCard.tsx` — nombre, path, env badge, botones Edit/Delete
  - [x] Subtask 3.2: `ProjectList.tsx` — grid de cards + estado vacío + confirm delete
  - [x] Subtask 3.3: `AddProjectDialog.tsx` — formulario modal con validación
- [x] Task 4: Integrar en App.tsx
  - [x] Subtask 4.1: ProjectsProvider wrapper
  - [x] Subtask 4.2: Top bar con título y botón Add Project
  - [x] Subtask 4.3: ProjectList como contenido principal
- [x] Task 5: Verificar build
  - [x] Subtask 5.1: `npm run build` sin errores TypeScript
  - [x] Subtask 5.2: `npm run tauri build` compila exitosamente

## Dev Notes

### TypeScript
- `verbatimModuleSyntax` requiere `type` imports para tipos: `import { type Project }`
- `invoke()` args requieren `Record<string, unknown>` — cast con `as Record<string, unknown>`

### UI
- Dark mode: bg `#101419`, cards `#141920`, borders `#424754` al 15%
- Inputs: bg `#0d1117`, font JetBrains Mono, focus border `#00e475` al 80%
- Botones: primary bg `#00e475/20`, secondary ghost border

### Estado vacío
- Cuando projects.length === 0: mensaje centrado + botón "Add Project"

## File List

| File | Action |
|------|--------|
| `src/lib/invoke.ts` | Created (Tauri invoke wrappers + types) |
| `src/contexts/ProjectsContext.tsx` | Created (React context for project state) |
| `src/components/ProjectCard.tsx` | Created (single project card) |
| `src/components/ProjectList.tsx` | Created (project grid + empty state + dialogs) |
| `src/components/AddProjectDialog.tsx` | Created (add/edit project modal form) |
| `src/App.tsx` | Modified (ProjectsProvider + top bar + layout) |

## Change Log

- 5 nuevos archivos React/TypeScript
- CRUD completo conectado a commands Rust via Tauri invoke
- Estado vacío con CTA, formulario modal con validación, confirm delete
- Build exitoso: .deb + .rpm

## Dev Agent Record

### Implementation Plan
1. Crear invoke wrappers con types
2. Crear ProjectsContext con auto-refresh
3. Crear 3 componentes: ProjectCard, ProjectList, AddProjectDialog
4. Integrar en App.tsx con layout
5. Verificar build

### Completion Notes
- Type-only imports necesarios por verbatimModuleSyntax
- invoke() requiere Record<string, unknown> — cast aplicado
- Grid responsive: 1 col mobile, 2 col md, 3 col lg

### Debug Log
- TS1484 errors: type imports → fix: `import { type X }` syntax
- TS2345 errors: invoke args → fix: `as Record<string, unknown>` cast
