---
story_key: 7-1-persistencia-de-notas-en-rust
status: done
epic: 7
epic_title: Bloc de Notas
---

# Story 7.1: Persistencia de notas en Rust

## Story

As a user,
I want my notes to be saved locally and persist between app restarts,
So that I don't lose my thoughts.

## Acceptance Criteria

**Given** el sistema de notas
**When** el usuario guarda una nota para un proyecto
**Then** se guarda en `~/.dev-control-center/notes/<project-id>.md`

**Given** el sistema de notas generales
**When** el usuario guarda una nota general
**Then** se guarda en `~/.dev-control-center/notes/_general.md`

**Given** notas existentes
**When** la app se abre
**Then** las notas se cargan automáticamente desde el filesystem

## Tasks/Subtasks

- [x] Task 1: Crear capa de persistencia de notas local
  - [x] Subtask 1.1: Directorio `~/.dev-control-center/notes/`
  - [x] Subtask 1.2: Path `<project-id>.md` para notas por proyecto
  - [x] Subtask 1.3: Path `_general.md` para nota general
- [x] Task 2: Exponer commands Tauri para lectura/escritura
  - [x] Subtask 2.1: `get_project_note(projectId)`
  - [x] Subtask 2.2: `save_project_note(projectId, content)`
  - [x] Subtask 2.3: `get_general_note()`
  - [x] Subtask 2.4: `save_general_note(content)`
- [x] Task 3: Validar integridad basica
  - [x] Subtask 3.1: Verificar proyecto existente antes de guardar/cargar nota por proyecto
  - [x] Subtask 3.2: Si el archivo no existe, devolver string vacio
- [x] Task 4: Preparar superficie para frontend futuro
  - [x] Subtask 4.1: Wrappers en `src/lib/invoke.ts`

## Dev Notes

### Alcance real de esta story
- Se implementa persistencia backend y API IPC lista para consumo desde React.
- La UI del bloc de notas queda para stories 7.2 y 7.3.
- El criterio "las notas se cargan automáticamente" se satisface a nivel de API disponible al iniciar la app; la carga visual quedará conectada cuando exista la UI.

### Paths
- Notas por proyecto: `~/.dev-control-center/notes/<project-id>.md`
- Nota general: `~/.dev-control-center/notes/_general.md`

## File List

| File | Action |
|------|--------|
| `src-tauri/src/commands/notes.rs` | Created (persistencia de notas y commands Tauri) |
| `src-tauri/src/commands/projects.rs` | Modified (expone helper de directorio base) |
| `src-tauri/src/lib.rs` | Modified (registro de commands de notas) |
| `src/lib/invoke.ts` | Modified (wrappers para notas) |

## Change Log

- Nuevo modulo `notes.rs` para notas locales en markdown
- Commands para guardar/cargar notas por proyecto y nota general
- Wrappers TypeScript listos para conectar en stories de UI

## Dev Agent Record

### Implementation Plan
1. Reutilizar el directorio base `~/.dev-control-center`
2. Crear helper de notes path y operaciones read/write
3. Exponer commands Tauri especificos para proyecto/general
4. Registrar commands y wrappers TS

### Completion Notes
- Se devolvio string vacio cuando el archivo de nota aun no existe para simplificar la UI futura.
- Las notas por proyecto validan que el proyecto exista en `projects.json` antes de operar.

### Debug Log
- `cargo check` OK
- `npm run build` OK
