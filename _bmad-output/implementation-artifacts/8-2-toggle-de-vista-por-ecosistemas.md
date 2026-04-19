---
story_key: 8-2-toggle-de-vista-por-ecosistemas
status: done
epic: 8
epic_title: Ecosistemas (Padre/Hijos)
---

# Story 8.2: Toggle de vista por ecosistemas

## Story

As a user,
I want to toggle between a flat list and an ecosystem-grouped view,
So that I can see projects organized or uncluttered depending on my needs.

## Acceptance Criteria

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

## Tasks/Subtasks

- [x] Task 1: Introducir estado de vista compartido entre top bar y listado
  - [x] Subtask 1.1: Modo `flat | ecosystem`
  - [x] Subtask 1.2: Toggle visible en `TopBar`
- [x] Task 2: Mantener la vista plana actual como comportamiento base
  - [x] Subtask 2.1: Reutilizar sorting y filtros existentes
  - [x] Subtask 2.2: No romper acciones de `ProjectCard`
- [x] Task 3: Implementar vista agrupada por ecosistema
  - [x] Subtask 3.1: Resolver grupos desde `Ecosystem`
  - [x] Subtask 3.2: Secciones colapsables por ecosistema
  - [x] Subtask 3.3: Seccion `Ungrouped` para proyectos sin ecosistema
- [x] Task 4: Verificar consistencia visual y tecnica
  - [x] Subtask 4.1: Build/checks limpios
  - [x] Subtask 4.2: Actualizar tracking y artifact

## Dev Notes

- La base visual de esta story ya existia en el prototipo anterior, pero se reajusto para usar `ecosystemId` y la entidad `Ecosystem` como fuente de verdad.
- La preferencia de vista sigue viviendo en memoria de la sesion, que es suficiente para esta iteracion.
- `Ungrouped` sigue agrupando proyectos sin ecosistema asignado.

## File List

| File | Action |
|------|--------|
| `_bmad-output/implementation-artifacts/8-2-toggle-de-vista-por-ecosistemas.md` | Created (artifact de story) |
| `dev-control-center/src/App.tsx` | Reused (estado `viewMode`) |
| `dev-control-center/src/components/TopBar.tsx` | Reused (toggle `Flat / By Ecosystem`) |
| `dev-control-center/src/components/ProjectList.tsx` | Modified previamente para agrupar desde `Ecosystem` |
| `dev-control-center/src/components/ProjectCard.tsx` | Modified previamente para mostrar el nombre resuelto del ecosistema |

## Change Log

- Artifact creado para formalizar la nueva Story 8.2 sobre el modelo `Ecosystem`
- Se confirma que la vista agrupada ya opera sobre la entidad `Ecosystem` y no sobre campos derivados del proyecto

## Dev Agent Record

### Implementation Plan
1. Confirmar que el toggle existente sigue siendo valido sobre el nuevo modelo
2. Verificar que `ProjectList` agrupa por entidad `Ecosystem`
3. Ejecutar `cargo check` y `npm run build`

### Completion Notes
- El trabajo visual principal ya estaba hecho y quedo absorbido por el refactor de 8.1, por eso esta story se cerró sin introducir nueva UI adicional.
- La agrupacion usa `ecosystemId` para resolver `name` y `defaultAgent` desde `Ecosystem`.

### Debug Log
- `cargo check` OK
- `npm run build` OK
