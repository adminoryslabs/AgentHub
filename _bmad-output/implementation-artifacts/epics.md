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

**Objetivo:** El usuario puede ver y reabrir sesiones existentes de los agentes (Claude, Qwen, OpenCode) que ya están guardadas en el directorio del proyecto.

> **Nota:** Los agentes como Claude Code y QwenCode guardan sus propias sesiones de conversación en el directorio del proyecto (ej: `.claude/`, `.qwen/`). Este epic lee esas sesiones existentes, no crea un log propio.

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
