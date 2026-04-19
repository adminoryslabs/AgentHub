---
story_key: 7-3-ui-del-bloc-de-notas-general
status: done
epic: 7
epic_title: Bloc de Notas
---

# Story 7.3: UI del bloc de notas general

## Story

As a user,
I want a general-purpose notepad accessible from the app's top bar,
So that I can capture ideas that aren't tied to a specific project.

## Acceptance Criteria

**Given** la barra superior
**When** el usuario hace clic en "Notes" o un ícono de bloc
**Then** se abre un panel/modal con la nota general (`_general.md`)

**Given** el editor general abierto
**When** el usuario guarda
**Then** se guarda en `~/.dev-control-center/notes/_general.md`

## Tasks/Subtasks

- [x] Task 1: Crear modal de nota general
  - [x] Subtask 1.1: Dialog reutilizando estilos existentes
  - [x] Subtask 1.2: Carga inicial desde `getGeneralNote()`
  - [x] Subtask 1.3: Textarea simple con placeholder
- [x] Task 2: Conectar guardado
  - [x] Subtask 2.1: Boton `Save`
  - [x] Subtask 2.2: Atajo `Ctrl+S` / `Cmd+S`
  - [x] Subtask 2.3: Toast `Saved` al persistir correctamente
- [x] Task 3: Integrar acceso en `TopBar`
  - [x] Subtask 3.1: Boton `Notes` en barra superior

## Dev Notes

- Se implemento como modal para mantener coherencia con `AddProjectDialog` y `ProjectNotesDialog`.
- La nota general reutiliza los commands de 7.1 y persiste en `_general.md`.
- El comportamiento de guardado y feedback replica el de las notas por proyecto.

## File List

| File | Action |
|------|--------|
| `src/components/GeneralNotesDialog.tsx` | Created (modal de nota general) |
| `src/components/TopBar.tsx` | Modified (boton Notes + apertura de dialog) |

## Change Log

- Nueva UI de nota general accesible desde la barra superior
- Guardado manual y por atajo de teclado
- Persistencia conectada a `_general.md`

## Dev Agent Record

### Implementation Plan
1. Crear modal de nota general simple
2. Conectarlo a `getGeneralNote` y `saveGeneralNote`
3. Integrar acceso en `TopBar`
4. Verificar build

### Completion Notes
- Se mantuvo el mismo placeholder y patron de interaccion que las notas por proyecto.
- El acceso desde top bar cubre el caso de ideas no asociadas a un proyecto.

### Debug Log
- `cargo check` OK
- `npm run build` OK
