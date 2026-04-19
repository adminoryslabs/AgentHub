---
story_key: 8-1-entidad-de-ecosistema-y-relacion-con-proyectos
status: done
epic: 8
epic_title: Ecosistemas (Padre/Hijos)
---

# Story 8.1: Entidad de ecosistema y relacion con proyectos

## Story

As a user,
I want ecosystems to have their own configuration,
So that root path, agent defaults, and grouping do not depend on arbitrary projects.

## Acceptance Criteria

**Given** la persistencia local de la app
**When** un ecosistema existe
**Then** se guarda como entidad propia con al menos `id`, `name`, `rootPath`, `defaultAgent` y `environment`

**Given** un proyecto registrado
**When** pertenece a un ecosistema
**Then** referencia al ecosistema por `ecosystemId` o equivalente estable, no por datos derivados del proyecto

**Given** que estamos en fase de testing
**When** se introduce el nuevo modelo
**Then** no es obligatorio mantener compatibilidad con el modelo provisional anterior

## Tasks/Subtasks

- [x] Task 1: Introducir la entidad `Ecosystem` en persistencia local
  - [x] Subtask 1.1: Crear modelo y store `ecosystems.json`
  - [x] Subtask 1.2: Añadir commands base para leer/escribir ecosistemas
- [x] Task 2: Cambiar `Project` para referenciar ecosistemas por id
  - [x] Subtask 2.1: Sustituir `ecosystem`/`ecosystemRoot` por `ecosystemId`
  - [x] Subtask 2.2: Validar integridad referencial al crear/editar proyectos
- [x] Task 3: Adaptar el frontend al nuevo modelo base
  - [x] Subtask 3.1: Tipos y wrappers para `Ecosystem`
  - [x] Subtask 3.2: UI minima compatible con proyectos y ecosistemas existentes
- [x] Task 4: Verificar el refactor del modelo
  - [x] Subtask 4.1: Build/checks limpios
  - [x] Subtask 4.2: Actualizar tracking y artifact

## Dev Notes

- Alcance minimo: dejar listo el modelo correcto para que 8.2, 8.3, 8.4 y 8.5 trabajen sobre una entidad `Ecosystem` real.
- No se implementa en esta story el flujo completo de crear ecosistemas desde carpeta; solo la base de datos y relacion.
- Dado que estamos en testing, se prioriza un modelo limpio sobre compatibilidad con el prototipo anterior.

## File List

| File | Action |
|------|--------|
| `_bmad-output/implementation-artifacts/8-1-entidad-de-ecosistema-y-relacion-con-proyectos.md` | Created (artifact de story) |
| `dev-control-center/src-tauri/src/models/ecosystem.rs` | Created (modelo y store de ecosistemas) |
| `dev-control-center/src-tauri/src/commands/ecosystems.rs` | Created (commands base de persistencia para ecosistemas) |
| `dev-control-center/src-tauri/src/models/project.rs` | Modified (proyectos referencian `ecosystemId`) |
| `dev-control-center/src-tauri/src/commands/projects.rs` | Modified (validacion de relacion proyecto-ecosistema) |
| `dev-control-center/src-tauri/src/commands/actions.rs` | Modified (`Open All` usa la entidad `Ecosystem`) |
| `dev-control-center/src-tauri/src/commands/notes.rs` | Modified (usa el helper nuevo de directorio base) |
| `dev-control-center/src-tauri/src/lib.rs` | Modified (registro de models y commands de ecosistemas) |
| `dev-control-center/src/lib/invoke.ts` | Modified (tipos `Ecosystem`, `Project` y wrappers) |
| `dev-control-center/src/components/AddProjectDialog.tsx` | Modified (selector por `ecosystemId`) |
| `dev-control-center/src/components/ProjectCard.tsx` | Modified (muestra nombre de ecosistema resuelto) |
| `dev-control-center/src/components/ProjectList.tsx` | Modified (resuelve grupos y `Open All` desde la entidad `Ecosystem`) |

## Change Log

- Artifact inicial creado para la nueva definicion de Story 8.1
- Se introdujo `ecosystems.json` como nueva fuente de verdad para ecosistemas
- `Project` ahora guarda `ecosystemId` en lugar de `ecosystem` y `ecosystemRoot`
- El backend expone commands base de ecosistemas y valida la integridad referencial al guardar proyectos
- El frontend resuelve nombres/grupos desde `Ecosystem`, manteniendo una UI minima compatible con el nuevo modelo
- Se agrego validacion adicional para rechazar proyectos cuyo `path` no cae dentro de `rootPath` del ecosistema

## Dev Agent Record

### Implementation Plan
1. Crear `Ecosystem` y su store en backend
2. Refactorizar `Project` para usar `ecosystemId`
3. Adaptar frontend/tipos a la nueva relacion
4. Ejecutar `cargo check` y `npm run build`

### Completion Notes
- Se priorizo un modelo limpio sobre compatibilidad con el prototipo anterior porque el proyecto sigue en fase de testing.
- `create_ecosystem`, `update_ecosystem` y `delete_ecosystem` quedaron disponibles como base para 8.4 y 8.5, aunque todavia no exista el flujo completo en UI.
- `Open All` dejo de depender del primer proyecto del grupo y ahora usa `rootPath` y `defaultAgent` del ecosistema.
- La asignacion de ecosistema en alta/edicion de proyectos ahora se hace por `ecosystemId` y valida que el `environment` del proyecto coincida con el del ecosistema.
- La asignacion tambien valida que `project.path` este contenido dentro de `ecosystem.rootPath`; si no, el backend rechaza la asociacion.

### Debug Log
- `cargo check` OK
- `npm run build` OK
