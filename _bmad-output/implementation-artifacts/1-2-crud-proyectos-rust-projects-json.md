---
story_key: 1-2-crud-proyectos-rust-projects-json
status: review
epic: 1
epic_title: Setup & Gestión de Proyectos
---

# Story 1.2: CRUD Proyectos en Rust (projects.json)

## Story

As a user,
I want to create, edit, and delete projects stored locally,
So that I can manage my development projects.

## Acceptance Criteria

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

## Tasks/Subtasks

- [x] Task 1: Crear modelo Project en Rust
  - [x] Subtask 1.1: `src-tauri/src/models/project.rs` con struct `Project`
  - [x] Subtask 1.2: Campos: id, name, path, env, preferredEditor, defaultAgent, tags, lastOpenedAt, createdAt
  - [x] Subtask 1.3: Serialize, Deserialize, Clone derivados
  - [x] Subtask 1.4: `ProjectsStore` con `projects: Vec<Project>`
- [x] Task 2: Implementar capa de persistencia (projects.json)
  - [x] Subtask 2.1: `get_projects_path()` → `~/.dev-control-center/projects.json`
  - [x] Subtask 2.2: `ensure_projects_dir()` → crea directorio si no existe
  - [x] Subtask 2.3: `load_projects()` → lee y parsea
  - [x] Subtask 2.4: `save_projects()` → serializa y escribe
  - [x] Subtask 2.5: Archivo inexistente → lista vacía
- [x] Task 3: Implementar Tauri commands de proyectos
  - [x] Subtask 3.1: `src-tauri/src/commands/projects.rs` creado
  - [x] Subtask 3.2: `get_projects()` → `Result<Vec<Project>, String>`
  - [x] Subtask 3.3: `create_project()` con validación de `name`
  - [x] Subtask 3.4: `update_project()` con búsqueda por id
  - [x] Subtask 3.5: `delete_project()` con búsqueda por id
- [x] Task 4: Registrar commands en main.rs/lib.rs
  - [x] Subtask 4.1: Módulos importados en `lib.rs`
  - [x] Subtask 4.2: 4 commands registrados con `tauri::generate_handler!`
- [x] Task 5: Verificar compilación
  - [x] Subtask 5.1: `cargo check` pasa sin errores
  - [x] Subtask 5.2: `npm run tauri build` compila exitosamente (.deb + .rpm)

## Dev Notes

### Arquitectura
- Commands: `src-tauri/src/commands/projects.rs`
- Modelo: `src-tauri/src/models/project.rs`
- Persistencia inline en commands (sin repo separado para MVP)

### Modelo Project
```rust
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub environment: String,
    pub preferred_editor: String,
    pub default_agent: String,
    pub tags: Vec<String>,
    pub last_opened_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
```

### JSON
- Frontend envía `camelCase`, Rust usa `snake_case` con `#[serde(rename_all = "camelCase")]`
- El campo `env` del JSON se mapea a `environment` en Rust con `#[serde(rename = "env")]`

### Borrow checker fix
- `update_project` clona el proyecto antes de `save_projects` para evitar borrow conflict

## File List

| File | Action |
|------|--------|
| `src-tauri/src/models/project.rs` | Created (Project + ProjectsStore structs) |
| `src-tauri/src/commands/projects.rs` | Created (4 Tauri commands + persistence) |
| `src-tauri/src/lib.rs` | Modified (added modules + invoke_handler) |
| `src-tauri/Cargo.toml` | Unchanged (deps from 1.1 sufficient) |

## Change Log

- Modelo Project con UUID, timestamps, y campos del PRD
- Persistencia en `~/.dev-control-center/projects.json` con auto-creación de directorio
- 4 Tauri commands: get_projects, create_project, update_project, delete_project
- Validación de `name` requerido
- Build exitoso: .deb + .rpm generados

## Dev Agent Record

### Implementation Plan
1. Crear modelo Project con serde
2. Implementar load/save de projects.json
3. Crear 4 commands Tauri
4. Registrar en lib.rs
5. Verificar build

### Completion Notes
- Borrow checker requirió clonar project antes de save en update_project
- `get_projects_dir` usa `$HOME` o `$USERPROFILE` para cross-platform
- Todos los commands son async pero sin Mutex (single user, no race conditions)

### Debug Log
- `cargo check` falló con E0502 (borrow conflict) → fix: clone antes de save
- Warning de imports no usados (chrono, State, Mutex) → eliminados
