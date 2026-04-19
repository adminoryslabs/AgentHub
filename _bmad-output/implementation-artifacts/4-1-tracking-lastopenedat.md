---
story_key: 4-1-tracking-lastopenedat
status: done
epic: 4
epic_title: Mejoras de Usabilidad
---

# Story 4.1: Tracking de `lastOpenedAt`

## Story

As a user,
I want each project to track when it was last opened,
So that I can see which projects I'm actively using.

## Tasks/Subtasks

- [x] Task 1: Persistir `lastOpenedAt` en el modelo de proyecto
  - [x] Subtask 1.1: Campo `last_opened_at` presente en Rust y serializado a `lastOpenedAt`
- [x] Task 2: Actualizar timestamp desde acciones del usuario
  - [x] Subtask 2.1: `open_editor` actualiza `lastOpenedAt`
  - [x] Subtask 2.2: `launch_agent` actualiza `lastOpenedAt`
  - [x] Subtask 2.3: `resume_agent_session` actualiza `lastOpenedAt`
  - [x] Subtask 2.4: `open_terminal` actualiza `lastOpenedAt`

## Dev Notes

- La logica comun se centralizo en `update_last_opened(project_id)` dentro de `actions.rs`.
- El cambio queda persistido en `projects.json`.
- Nota de alcance: el refresh visual inmediato del dashboard no esta forzado tras cada accion; la persistencia si quedo implementada.

## File List

| File | Action |
|------|--------|
| `src-tauri/src/models/project.rs` | Existing model consumed |
| `src-tauri/src/commands/actions.rs` | Modified (actualizacion de `lastOpenedAt`) |

## Change Log

- `lastOpenedAt` se actualiza desde editores, agentes, sesiones y terminal por proyecto
