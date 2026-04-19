---
story_key: 8-1-modelo-de-datos-de-ecosistemas
status: superseded
epic: 8
epic_title: Ecosistemas (Padre/Hijos)
---

# Story 8.1: Modelo de datos de ecosistemas

## Story

As a user,
I want to assign projects to ecosystems,
So that I can organize them logically.

## Acceptance Criteria

**Given** el archivo `projects.json`
**When** un proyecto tiene un ecosistema
**Then** incluye el campo `ecosystem: "cosnautas"` o `ecosystem: null`

**Given** el ecosistema es nuevo
**When** el usuario asigna un proyecto a un ecosistema inexistente
**Then** el ecosistema se crea automaticamente

**Given** la estructura de ecosistemas
**When** se carga la app
**Then** los ecosistemas se leen de los proyectos existentes (no hay entidad separada)

## Tasks/Subtasks

- [x] Task 1: Extender el modelo `Project` con `ecosystem` opcional
  - [x] Subtask 1.1: Serializacion camelCase compatible con `projects.json`
  - [x] Subtask 1.2: Compatibilidad con proyectos existentes sin campo
- [x] Task 2: Propagar el campo en el CRUD y wrappers del frontend
  - [x] Subtask 2.1: `create_project` y `update_project` aceptan `ecosystem`
  - [x] Subtask 2.2: `src/lib/invoke.ts` tipa el nuevo campo
- [x] Task 3: Permitir asignar ecosistema desde el flujo actual de proyecto
  - [x] Subtask 3.1: Campo opcional en `AddProjectDialog`
  - [x] Subtask 3.2: Mostrar ecosistema en la card para hacer visible el dato
- [x] Task 4: Verificar que la app deriva ecosistemas desde los proyectos
  - [x] Subtask 4.1: Sin store independiente de ecosistemas
  - [x] Subtask 4.2: Build/checks limpios

## Dev Notes

- Alcance minimo y consistente: no se crea una entidad separada para ecosistemas en esta story.
- El nombre del ecosistema vive dentro de cada proyecto como string opcional.
- Si el usuario escribe un nombre nuevo en alta/edicion, el ecosistema existe implicitamente porque se deriva desde los proyectos guardados.

## File List

| File | Action |
|------|--------|
| `_bmad-output/implementation-artifacts/8-1-modelo-de-datos-de-ecosistemas.md` | Created (artifact de story) |
| `dev-control-center/src-tauri/src/models/project.rs` | Modified (campo `ecosystem` opcional en el modelo) |
| `dev-control-center/src-tauri/src/commands/projects.rs` | Modified (CRUD acepta y normaliza `ecosystem`) |
| `dev-control-center/src/lib/invoke.ts` | Modified (tipos y requests con `ecosystem`) |
| `dev-control-center/src/components/AddProjectDialog.tsx` | Modified (campo opcional de ecosistema en alta/edicion) |
| `dev-control-center/src/components/ProjectCard.tsx` | Modified (muestra ecosistema asignado) |

## Change Log

- Artifact inicial creado siguiendo el formato usado en Epic 7
- `Project` ahora serializa `ecosystem` como string o `null` sin crear entidad separada
- Alta y edicion de proyectos permiten asignar ecosistema desde el dialog existente
- La card muestra el ecosistema para validar visualmente la persistencia
- 2026-04-18: Marcado como `superseded` tras redefinir Epic 8 hacia ecosistemas como entidad propia

## Dev Agent Record

### Implementation Plan
1. Extender el modelo y persistencia de `Project` con `ecosystem`
2. Propagar el campo al frontend y al dialog actual de alta/edicion
3. Hacer visible el dato en la card para validar el flujo
4. Ejecutar `cargo check` y `npm run build`

### Completion Notes
- Los proyectos existentes siguen siendo compatibles porque `ecosystem` usa `#[serde(default)]` y cae en `None` cuando el campo no existe.
- El valor se normaliza en backend para que strings vacios no terminen persistidos como ecosistemas invalidos.
- La existencia de un ecosistema se deriva implicitamente de los proyectos guardados, que es el comportamiento pedido por la story.
- Esta implementacion corresponde al modelo provisional anterior y ya no representa la definicion vigente de la story.

### Debug Log
- `cargo check` OK
- `npm run build` OK
