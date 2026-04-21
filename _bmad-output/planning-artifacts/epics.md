---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - '/home/marioyahuar/AgentHub/PRD.md'
  - '/home/marioyahuar/AgentHub/_bmad-output/planning-artifacts/architecture.md'
  - '/home/marioyahuar/AgentHub/DESIGN.md'
  - '/home/marioyahuar/AgentHub/code.html'
  - '/home/marioyahuar/AgentHub/_bmad-output/planning-artifacts/prd-validation-report.md'
---

# Dev Control Center - Epic Breakdown

## Overview

Este documento contiene la descomposición completa de epics y stories para **Dev Control Center**, una aplicación de escritorio Tauri v2 que centraliza gestión de proyectos, ejecución de agentes de IA y contexto de trabajo para 2 usuarios (multi-máquina: Windows, WSL, Mac) con memorias compartidas vía Neon PostgreSQL.

## Estado Actual

Estado documental reconciliado contra el código actual, `implementation-artifacts` y validación manual del producto al 2026-04-20.

| Epic | Estado actual | Nota |
|------|---------------|------|
| Epic 1 | Done | Documentado en implementation artifacts |
| Epic 2 | Done | Incluye 2.3 aunque faltaba artifact dedicado |
| Epic 3 | Done | 3.1 y 3.2 estaban implementados pero el artifact seguia incompleto |
| Epic 4 | Done | Faltaban artifacts 4.x |
| Epic 5 | Done | Implementado y cerrado |
| Epic 6 | In progress | 6.1-6.4 y 6.6 implementados; 6.5 sigue en backlog |
| Epic 7 | Done | Implementado y cerrado |
| Epic 8 | Done | Implementado y cerrado |
| Continue Work (Neon) | Blocked / deprecated candidate | Sigue dependiendo de Neon |
| Dashboard con estado (Neon) | Blocked / deprecated candidate | Sigue dependiendo de Neon |

> Nota: los epics 4, 5, 7 y 8 reflejan la evolucion real del producto despues del plan inicial y hoy se consideran cerrados. Epic 6 sigue activo por la story 6.5 en backlog. Los epics dependientes de Neon permanecen fuera del flujo activo y hoy se consideran bloqueados, con alta probabilidad de quedar deprecados.

## Requirements Inventory

### Functional Requirements

FR1: El sistema debe permitir crear proyectos manualmente con los campos: name, path, environment (windows|wsl|mac), preferredEditor (vscode|cursor), defaultAgent (qwencode|claude|opencode), tags (opcional)
FR2: El sistema debe permitir editar proyectos existentes
FR3: El sistema debe permitir eliminar proyectos
FR4: El sistema debe listar todos los proyectos registrados en una vista de dashboard
FR5: El sistema debe permitir abrir un proyecto en VS Code mediante comando local del SO
FR6: El sistema debe permitir abrir un proyecto en Cursor mediante comando local del SO
FR7: El sistema debe permitir abrir un terminal externo con Claude Code para un proyecto
FR8: El sistema debe permitir abrir un terminal externo con OpenCode para un proyecto
FR9: El sistema debe permitir abrir un terminal externo con QwenCode para un proyecto
FR10: El sistema debe adaptar los comandos de ejecución según el entorno del proyecto (Windows, WSL, Mac)
FR11: El sistema debe convertir rutas WSL a Windows (wslpath -w) al abrir editores
FR12: El sistema debe permitir al usuario hacer clic en "Continuar trabajo" y seleccionar un agente (Claude Code, OpenCode, QwenCode)
FR13: El sistema debe obtener las últimas memorias del proyecto desde Neon al ejecutar "Continuar trabajo"
FR14: El sistema debe construir un contexto con la última actividad y pendientes del proyecto
FR15: El sistema debe lanzar el agente seleccionado en terminal externo con el contexto inyectado
FR16: El sistema debe conectar directamente a Neon PostgreSQL para leer memorias por proyecto
FR17: El sistema debe mostrar en cada project card: última actividad (resumen), último agente usado, timestamp
FR18: El sistema debe persistir la configuración de proyectos en un archivo local `~/.dev-control-center/projects.json`
FR19: El sistema debe leer la URL de conexión a Neon desde un archivo `.env` local

### Non-Functional Requirements

NFR1: La aplicación debe funcionar nativamente en Windows, WSL y Mac sin configuración manual compleja
NFR2: Las credenciales de Neon nunca deben exponerse al frontend React
NFR3: Los agentes deben ejecutarse en terminales externos del SO, no embebidos en la app
NFR4: La app debe usar Tauri v2 como framework de escritorio
NFR5: La comunicación entre frontend y core debe ser vía IPC directo (`invoke()`), sin HTTP
NFR6: Los permisos de Tauri deben limitarse a `shell:allow-execute` y `fs:read-write`
NFR7: Las queries a Neon deben estar indexadas por `project_id + timestamp`
NFR8: Si Neon no responde, el sistema debe usar cache local con indicador visual "última sync: hace X min"
NFR9: Antes de ejecutar cualquier comando, el sistema debe validar que existe en PATH
NFR10: La app debe seguir el design system "The Command Matrix" (dark mode, alta densidad, Space Grotesk + Inter/JetBrains Mono)
NFR11: Las compilaciones deben generarse como instaladores nativos (.msi, .dmg, .deb)
NFR12: El código debe seguir los patrones de naming: `snake_case` para Rust, `camelCase` para JSON, `PascalCase` para componentes React
NFR13: Las respuestas de Tauri commands deben usar `Result<T, String>` nativo
NFR14: Los errores de comandos deben mostrarse como toast al usuario, sin stack traces

### Additional Requirements (Architecture)

- Starter: `create-tauri-app` con React + TypeScript + npm
- Post-init: instalar `tailwindcss @tailwindcss/vite` y `@tauri-apps/api`
- Driver PostgreSQL: `sqlx` con feature `runtime-tokio`
- State management: React Context + Hooks (3 contexts: ProjectsContext, NeonContext, UIContext)
- Persistencia local: `projects.json` en `~/.dev-control-center/`
- Tauri commands organizados en módulos: `projects.rs`, `neon.rs`, `actions.rs`
- Neon schema: tabla `memories` con campos id, content, title, metadata (JSONB), status, created_at, updated_at
- Permisos Tauri v2: allowlist explícita de comandos shell (code, cursor, claude, opencode, qwen, wsl, which, where, wt, open, bash, cmd)
- Frontend single-view — sin router
- Tests React co-locados, tests Rust al final de cada módulo

### UX Design Requirements

UX-DR1: Implementar sistema de colores basado en la paleta "The Command Matrix": background #101419, primary #00e475, secondary #adc6ff, tertiary #ffb95f, outline-variant #424754
UX-DR2: Implementar "No-Line Rule" — prohibir bordes sólidos de 1px para separar secciones; usar shifts de background y espacio negativo
UX-DR3: Implementar tipografía dual: Space Grotesk para headlines, Inter para UI, JetBrains Mono para inputs/código
UX-DR4: Implementar "Glow Tags" — chips de status con un dot de 6px del color de status + texto label-sm, sin background
UX-DR5: Implementar Project Cards con: nombre, estado, última actividad, y 6 botones de acción (Continue, VSCode, Cursor, Claude Code, OpenCode, QwenCode)
UX-DR6: Implementar barra superior con: título de la app y botón "Add Project"
UX-DR7: Implementar AgentSelector como modal para seleccionar agente al hacer clic en "Continue Work"
UX-DR8: Implementar AddProjectDialog como formulario con campos: name, path, environment, preferredEditor, defaultAgent, tags
UX-DR9: Implementar StatusBar con indicador de conexión a Neon y timestamp de última sync
UX-DR10: Aplicar "Ghost Border" fallback — bordes con outline-variant al 15% de opacidad cuando sean necesarios
UX-DR11: Aplicar texturas de "Subtle Glow" — surface_tint al 10% con blur de 12px en indicadores de status crítico
UX-DR12: Inputs de formulario con background surface-container-lowest, sin anillos de focus gruesos, fuente JetBrains Mono
UX-DR13: Layout con alta densidad — padding máximo 12-16px, radius 0.25rem, sin "marketing air"

### FR Coverage Map

| FR | Epic | Story |
|----|------|-------|
| FR1-4 | Epic 1: Setup + CRUD Proyectos | 1.1, 1.2, 1.3 |
| FR5-6 | Epic 2: Acciones Locales (Editores) | 2.1 |
| FR7-9 | Epic 3: Acciones Locales (Agentes) | 3.1 |
| FR10-11 | Epic 2: Acciones Locales (Multi-env) | 2.2 |
| FR12-15 | Epic 4: Continue Work (Core) | 4.1, 4.2, 4.3 |
| FR16 | Epic 1: Setup + Neon Connection | 1.4 |
| FR17 | ~~Epic 4: Dashboard Completo~~ → **Bloqueado** | Espera Neon |
| FR18-19 | Epic 1: Setup + Config | 1.1, 1.2 |
| NFR1-14 | Transversal | Aplicado en todas las stories |
| UX-DR1-13 | Epic 3: Polish Visual | 3.1, 3.2

## Epic List

### Epic 1: Setup & Gestión de Proyectos
**El usuario puede crear, editar y eliminar proyectos en un dashboard funcional.**
**FRs covered:** FR1, FR2, FR3, FR4, FR18, FR19
**NFRs:** NFR1, NFR4, NFR5, NFR6, NFR12, NFR13
**UX-DRs:** UX-DR6, UX-DR8, UX-DR12, UX-DR13

### Epic 2: Acciones Locales — Abrir Herramientas
**El usuario puede abrir cualquier proyecto en su editor preferido o lanzar agentes desde un clic.**
**FRs covered:** FR5, FR6, FR7, FR8, FR9, FR10, FR11
**NFRs:** NFR3, NFR6, NFR9, NFR14
**UX-DRs:** UX-DR5

### Epic 3: Polish Visual — The Command Matrix
**La app tiene el look & feel "Command Matrix": dark, densa, profesional.**
**FRs covered:** Transversal (aplica sobre todas las epics anteriores)
**NFRs:** NFR10, NFR11
**UX-DRs:** UX-DR1, UX-DR2, UX-DR3, UX-DR4, UX-DR9, UX-DR10, UX-DR11, UX-DR13

### Epic 4: Mejoras de Usabilidad
**El usuario puede buscar, filtrar, y ver sus proyectos ordenados por uso reciente.**
**FRs covered:** Nuevas (no en PRD original)
**NFRs:** NFR1, NFR12

### Epic 5: Historial de Sesiones
**El usuario puede ver y reabrir sesiones anteriores de agentes por proyecto.**
**FRs covered:** Nuevas (no en PRD original)
**NFRs:** NFR3, NFR12

### ⏸️ Bloqueados / Diferidos

### ~~Epic: Continue Work (Core Feature)~~ → **Bloqueado**
**El usuario puede retomar su trabajo donde lo dejó, con contexto inyectado en el agente.**
**FRs covered:** FR12, FR13, FR14, FR15, FR16
**Bloqueado por:** El sistema de memorias en Neon aún no está listo. Stories 3.1-3.3 (originales) se retoman cuando haya datos en Neon.

### ~~Epic: Dashboard Completo con Estado~~ → **Bloqueado**
**El usuario ve el estado de todos sus proyectos de un vistazo: última actividad, agente, timestamp.**
**FRs covered:** FR17
**Bloqueado por:** Depende de datos de Neon. Stories 4.1-4.2 (originales) se retoman cuando haya datos en Neon.

## Epic 1: Setup & Gestión de Proyectos

**Objetivo:** El usuario puede crear, editar y eliminar proyectos en un dashboard funcional con conexión a Neon.

### Story 1.1: Scaffold Tauri App + Configurar Permisos

As a developer,
I want a working Tauri project configured with proper permissions and dependencies,
So that I have a solid foundation for all features.

**Acceptance Criteria:**

**Given** un directorio vacío
**When** ejecuto `npm create tauri-app@latest dev-control-center` con React + TypeScript
**And** instalo `tailwindcss @tailwindcss/vite` y `@tauri-apps/api`
**And** instalo `sqlx` en `src-tauri/Cargo.toml` con features `runtime-tokio` y `postgres`
**And** configuro `src-tauri/capabilities/default.json` con permisos `shell:allow-execute` (allowlist de comandos) y `fs:read-write` (scope `~/.dev-control-center/**`)
**Then** `npm run tauri dev` compila y abre la ventana desktop
**And** el frontend muestra un placeholder básico

### Story 1.2: CRUD Proyectos en Rust (projects.json)

As a user,
I want to create, edit, and delete projects stored locally,
So that I can manage my development projects.

**Acceptance Criteria:**

**Given** el archivo `~/.dev-control-center/projects.json` no existe
**When** el comando Tauri `get_projects` se ejecuta
**Then** retorna una lista vacía `[]` sin error

**Given** el archivo `~/.dev-control-center/projects.json` no existe
**When** el comando Tauri `create_project` se ejecuta con `{ name: "test", path: "/dev/test", env: "wsl", preferredEditor: "vscode", defaultAgent: "qwencode", tags: [] }`
**Then** el proyecto se crea con un UUID, se guarda en `projects.json`, y retorna el proyecto completo con `createdAt` y `lastOpenedAt`

**Given** un proyecto existente con id `abc-123`
**When** el comando Tauri `update_project` se ejecuta con `{ id: "abc-123", name: "updated-name", ... }`
**Then** el proyecto se actualiza en `projects.json` y retorna el proyecto actualizado

**Given** un proyecto existente con id `abc-123`
**When** el comando Tauri `delete_project` se ejecuta con `{ id: "abc-123" }`
**Then** el proyecto se elimina de `projects.json` y `get_projects` ya no lo retorna

**Given** un comando `create_project` sin campo `name`
**When** se ejecuta
**Then** retorna `Err("El campo 'name' es requerido")`

### Story 1.3: Frontend — Dashboard Básico con ProjectList + AddProjectDialog

As a user,
I want to see my projects in a dashboard and add new ones,
So that I can visually manage my project list.

**Acceptance Criteria:**

**Given** no hay proyectos
**When** el usuario abre la app
**Then** ve un mensaje "No projects yet — click 'Add Project' to start"
**And** un botón "Add Project" en la barra superior

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

### Story 1.4: Conexión a Neon (Read-Only)

As a user,
I want the app to connect to Neon PostgreSQL and fetch project memories,
So that I can see my shared context across machines.

**Acceptance Criteria:**

**Given** el archivo `.env` existe con `NEON_DATABASE_URL`
**When** la app se inicia
**Then** el core Rust lee la URL del `.env` y establece conexión a Neon

**Given** la conexión a Neon es exitosa
**When** el comando `get_memories` se ejecuta con `{ projectId: "abc-123" }`
**Then** retorna las últimas 5 memorias del proyecto ordenadas por `created_at DESC`

**Given** la conexión a Neon falla (timeout, credenciales inválidas, sin red)
**When** se intenta obtener memorias
**Then** retorna error cacheable y el frontend muestra indicador "última sync: hace X min"
**And** reintentará automáticamente en 30 segundos (máximo 3 reintentos)

**Given** no hay memorias para un proyecto
**When** se ejecuta `get_memories`
**Then** retorna lista vacía sin error

**Given** el comando `get_project_status` se ejecuta con `{ projectId: "abc-123" }`
**Then** retorna `{ lastSummary, lastAgent, lastActivityAt }` de la última memoria del proyecto

## Epic 2: Acciones Locales — Abrir Herramientas

**Objetivo:** El usuario puede abrir cualquier proyecto en su editor preferido o lanzar agentes desde un clic.

### Story 2.1: Abrir Editores (VSCode + Cursor)

As a user,
I want to open my project in VSCode or Cursor with one click,
So that I can start coding immediately.

**Acceptance Criteria:**

**Given** un proyecto con `env: "wsl"` y `preferredEditor: "vscode"`
**When** el comando Tauri `open_editor` se ejecuta con `{ projectId: "abc-123", editor: "vscode" }`
**Then** se ejecuta `wsl code /home/mario/dev/project` en la máquina local
**And** VSCode abre con el proyecto cargado

**Given** un proyecto con `env: "windows"` y `preferredEditor: "cursor"`
**When** el comando Tauri `open_editor` se ejecuta con `{ projectId: "abc-123", editor: "cursor" }`
**Then** se ejecuta `cursor .` con el `cwd` del proyecto
**And** Cursor abre con el proyecto cargado

**Given** un proyecto con `env: "mac"` y `preferredEditor: "vscode"`
**When** el comando Tauri `open_editor` se ejecuta con `{ projectId: "abc-123", editor: "vscode" }`
**Then** se ejecuta `code .` con el `cwd` del proyecto
**And** VSCode abre con el proyecto cargado

**Given** el editor no está instalado en PATH
**When** se intenta ejecutar `open_editor`
**Then** retorna `Err("VSCode no encontrado en PATH. Instalalo primero.")`
**And** el frontend muestra un toast con el error

**Given** la ruta del proyecto no existe en el filesystem
**When** se intenta ejecutar `open_editor`
**Then** retorna `Err("Ruta no encontrada: /path/to/project")`
**And** el frontend muestra un toast con el error

### Story 2.2: Lanzar Agentes (Claude Code + OpenCode + QwenCode)

As a user,
I want to launch Claude Code, OpenCode, or QwenCode in an external terminal,
So that I can start working with my preferred AI agent.

**Acceptance Criteria:**

**Given** un proyecto con `env: "wsl"`
**When** el comando Tauri `launch_agent` se ejecuta con `{ projectId: "abc-123", agent: "claude" }`
**Then** se abre un terminal externo ejecutando `wsl claude` en el directorio del proyecto

**Given** un proyecto con `env: "windows"`
**When** el comando Tauri `launch_agent` se ejecuta con `{ projectId: "abc-123", agent: "opencode" }`
**Then** se abre un terminal externo ejecutando `opencode` en el directorio del proyecto

**Given** un proyecto con `env: "mac"`
**When** el comando Tauri `launch_agent` se ejecuta con `{ projectId: "abc-123", agent: "qwen" }`
**Then** se abre un terminal externo ejecutando `qwen` en el directorio del proyecto

**Given** el agente no está instalado en PATH
**When** se intenta ejecutar `launch_agent`
**Then** retorna `Err("Claude Code no encontrado en PATH. Instalalo primero.")`
**And** el frontend muestra un toast con el error

**Given** el proyecto no tiene un agente configurado como `defaultAgent`
**When** el usuario hace clic en un botón de agente
**Then** igual se ejecuta el comando del agente seleccionado (el botón es explícito, no depende de `defaultAgent`)

### Story 2.3: Frontend — Botones de Acción en ProjectCard

As a user,
I want to see action buttons on each project card,
So that I can launch editors or agents with one click.

**Acceptance Criteria:**

**Given** hay proyectos en el dashboard
**When** se renderiza un ProjectCard
**Then** muestra 6 botones: Continue (primary), VSCode, Cursor, Claude Code, OpenCode, QwenCode
**And** el botón Continue tiene estilo filled primary
**And** los otros 5 botones tienen estilo secondary (ghost border)

**Given** el usuario hace clic en "VSCode" o "Cursor"
**When** se ejecuta `invoke('open_editor', { projectId, editor })`
**Then** el editor se abre (si existe) o muestra toast de error

**Given** el usuario hace clic en "Claude Code", "OpenCode" o "QwenCode"
**When** se ejecuta `invoke('launch_agent', { projectId, agent })`
**Then** el agente se abre en terminal externo (si existe) o muestra toast de error

**Given** el usuario hace clic en "Continue"
**When** se abre el AgentSelector modal
**Then** muestra 3 opciones: Claude Code, OpenCode, QwenCode
**And** al seleccionar uno, ejecuta `launch_agent` (el flujo completo de contexto se implementa en Epic 3)

## Epic 3: Polish Visual — The Command Matrix

**Objetivo:** La app tiene el look & feel "Command Matrix": dark, densa, profesional.

### Story 3.1: Design Tokens + Layout Base

As a user,
I want a consistent, high-density dark UI,
So that the app feels like a professional command center.

**Acceptance Criteria:**

**Given** la app se renderiza
**When** se aplican los estilos globales
**Then** el background principal es `#101419`
**And** las tarjetas usan `surface-container-low`
**And** las zonas interactivas usan `surface-container-highest` en hover
**And** no hay bordes sólidos de 1px para separar secciones (No-Line Rule)
**And** el padding máximo es 12-16px, radius 0.25rem

**Given** hay texto de encabezado
**When** se renderiza
**Then** usa Space Grotesk
**And** el texto de UI usa Inter
**And** los inputs y datos técnicos usan JetBrains Mono

**Given** hay un contenedor que necesita perímetro
**When** se aplica "Ghost Border"
**Then** usa `outline-variant` (#424754) al 15% de opacidad

### Story 3.2: Status Indicators + Detalles Visuales

As a user,
I want visual polish on status indicators and interactive elements,
So that the app feels alive and professional.

**Acceptance Criteria:**

**Given** un indicador de estado (env badge)
**When** se renderiza
**Then** es un "Glow Tag": dot de 6px del color de status (`#00e475`, `#ffb95f`, `#ef4444`) + texto `label-sm`
**And** aplica "Subtle Glow": `surface_tint` al 10% con blur de 12px detrás del dot

**Given** un botón primario (Continue)
**When** se renderiza
**Then** tiene fondo `primary_container`, texto `on_primary_container`
**And** sin rounded corners (sm o none)

**Given** un botón secundario (VSCode, Cursor, agentes)
**When** se renderiza
**Then** fondo transparente con "Ghost Border" en `outline-variant` al 15%

**Given** un input de formulario está en focus
**When** el usuario hace clic en el input
**Then** el "Ghost Border" cambia a 80% de opacidad con color primary (#00e475)
**And** no hay anillos gruesos de focus

## Epic 4: Mejoras de Usabilidad

**Objetivo:** El usuario puede buscar, filtrar, y ver sus proyectos ordenados por uso reciente.

### Story 4.1: Tracking de `lastOpenedAt`

As a user,
I want each project to track when it was last opened,
So that I can see which projects I'm actively using.

**Acceptance Criteria:**

**Given** un proyecto existente
**When** el usuario abre un editor (VSCode/Cursor) o un agente (Claude/OpenCode/Qwen)
**Then** se actualiza `lastOpenedAt` del proyecto en `projects.json` con el timestamp actual
**And** el dashboard refleja el cambio inmediatamente

### Story 4.2: Ordenar proyectos por `lastOpenedAt`

As a user,
I want my most recently used projects to appear first,
So that I can find my active work without scrolling.

**Acceptance Criteria:**

**Given** hay más de un proyecto en el dashboard
**When** se renderiza el ProjectList
**Then** los proyectos se ordenan por `lastOpenedAt DESC` (más reciente primero)
**And** los proyectos sin `lastOpenedAt` aparecen al final

### Story 4.3: Search/Filter de proyectos

As a user,
I want to search projects by name, path, or tags,
So that I can find a specific project quickly when I have many.

**Acceptance Criteria:**

**Given** hay proyectos en el dashboard
**When** el usuario escribe en el campo de búsqueda
**Then** la lista se filtra en tiempo real por nombre, path, o tags
**And** se muestra el count de resultados ("3 of 8 projects")
**And** el campo de búsqueda tiene un botón "X" para limpiar el filtro
**And** cuando no hay resultados muestra "No projects match your search"

## Epic 5: Historial de Sesiones

**Objetivo:** El usuario puede ver y reabrir sesiones existentes de los agentes (Claude, Qwen, OpenCode) usando el storage real que cada agente exponga localmente.

> **Nota:** Claude Code y QwenCode exponen sesiones en archivos locales conocidos. OpenCode queda como integracion futura via adapter, sin asumir hoy si su storage real es archivo, base de datos u otro formato. Este epic lee sesiones existentes; no crea un log propio.

### Story 5.1: Detectar sesiones de agentes en el directorio del proyecto

As a user,
I want the app to discover existing agent sessions in my project directory,
So that I can see my conversation history without manual configuration.

**Acceptance Criteria:**

**Given** un proyecto con sesiones de Claude Code en `.claude/PROJECT_CLAUDE.md` o `.claude/CLAUDE.md`
**When** el comando `get_sessions` se ejecuta con `{ projectId }`
**Then** retorna una lista de sesiones detectadas con: `{ agent, sessionName, modifiedAt, size }`

**Given** un proyecto con sesiones de QwenCode en `.qwen/` o directorio similar
**When** el comando `get_sessions` se ejecuta
**Then** retorna las sesiones de QwenCode detectadas

**Given** un proyecto sin sesiones de ningún agente
**When** el comando `get_sessions` se ejecuta
**Then** retorna una lista vacía `[]` sin error

### Story 5.2: Mostrar lista de sesiones expandible en ProjectCard

**Estado:** Done

As a user,
I want to see agent sessions grouped by agent in an expandable section of each project card,
So that I can understand my work patterns across different agents.

**Acceptance Criteria:**

**Given** un proyecto con sesiones detectadas
**When** el usuario expande la sección "Sessions" en el ProjectCard
**Then** muestra las sesiones agrupadas por agente (Claude, Qwen, OpenCode)
**And** cada sesión muestra: nombre, fecha relativa ("hace 2 horas"), tamaño
**And** las sesiones se ordenan por fecha modificada (más reciente primero)
**And** si no hay sesiones muestra "No agent sessions found"

**Given** un ecosistema configurado con `rootPath`
**When** el usuario expande la sección de sesiones del ecosistema
**Then** puede ver el historial detectado en la carpeta root del ecosistema
**And** puede reabrir sesiones desde ese contexto cross-project

### Story 5.3: Reabrir sesión de agente

As a user,
I want to reopen a specific agent session from the history,
So that I can continue working in that conversation context.

**Acceptance Criteria:**

**Given** un proyecto con sesiones de Claude Code
**When** el usuario hace clic en una sesión de Claude
**Then** se abre Claude Code en el directorio del proyecto
**And** (si el agente soporta resume) se pasa el nombre de sesión para reabrir esa conversación

**Given** un proyecto con sesiones de QwenCode
**When** el usuario hace clic en una sesión de QwenCode
**Then** se abre QwenCode en el directorio del proyecto

### Story 5.4: Titulos legibles para sesiones

**Estado:** Done

As a user,
I want each detected session to show a human-readable title instead of an opaque id,
So that I can understand what was done in that session before reopening it.

**Acceptance Criteria:**

**Given** una sesion detectada de Claude o Qwen
**When** el backend enriquece la metadata de la sesion
**Then** retorna un `title` legible derivado del contenido inicial real de la conversacion
**And** no expone el `sessionId` como label principal en la UI

**Given** un archivo de sesion grande
**When** el sistema calcula el titulo
**Then** lee solo un fragmento acotado del contenido necesario para extraer el primer prompt o texto util
**And** no necesita cargar el archivo completo en memoria

**Given** que no se puede derivar un titulo util desde el contenido
**When** el backend normaliza la sesion
**Then** retorna un fallback legible con agente + fecha relativa o equivalente

### Story 5.5: Orden cronologico correcto y metadata consistente

**Estado:** Done

As a user,
I want sessions to appear from newest to oldest with reliable timestamps and metadata,
So that the history reflects my actual recent work.

**Acceptance Criteria:**

**Given** sesiones detectadas de uno o varios agentes
**When** se muestran en la UI
**Then** aparecen ordenadas de mas nueva a mas antigua segun `modifiedAt`
**And** el orden no depende del orden natural del filesystem

**Given** sesiones detectadas en escenarios WSL
**When** el backend obtiene la metadata
**Then** resuelve `modifiedAt` y `sizeBytes` reales o una aproximacion consistente basada en el storage real
**And** evita usar timestamps artificiales como reemplazo silencioso del valor real

**Given** una sesion listada en la UI
**When** se renderiza
**Then** muestra claramente a que agente pertenece cada item
**And** el usuario puede distinguir Claude, Qwen y futuros adapters sin depender del grouping actual

### Story 5.6: Adapter generico para sesiones de OpenCode

**Estado:** Done

As a user,
I want OpenCode sessions to be supported through a dedicated adapter,
So that the app can list and reopen them without asumir un formato de storage incorrecto.

**Acceptance Criteria:**

**Given** que OpenCode puede persistir sesiones en un formato distinto a archivos `json` o `jsonl`
**When** se planifica su integracion
**Then** el sistema define un adapter especifico para discovery, lectura de metadata y resume
**And** el contrato comun de sesiones se mantiene estable para la UI

**Given** que OpenCode persiste sesiones en `opencode.db`
**When** el sistema detecta sesiones de OpenCode
**Then** usa un adapter SQLite read-only para mapear proyecto, sesion, titulo y timestamps
**And** mantiene el mismo contrato comun de sesiones que usa la UI

---

## ⏸️ Epics Bloqueados / Diferidos

> **Nota:** Estos epics se retoman cuando el sistema de memorias en Neon esté listo.

### ~~Epic: Continue Work (Core Feature)~~

**Objetivo original:** El usuario puede retomar su trabajo donde lo dejó, con contexto inyectado en el agente.
**FRs:** FR12-16
**Estado:** 🟡 **Blocked** — depende de datos en Neon
**Stories originales:** AgentSelector Modal, Construir Contexto desde Neon, Lanzar Agente con Contexto

### ~~Epic: Dashboard Completo con Estado~~

**Objetivo original:** El usuario ve el estado de todos sus proyectos de un vistazo: última actividad, agente, timestamp.
**FRs:** FR17
**Estado:** 🟡 **Blocked** — depende de datos en Neon
**Stories originales:** Status Cards con Datos de Neon, StatusBar Global

---

## 🆕 Epics Nuevos Incorporados al Producto

## Epic 6: Quick Wins

**Objetivo:** Mejoras rápidas de alta utilidad diaria con bajo esfuerzo de implementación.

### Story 6.1: Botón de terminal por proyecto

As a user,
I want to open a terminal in my project directory with one click,
So that I can run ad-hoc commands without manual navigation.

**Acceptance Criteria:**

**Given** un proyecto con `env: "wsl"`
**When** el usuario hace clic en "Terminal WSL"
**Then** se abre un terminal de Windows Terminal ejecutando `wsl bash -ic "cd '/path'"`
**And** el terminal queda en el directorio del proyecto listo para usar

**Given** un proyecto con `env: "windows"`
**When** el usuario hace clic en "Terminal PowerShell"
**Then** se abre un terminal de Windows Terminal ejecutando `pwsh -NoExit -Command "cd 'D:\path'"`

### Story 6.2: Botón de terminal general

As a user,
I want to quickly open a WSL or PowerShell terminal from the app,
So that I can run commands without switching contexts manually.

**Acceptance Criteria:**

**Given** la app abierta
**When** el usuario hace clic en "Terminal WSL" en la barra superior
**Then** se abre Windows Terminal en WSL en el home directory (`~`)

**Given** la app abierta
**When** el usuario hace clic en "Terminal PS" en la barra superior
**Then** se abre Windows Terminal en PowerShell en el home directory de Windows

### Story 6.3: Abrir settings del agente

As a user,
I want to open the agent's settings file directly from the project card,
So that I can quickly edit Claude/Qwen/OpenCode configurations.

**Acceptance Criteria:**

**Given** un proyecto con `env: "wsl"`
**When** el usuario hace clic en "Settings Claude"
**Then** se abre VS Code en WSL con `~/.claude/settings.json`

**Given** un proyecto con `env: "wsl"`
**When** el usuario hace clic en "Settings Qwen"
**Then** se abre VS Code en WSL con `~/.qwen/settings.json`

**Given** un proyecto con sesiones de agentes
**When** el usuario expande las sesiones
**Then** se muestra un botón/link para abrir el settings del agente correspondiente

### Story 6.4: File picker para Add Project

As a user,
I want to select a folder using a native file dialog instead of typing the path,
So that I can add projects faster and avoid typos.

**Acceptance Criteria:**

**Given** el usuario hace clic en "+ Add Project"
**When** se abre el dialog
**Then** muestra un campo de ruta editable Y un botón "Browse" con ícono de carpeta

**Given** el usuario hace clic en "Browse"
**When** se abre el file picker nativo de Windows (o WSL)
**Then** al seleccionar una carpeta, el campo de ruta se auto-completa con la ruta seleccionada

### Story 6.5: Crear proyecto desde cero

As a user,
I want to create a new project directory and register it simultaneously,
So that I don't have to manually create folders before adding projects.

**Acceptance Criteria:**

**Given** el dialog de Add Project
**When** el usuario hace clic en "Create New Project"
**Then** se abre un dialog para seleccionar la ubicación padre y el nombre del proyecto
**And** se crea la carpeta en el filesystem
**And** se registra automáticamente en `projects.json`

### Story 6.6: Rediseño de ProjectCard para herramientas escalables

**Estado:** Done

As a user,
I want the project card to separate Continue, IDE tools, and CLI tools,
So that the UI scales cleanly as new editors and agents are added.

**Acceptance Criteria:**

**Given** un ProjectCard renderizado
**When** el usuario lo visualiza
**Then** mantiene la informacion principal del proyecto tal como hoy
**And** conserva un CTA primario `Continue with {defaultAgent}`

**Given** la seccion de herramientas de edicion
**When** el usuario interactua con `IDE`
**Then** puede seleccionar entre los IDEs disponibles para ese proyecto desde un control compacto
**And** la card no crea un boton fijo adicional por cada IDE futuro

**Given** la seccion de herramientas agenticas
**When** el usuario interactua con `CLI`
**Then** puede seleccionar entre los CLIs disponibles para ese proyecto desde un control compacto
**And** la card puede crecer a futuros agentes como Codex CLI sin explotar en cantidad de botones

**Given** las acciones secundarias del proyecto
**When** se renderiza la card
**Then** permanecen visibles `Terminal`, `Notes`, `Edit` y `Delete`
**And** el layout final prioriza densidad, claridad y escalabilidad sobre una grilla fija de botones

---

## Epic 7: Bloc de Notas

**Objetivo:** El usuario puede tomar notas rápidas por proyecto y una nota general, accesible desde la app.

### Story 7.1: Persistencia de notas en Rust

As a user,
I want my notes to be saved locally and persist between app restarts,
So that I don't lose my thoughts.

**Acceptance Criteria:**

**Given** el sistema de notas
**When** el usuario guarda una nota para un proyecto
**Then** se guarda en `~/.dev-control-center/notes/<project-id>.md`

**Given** el sistema de notas generales
**When** el usuario guarda una nota general
**Then** se guarda en `~/.dev-control-center/notes/_general.md`

**Given** notas existentes
**When** la app se abre
**Then** las notas se cargan automáticamente desde el filesystem

### Story 7.2: UI del bloc de notas por proyecto

As a user,
I want to access a notepad for each project from the project card,
So that I can jot down quick thoughts about that project.

**Acceptance Criteria:**

**Given** un ProjectCard
**When** el usuario expande el card o hace clic en "Notes"
**Then** se abre un panel lateral o modal con un editor de texto simple

**Given** el editor de notas abierto
**When** el usuario escribe y presiona Ctrl+S o hace clic en "Save"
**Then** la nota se guarda y se muestra "Saved" brevemente

**Given** una nota vacía
**When** el usuario abre el bloc de notas
**Then** muestra un placeholder "No notes yet — start typing..."

### Story 7.3: UI del bloc de notas general

As a user,
I want a general-purpose notepad accessible from the app's top bar,
So that I can capture ideas that aren't tied to a specific project.

**Acceptance Criteria:**

**Given** la barra superior
**When** el usuario hace clic en "Notes" o un ícono de bloc
**Then** se abre un panel/modal con la nota general (`_general.md`)

**Given** el editor general abierto
**When** el usuario guarda
**Then** se guarda en `~/.dev-control-center/notes/_general.md`

---

## Epic 8: Ecosistemas (Padre/Hijos)

**Objetivo:** El usuario puede registrar ecosistemas como carpetas root reales, importar sus proyectos hijos, alternar entre vista plana y vista por ecosistemas, y abrir una sola sesion de agente en la carpeta root del ecosistema para trabajar a nivel cross-project.

### Story 8.1: Entidad de ecosistema y relacion con proyectos

As a user,
I want ecosystems to have their own configuration,
So that root path, agent defaults, and grouping do not depend on arbitrary projects.

**Acceptance Criteria:**

**Given** la persistencia local de la app
**When** un ecosistema existe
**Then** se guarda como entidad propia con al menos `id`, `name`, `rootPath`, `defaultAgent` y `environment`

**Given** un proyecto registrado
**When** pertenece a un ecosistema
**Then** referencia al ecosistema por `ecosystemId` o equivalente estable, no por datos derivados del proyecto

**Given** que estamos en fase de testing
**When** se introduce el nuevo modelo
**Then** no es obligatorio mantener compatibilidad con el modelo provisional anterior

### Story 8.2: Toggle de vista por ecosistemas

As a user,
I want to toggle between a flat list and an ecosystem-grouped view,
So that I can see projects organized or uncluttered depending on my needs.

**Acceptance Criteria:**

**Given** la barra superior
**When** el usuario hace clic en un toggle "By Ecosystem" / "Flat"
**Then** la vista cambia entre:
- **Flat:** lista plana de todos los proyectos (comportamiento actual)
- **By Ecosystem:** secciones colapsables por ecosistema

**Given** la vista por ecosistemas
**When** el usuario expande un ecosistema
**Then** muestra los proyectos de ese ecosistema como cards normales

**Given** la vista por ecosistemas
**When** un proyecto no tiene ecosistema
**Then** aparece en una sección "Ungrouped"

**Given** la vista por ecosistemas
**When** la app agrupa proyectos
**Then** usa la entidad `Ecosystem` como fuente de verdad, no campos derivados desde `projects.json`

### Story 8.3: Abrir ecosistema completo en agente

As a user,
I want to launch an agent at the ecosystem root directory,
So that I can work on cross-project implementations.

**Acceptance Criteria:**

**Given** un ecosistema expandido en la vista por ecosistemas
**When** el usuario hace clic en "Open All" o similar
**Then** se abre una sola sesion del agente en `rootPath` del ecosistema

**Given** el usuario define un ecosistema
**When** configura el ecosistema
**Then** puede definir su agente por defecto para `Open All`

**Given** un ecosistema configurado
**When** el usuario usa `Open All`
**Then** no se abre una sesion por cada proyecto hijo

### Story 8.4: Crear ecosistema desde carpeta e importar proyectos hijos

As a user,
I want to register an ecosystem folder and import its child projects,
So that I can onboard a whole workspace quickly.

**Acceptance Criteria:**

**Given** el flujo de alta
**When** el usuario elige "Add Ecosystem Folder"
**Then** puede introducir `name`, `rootPath`, `environment` y `defaultAgent`

**Given** una carpeta root valida
**When** la app la escanea
**Then** detecta las subcarpetas directas candidatas a proyecto

**Given** el resultado del escaneo
**When** el usuario confirma la importacion
**Then** puede seleccionar que subcarpetas se registran como proyectos hijos del ecosistema

**Given** una subcarpeta ya registrada como proyecto
**When** forma parte del escaneo
**Then** la app evita duplicados y muestra el conflicto de forma clara

### Story 8.5: Gestion manual de ecosistemas

As a user,
I want to create, edit, and delete ecosystems manually,
So that I can manage grouping and ecosystem metadata even without reimporting folders.

**Acceptance Criteria:**

**Given** la barra superior o un menú de gestión
**When** el usuario hace clic en "Manage Ecosystems"
**Then** se abre un dialog con la lista de ecosistemas existentes y sus proyectos

**Given** el dialog de ecosistemas
**When** el usuario edita un ecosistema
**Then** puede cambiar `name`, `rootPath` y `defaultAgent`

**Given** el dialog de ecosistemas
**When** el usuario asigna proyectos existentes a un ecosistema
**Then** puede hacerlo con checkboxes o una interacción equivalente simple

**Given** el dialog de ecosistemas
**When** el usuario elimina un ecosistema
**Then** los proyectos quedan como "Ungrouped" y no se eliminan
