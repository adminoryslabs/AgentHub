---
stepsCompleted: [1, 2, 3, 4, 5, 6]
inputDocuments:
  - '/home/marioyahuar/AgentHub/PRD.md'
  - '/home/marioyahuar/AgentHub/DESIGN.md'
  - '/home/marioyahuar/AgentHub/code.html'
  - '/home/marioyahuar/AgentHub/_bmad-output/planning-artifacts/prd-validation-report.md'
workflowType: 'architecture'
project_name: 'Dev Control Center'
user_name: 'Mario'
date: '2026-04-10'
validationStatus: 'COMPLETED'
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

## Project Context Analysis

### Requirements Overview

**Requisitos Funcionales:**

1. **CRUD manual de proyectos** — Crear, editar, eliminar proyectos con metadata (nombre, ruta, entorno WSL/Windows/Mac, editor preferido, agente por defecto, tags)
2. **Acciones rápidas por proyecto** — 5 botones concretos: VSCode, Cursor, Claude Code, OpenCode, QwenCode. Cada uno ejecuta un comando local del SO
3. **Continue Work (CORE)** — Leer memorias de Neon → construir contexto → lanzar agente seleccionado con contexto inyectado en terminal externo
4. **Lectura de memorias Neon** — Conexión directa a PostgreSQL para obtener última actividad, agente usado, resumen y timestamp por proyecto
5. **Visualización de estado** — Status cards con última actividad, agente, timestamp
6. **Soporte multi-entorno** — Windows, WSL, Mac con comandos adaptativos por SO

**Requisitos No Funcionales:**

- App de escritorio multiplataforma (Windows, WSL, Mac)
- Sin backend intermedio — conexión directa a Neon desde cada máquina
- Terminal externo — los agentes se abren en terminales del SO, no embebidos
- Persistencia local de proyectos en `projects.json`
- Auth mínima (token simple, opcional en MVP)
- Credenciales de Neon en `.env` local

### Scale & Complexity

- **Dominio técnico:** Aplicación de escritorio full-stack local (Tauri + React + PostgreSQL directo)
- **Nivel de complejidad:** Media (2 usuarios, 3 entornos, conexión cloud a Neon, `child_process` adaptativo)
- **Componentes arquitectónicos estimados:** 3 (Frontend React, Core Rust/Tauri, Neon PostgreSQL)

### Technical Constraints & Dependencies

| Restricción | Detalle |
|-------------|---------|
| **Tauri v2** | Framework de app de escritorio — React como frontend, Rust como core |
| **IPC directo** | Comunicación frontend-core sin HTTP — llamadas de función nativas |
| **Neon PostgreSQL** | Conexión directa desde cada máquina. Read-only en MVP, extensible a write |
| **3 entornos de SO** | Windows (nativo), WSL (con prefijo `wsl`), Mac (nativo) — comandos adaptativos |
| **Mapeo de rutas** | `wslpath -w` para convertir rutas WSL → Windows al abrir editores |
| **Sin terminal embebido** | `child_process.exec()` para abrir terminales externos del SO |
| **Sin VPS/backend** | Neon actúa como capa de sincronización cloud |

### Cross-Cutting Concerns Identified

1. **Capa de abstracción por entorno** — Cada comando (editor, agente, terminal) debe adaptarse al `env` del proyecto (Windows/WSL/Mac)
2. **Gestión de credenciales Neon** — `.env` local con URL de conexión, permisos limitados (solo lectura en MVP)
3. **Latencia de queries a Neon** — El dashboard depende de Neon para mostrar estado; queries deben ser eficientes
4. **Compilación cross-platform** — Tauri genera binarios para Windows, Mac, Linux desde un mismo codebase
5. **Extensibilidad a write** — Arquitectura debe permitir INSERT/UPDATE de memorias en el futuro sin reescribir la capa de datos

## Starter Template Evaluation

### Primary Technology Domain

**Aplicación de escritorio** (Tauri v2 + React) basado en los requisitos de multi-plataforma (Windows, WSL, Mac), ejecución de comandos locales, y conexión directa a Neon PostgreSQL.

### Starter Options Considered

| Opción | Descripción | Estado |
|--------|-------------|--------|
| `create-tauri-app` (oficial) | CLI interactivo del equipo Tauri — seleccionás framework, lenguaje, package manager | ✅ Seleccionado |
| `tauri-react-template` | React + Vite + TS + React Router + window state utils | ❌ Menos mantenido |
| Custom (Vite + React + Tailwind + `tauri init`) | Armar todo desde cero | ❌ Más trabajo innecesario |

### Selected Starter: `create-tauri-app`

**Rationale:**
- Template oficial mantenido por el equipo Tauri — garantiza estructura Rust correcta, permisos de seguridad y pipeline de build cross-platform
- Ya se tiene diseño definido en DESIGN.md — no se necesita template UI pre-armado
- Base mínima y limpia a la que se le agrega Tailwind y dependencias según necesidad
- Soporte oficial para mobile (iOS/Android) si se desea en el futuro

**Initialization Command:**

```bash
npm create tauri-app@latest dev-control-center
# Selección interactiva:
#   Framework: React
#   Language: TypeScript
#   Package manager: npm
```

**Post-init setup:**

```bash
cd dev-control-center
npm install -D tailwindcss @tailwindcss/vite
npm install @tauri-apps/api
```

**Architectural Decisions Provided by Starter:**

**Language & Runtime:**
- TypeScript configurado en el frontend
- Rust (Cargo) en `src-tauri/` como core de la aplicación
- React 19 como framework UI

**Styling Solution:**
- CSS por defecto del starter; Tailwind CSS se agrega manualmente con `@tailwindcss/vite` (plugin oficial de Tailwind para Vite)

**Build Tooling:**
- Vite — dev server con HMR, production build optimizado
- Tauri CLI — `npm run tauri dev` compila Rust + inicia Vite + abre ventana desktop
- `npm run tauri build` — genera instaladores `.msi`, `.dmg`, `.deb`, `.AppImage`

**Testing Framework:**
- No incluido por defecto — se configura según necesidad (Vitest para frontend, `cargo test` para Rust)

**Code Organization:**
- `src/` — código React del frontend
- `src-tauri/` — código Rust (Tauri commands, configuración, Cargo.toml)
- `tauri.conf.json` — configuración de ventana, permisos, build pipeline

**Development Experience:**
- Hot reloading vía Vite
- DevTools del browser embebidos en la ventana Tauri
- `invoke()` para comunicación bidireccional React ↔ Rust

**Nota:** La inicialización con `create-tauri-app` debe ser la primera story de implementación.

## Core Architectural Decisions

### Decision Priority Analysis

**Decisiones Críticas (bloquean implementación):**
- State management del frontend
- Driver PostgreSQL en Rust
- Persistencia local de proyectos
- Patrón de Tauri Commands
- Permisos de Tauri

**Decisiones Importantes (dan forma a la arquitectura):**
- Manejo de errores de red (Neon)
- Protección de credenciales Neon
- Manejo de errores de `child_process`
- Component architecture

**Decisiones Diferidas (post-MVP):**
- CI/CD automatizado — 2 usuarios compilan manualmente
- Auth completa — token simple opcional en MVP
- Routing — single-view suficiente para MVP

### Data Architecture

**State Management del Frontend:**
- **Decisión:** React Context + Hooks (sin librería externa)
- **Rationale:** Scope reducido (lista de proyectos, estado de UI, datos de Neon). Zero dependencies. Migración trivial a Zustand si crece.
- **Afecta:** Todos los componentes del frontend

**Driver PostgreSQL en Rust:**
- **Decisión:** `sqlx` con feature `runtime-tokio`
- **Rationale:** Compile-time query checking previene errores de SQL. Async nativo. Buena documentación para Neon/PostgreSQL.
- **Afecta:** Capa de lectura de memorias en `src-tauri/`

**Persistencia Local de Proyectos:**
- **Decisión:** `projects.json` (archivo JSON plano)
- **Rationale:** Simple, editable a mano, sin dependencies adicionales. Suficiente para ~50 proyectos.
- **Afecta:** CRUD de proyectos en `src-tauri/`

**Manejo de Errores de Red (Neon):**
- **Decisión:** Cache local de última lectura + retry automático cada 30s
- **Rationale:** El dashboard siempre muestra algo (datos cacheados) con indicador visual "última sync: hace X min". Sin bloqueos por caída de red.
- **Afecta:** StatusBar, ProjectCard (datos de estado)

### Authentication & Security

**Protección de Credenciales Neon:**
- **Decisión:** `.env` local leído desde Rust en runtime. La URL NUNCA pasa al frontend React.
- **Rationale:** Simple para 2 usuarios de confianza. `tauri-plugin-store` o `dotenv` en Rust para lectura.
- **Afecta:** Core Rust (conexión a Neon)

**Permisos de Tauri:**
- **Decisión:** Capacidad mínima — solo `shell:allow-execute` + `fs:read-write`
- **Rationale:** Principio de mínimo privilegio. La app solo necesita ejecutar comandos locales y leer/escribir `projects.json`.
- **Afecta:** `src-tauri/tauri.conf.json` (sección `capabilities`)

### API & Communication Patterns

**Patrón de Tauri Commands:**
- **Decisión:** Un command por acción, organizados en módulos Rust separados
- **Estructura:**
  - `src-tauri/src/commands/projects.rs` — `get_projects`, `create_project`, `update_project`, `delete_project`
  - `src-tauri/src/commands/neon.rs` — `get_memories`, `get_project_status`
  - `src-tauri/src/commands/actions.rs` — `open_editor`, `launch_agent`, `continue_work`
- **Rationale:** Convención natural de Tauri. Simple, testeable, claro.
- **Afecta:** Toda la comunicación frontend ↔ core

**Manejo de Errores de `child_process`:**
- **Decisión:** Validación previa con `which`/`where` + toast de error en frontend
- **Rationale:** Previene crashes. El usuario sabe exactamente qué falta (ej: "Claude Code no encontrado en PATH").
- **Afecta:** `launch_agent`, `open_editor`, `continue_work`

### Frontend Architecture

**State Management:**
- React Context + Hooks (ver Data Architecture)
- Contextos: `ProjectsContext` (lista de proyectos), `NeonContext` (estado de conexión), `UIContext` (modales, toasts)

**Routing:**
- **Decisión:** Single-view — sin router
- **Rationale:** Una sola pantalla (dashboard). Si se agregan settings o project detail en el futuro, se agrega `react-router`.
- **Afecta:** Estructura de `src/App.tsx`

**Component Architecture:**

| Componente | Responsabilidad |
|-----------|-----------------|
| `App` | Layout principal, providers de contexto |
| `ProjectList` | Grid de project cards |
| `ProjectCard` | Info de proyecto + 6 botones de acción + status |
| `AgentSelector` | Modal para seleccionar agente en "Continue Work" |
| `AddProjectDialog` | Formulario para crear/editar proyecto |
| `StatusBar` | Indicador de conexión a Neon + última sync |

### Infrastructure & Deployment

**Build Pipeline:**
- **Desarrollo:** `npm run tauri dev` — compila Rust, inicia Vite, abre ventana
- **Producción:** `npm run tauri build` — genera instalador nativo del SO
- **Cada usuario compila en su máquina.** Sin CI/CD para MVP.

**Configuración por Entorno:**
- **Decisión:** Un `.env` local con `NEON_DATABASE_URL`. Sin complejidad de `.env.development` vs `.env.production`.
- **Afecta:** Setup inicial de cada usuario

### Decision Impact Analysis

**Secuencia de Implementación (orden de decisiones):**

1. Crear proyecto con `create-tauri-app` + instalar Tailwind
2. Configurar permisos Tauri (`shell`, `fs:read-write`)
3. Implementar capa de persistencia local (`projects.json` CRUD en Rust)
4. Implementar Tauri commands de proyectos (`get_projects`, `create_project`, etc.)
5. Implementar frontend: `ProjectList` + `ProjectCard` + `AddProjectDialog`
6. Implementar capa de conexión a Neon (`sqlx` en Rust)
7. Implementar Tauri commands de memorias (`get_memories`, `get_project_status`)
8. Implementar `StatusBar` con cache + retry
9. Implementar `AgentSelector` + commands de acciones (`open_editor`, `launch_agent`)
10. Implementar `continue_work` (lee Neon → construye contexto → lanza agente)
11. Integración E2E + manejo de errores + toasts
12. Compilación cross-platform + polish visual (DESIGN.md)

**Dependencias Cruzadas:**

| Decisión | Depende de |
|----------|-----------|
| Conexión a Neon | Permisos Tauri configurados, `.env` creado |
| Continue Work | Neon read funcionando + launch_agent funcionando |
| Cache de Neon | `get_memories` implementado |
| Validación de PATH | `shell:allow-execute` habilitado |
| ProjectCard con status | `get_project_status` implementado |

## Implementation Patterns & Consistency Rules

### Puntos de Conflicto Identificados

**7 áreas** donde AI agents podrían tomar decisiones inconsistentes:

1. **Naming Rust vs React** — `snake_case` vs `camelCase` en commands, campos JSON, DB
2. **Estructura del proyecto** — ¿Componentes por feature o por tipo? ¿Tests co-locados o separados?
3. **Formato de respuestas** — ¿`Result<T, String>` nativo o wrapper `{ data, error }`?
4. **Manejo de errores** — ¿String plano u objeto estructurado?
5. **Loading states** — ¿Global o por operación?
6. **Eventos Tauri** — ¿`snake_case` o `PascalCase`?
7. **Validación** — ¿Cuándo y cómo validar datos entre frontend y core?

### Naming Patterns

| Convención | Regla | Ejemplo |
|-----------|-------|---------|
| Tauri commands (Rust) | `snake_case` | `get_projects`, `launch_agent`, `continue_work` |
| Campos JSON IPC | `camelCase` | `{ projectId, name, env, lastOpenedAt }` |
| DB tablas/columnas | `snake_case` | `projects`, `project_id`, `last_opened_at` |
| Componentes React | `PascalCase` | `ProjectCard`, `AgentSelector`, `AddProjectDialog` |
| Archivos React | `PascalCase.tsx` | `ProjectCard.tsx` |
| Archivos Rust commands | `snake_case.rs` | `projects.rs`, `neon.rs`, `actions.rs` |
| Funciones Rust | `snake_case` | `fn get_projects()`, `fn launch_agent()` |
| Custom hooks React | `use` + `camelCase` | `useProjects`, `useNeonStatus`, `useActions` |
| Índices de DB | `idx_tabla_columna` | `idx_memories_project_id`, `idx_memories_created_at` |

### Structure Patterns

**Frontend (`src/`):**

```
src/
├── components/          # Por feature — un archivo por componente
│   ├── ProjectList.tsx
│   ├── ProjectCard.tsx
│   ├── AgentSelector.tsx
│   ├── AddProjectDialog.tsx
│   └── StatusBar.tsx
├── contexts/            # React Contexts
│   ├── ProjectsContext.tsx
│   ├── NeonContext.tsx
│   └── UIContext.tsx
├── hooks/               # Custom hooks
│   ├── useProjects.ts
│   ├── useNeonStatus.ts
│   └── useActions.ts
├── lib/                 # Utilidades compartidas
│   └── invoke.ts        # Wrappers de invoke()
├── styles/
│   └── globals.css      # Tailwind directives
├── App.tsx
└── main.tsx
```

**Core Rust (`src-tauri/`):**

```
src-tauri/
├── src/
│   ├── commands/        # Un archivo por dominio
│   │   ├── projects.rs
│   │   ├── neon.rs
│   │   └── actions.rs
│   ├── models/          # Estructuras de datos compartidas
│   │   └── project.rs
│   ├── db.rs            # Conexión Neon (sqlx)
│   ├── config.rs        # Carga de .env
│   └── main.rs          # Entry point + registro de commands
├── Cargo.toml
├── tauri.conf.json
└── capabilities/
    └── default.json     # Permisos: shell + fs
```

**Tests:**
- **React:** Co-locados — `ProjectCard.test.tsx` junto al componente
- **Rust:** Al final de cada módulo — `mod tests { ... }`

### Format Patterns

**Respuestas de Tauri commands:**
- **Decisión:** `Result<T, String>` nativo de Rust (sin wrapper)
- **Éxito:** `Ok(data)` → React recibe los datos directamente
- **Error:** `Err("mensaje legible")` → React muestra toast

**Formato de datos en JSON:**

| Tipo | Regla | Ejemplo |
|------|-------|---------|
| Fechas | ISO 8601 | `"2026-04-10T19:12:06Z"` |
| Booleanos | `true`/`false` nativos | — |
| Null/Optional | `null` en JSON, `Option<T>` en Rust | `"lastOpenedAt": null` |
| IDs | `camelCase` en JSON, `snake_case` en DB | JSON: `projectId` → DB: `project_id` |
| Listas | Objeto con array: `{ projects: [...] }` | — |

**Modelo de proyecto (JSON → React):**

```json
{
  "id": "uuid",
  "name": "payments-api",
  "path": "/home/mario/dev/payments-api",
  "env": "wsl",
  "preferredEditor": "vscode",
  "defaultAgent": "qwencode",
  "tags": ["backend"],
  "lastOpenedAt": "2026-04-10T19:12:06Z"
}
```

### Communication Patterns

**Tauri invoke:**
```typescript
// Patrón estándar
const result = await invoke<T>('command_name', { arg1, arg2 });
```

**Eventos Tauri (si se usan en el futuro):**
- Naming: `snake_case` — `project_created`, `agent_launched`
- Payload: mismo formato JSON que las respuestas de commands

**State management (React):**
- Updates: inmutables (spread operator)
```typescript
setProjects([...prev, newProject]);
setProjects(prev.filter(p => p.id !== id));
```
- Organización: 3 contexts — `ProjectsContext`, `NeonContext`, `UIContext`

### Process Patterns

**Error Handling:**

| Capa | Patrón | Ejemplo |
|------|--------|---------|
| Rust (core) | `eprintln!("[ERROR] contexto: mensaje")` | `eprintln!("[ERROR] neon: connection refused")` |
| Tauri command | `Err("mensaje para el usuario")` | `Err("No se encontró Claude Code en PATH")` |
| React (UI) | Toast con mensaje del error | `toast.error(result)` |
| Validación | Pre-check en Rust, no en frontend | `which("code")` antes de ejecutar |

**Loading States:**
- Por operación, no global
- Nomenclatura: `isLoading` + recurso en camelCase
- Ejemplos: `isLoadingProjects`, `isLoadingMemories`, `isLaunchingAgent`

**Retry Neon:**
- Automático cada 30s, máximo 3 reintentos
- Si falla: muestra cache con indicador "última sync: hace X min"
- El usuario puede forzar retry manual

**Validación PATH:**
- Pre-check con `which` (Unix/WSL/Mac) o `where` (Windows) antes de cada `child_process`
- Si no existe: `Err("Herramienta no encontrada: <nombre>")`

### Enforcement Guidelines

**Todos los AI agents DEBEN:**
1. Usar `snake_case` para commands Rust, `camelCase` para campos JSON
2. Usar `PascalCase` para componentes y archivos React
3. Usar `Result<T, String>` para respuestas de Tauri commands
4. Co-locar tests junto a los componentes (React) o al final del módulo (Rust)
5. Loading states por operación, no global
6. Pre-validar PATH antes de ejecutar comandos
7. Fechas en ISO 8601
8. State updates inmutables en React

**Anti-patrones (NO hacer):**
- ❌ Wrapper `{ data, error }` en Tauri commands — usar `Result<T, String>`
- ❌ Loading state global para todo — uno por operación
- ❌ Separar componentes por tipo (`/types`, `/components`, `/views`) — organizar por feature
- ❌ `child_process` sin validación previa de PATH
- ❌ Credenciales de Neon expuestas al frontend React
- ❌ Mezclar `snake_case` y `camelCase` en el mismo archivo

## Project Structure Details

### Neon Database Schema — Tabla `memories`

```sql
CREATE TABLE memories (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content         TEXT,
    title           TEXT,
    content_fts     TSVECTOR,
    source_detail   TEXT,
    metadata        JSONB,
    people          TEXT[],
    status          VARCHAR,
    type            VARCHAR,
    source_app      VARCHAR,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

-- Índices para queries del dashboard
CREATE INDEX idx_memories_created_at ON memories(created_at DESC);
CREATE INDEX idx_memories_project_id ON memories((metadata->>'projectId'));
CREATE INDEX idx_memories_content_fts ON memories USING GIN(content_fts);
```

**Queries principales para el dashboard:**

```sql
-- Última memoria de un proyecto
SELECT id, title, content, metadata, status, created_at
FROM memories
WHERE metadata->>'projectId' = $1
ORDER BY created_at DESC
LIMIT 1;

-- Últimas N memorias de un proyecto
SELECT id, title, content, metadata, status, created_at
FROM memories
WHERE metadata->>'projectId' = $1
ORDER BY created_at DESC
LIMIT $2;

-- Último agente usado (del metadata JSONB)
SELECT metadata->>'agent' AS last_agent, created_at
FROM memories
WHERE metadata->>'projectId' = $1
ORDER BY created_at DESC
LIMIT 1;
```

**Mapeo Rust → DB (`src-tauri/src/models/memory.rs`):**

```rust
pub struct Memory {
    pub id: Uuid,
    pub title: Option<String>,
    pub content: Option<String>,
    pub metadata: serde_json::Value,  // JSONB
    pub status: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Para queries con sqlx — el campo projectId viene del JSONB
pub struct ProjectStatus {
    pub last_summary: Option<String>,
    pub last_agent: Option<String>,
    pub last_activity_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

### Persistencia Local — `projects.json`

**Ubicación:** `~/.dev-control-center/projects.json` (no en el repo del proyecto)

**Rationale:** El archivo es por usuario/máquina, no por proyecto de desarrollo. Si Mario compila la app en su PC de trabajo y en su casa, cada una tiene su propio `projects.json`.

**Estructura:**

```json
{
  "version": 1,
  "projects": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "payments-api",
      "path": "/home/mario/dev/payments-api",
      "env": "wsl",
      "preferredEditor": "vscode",
      "defaultAgent": "qwencode",
      "tags": ["backend"],
      "lastOpenedAt": "2026-04-10T19:12:06Z",
      "createdAt": "2026-04-01T10:00:00Z"
    }
  ]
}
```

**Acceso:** El core Rust lee/escribe este archivo en `~/.dev-control-center/projects.json`. El frontend React nunca accede directamente — siempre via Tauri commands.

**Config de Tauri v2 — Permisos (`src-tauri/capabilities/default.json`):**

```json
{
  "identifier": "default",
  "description": "Default capabilities for Dev Control Center",
  "local": true,
  "windows": ["main"],
  "permissions": [
    "shell:default",
    {
      "identifier": "shell:allow-execute",
      "allow": [
        { "name": "exec", "cmd": "code" },
        { "name": "exec", "cmd": "cursor" },
        { "name": "exec", "cmd": "claude" },
        { "name": "exec", "cmd": "opencode" },
        { "name": "exec", "cmd": "qwen" },
        { "name": "exec", "cmd": "wsl" },
        { "name": "exec", "cmd": "which" },
        { "name": "exec", "cmd": "where" },
        { "name": "exec", "cmd": "wt" },
        { "name": "exec", "cmd": "open" },
        { "name": "exec", "cmd": "bash" },
        { "name": "exec", "cmd": "cmd" }
      ]
    },
    "fs:allow-read-text-file",
    "fs:allow-write-text-file",
    {
      "identifier": "fs:scope",
      "allow": ["$HOME/.dev-control-center/**"]
    }
  ]
}
```

**Nota:** Los comandos en `shell:allow-execute` son una allowlist. Si se agrega un agente nuevo en el futuro, hay que agregarlo aquí.
