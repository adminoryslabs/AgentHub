---
story_key: 7-2-ui-del-bloc-de-notas-por-proyecto
status: done
epic: 7
epic_title: Bloc de Notas
---

# Story 7.2: UI del bloc de notas por proyecto

## Story

As a user,
I want to access a notepad for each project from the project card,
So that I can jot down quick thoughts about that project.

## Acceptance Criteria

**Given** un ProjectCard
**When** el usuario expande el card o hace clic en "Notes"
**Then** se abre un panel lateral o modal con un editor de texto simple

**Given** el editor de notas abierto
**When** el usuario escribe y presiona Ctrl+S o hace clic en "Save"
**Then** la nota se guarda y se muestra "Saved" brevemente

**Given** una nota vacía
**When** el usuario abre el bloc de notas
**Then** muestra un placeholder "No notes yet — start typing..."

## Tasks/Subtasks

- [x] Task 1: Crear modal de notas por proyecto
  - [x] Subtask 1.1: Dialog reutilizando estilos existentes
  - [x] Subtask 1.2: Carga inicial desde `getProjectNote()`
  - [x] Subtask 1.3: Textarea simple con placeholder
- [x] Task 2: Conectar guardado
  - [x] Subtask 2.1: Boton `Save`
  - [x] Subtask 2.2: Atajo `Ctrl+S` / `Cmd+S`
  - [x] Subtask 2.3: Toast `Saved` al persistir correctamente
- [x] Task 3: Integrar acceso desde `ProjectCard`
  - [x] Subtask 3.1: Boton `Notes` junto a acciones del proyecto

## Dev Notes

- Se implemento como modal en lugar de panel lateral para reutilizar el patron `dialog-*` ya existente.
- La nota se carga on-demand al abrir el dialog.
- El guardado usa los commands de la story 7.1 sin introducir estado global nuevo.

## File List

| File | Action |
|------|--------|
| `src/components/ProjectNotesDialog.tsx` | Created (modal de notas por proyecto) |
| `src/components/ProjectCard.tsx` | Modified (boton Notes + apertura de dialog) |

## Change Log

- Nueva UI de notas por proyecto accesible desde cada card
- Guardado manual y por atajo de teclado
- Placeholder y feedback visual de persistencia

## Dev Agent Record

### Implementation Plan
1. Crear modal de notas simple
2. Conectarlo a `getProjectNote` y `saveProjectNote`
3. Integrar boton de acceso en `ProjectCard`
4. Verificar build

### Completion Notes
- Se uso `Ctrl+S` y tambien `Cmd+S` para compatibilidad con macOS.
- El feedback de "Saved" se resolvio con toast, consistente con el resto de la app.

### Debug Log
- `cargo check` OK
- `npm run build` OK
