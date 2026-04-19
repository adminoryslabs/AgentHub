---
story_key: 8-3-abrir-ecosistema-completo-en-agente
status: done
epic: 8
epic_title: Ecosistemas (Padre/Hijos)
---

# Story 8.3: Abrir ecosistema completo en agente

## Story

As a user,
I want to launch an agent at the ecosystem root directory,
So that I can work on cross-project implementations.

## Acceptance Criteria

**Given** un ecosistema expandido en la vista por ecosistemas
**When** el usuario hace clic en "Open All" o similar
**Then** se abre una sola sesion del agente en `rootPath` del ecosistema

**Given** el usuario define un ecosistema
**When** configura el ecosistema
**Then** puede definir su agente por defecto para `Open All`

**Given** un ecosistema configurado
**When** el usuario usa `Open All`
**Then** no se abre una sesion por cada proyecto hijo

## Tasks/Subtasks

- [x] Task 1: Lanzar agente a nivel ecosistema desde la entidad `Ecosystem`
  - [x] Subtask 1.1: Command Tauri `launch_ecosystem_agent`
  - [x] Subtask 1.2: Uso de `rootPath` y `defaultAgent` del ecosistema
- [x] Task 2: Integrar `Open All` en la vista agrupada
  - [x] Subtask 2.1: Boton por seccion de ecosistema
  - [x] Subtask 2.2: No habilitar `Open All` para `Ungrouped`
- [x] Task 3: Validar ejecucion segura del root del ecosistema
  - [x] Subtask 3.1: Verificar que la ruta exista
  - [x] Subtask 3.2: Verificar que el agente exista en el entorno correcto
- [x] Task 4: Verificar story y tracking
  - [x] Subtask 4.1: Build/checks limpios
  - [x] Subtask 4.2: Actualizar artifact y sprint status

## Dev Notes

- Esta story quedo materialmente resuelta al introducir la entidad `Ecosystem` en 8.1 y conectar `ProjectList` con `launch_ecosystem_agent`.
- `Open All` abre una sola sesion en `ecosystem.rootPath` y no itera por cada proyecto hijo.
- El agente usado sale de `ecosystem.defaultAgent`, que ya forma parte del modelo persistido.

## File List

| File | Action |
|------|--------|
| `_bmad-output/implementation-artifacts/8-3-abrir-ecosistema-completo-en-agente.md` | Created (artifact de story) |
| `dev-control-center/src-tauri/src/commands/actions.rs` | Modified previamente (`launch_ecosystem_agent`) |
| `dev-control-center/src/lib/invoke.ts` | Modified previamente (wrapper `launchEcosystemAgent`) |
| `dev-control-center/src/components/ProjectList.tsx` | Modified previamente (boton `Open All`) |
| `dev-control-center/src-tauri/src/models/ecosystem.rs` | Reused (fuente de `rootPath` y `defaultAgent`) |
| `dev-control-center/src-tauri/src/commands/ecosystems.rs` | Reused (configuracion persistida del ecosistema) |

## Change Log

- Artifact creado para formalizar la nueva Story 8.3 sobre la entidad `Ecosystem`
- Se confirma que `Open All` ya opera con `rootPath` y `defaultAgent` del ecosistema

## Dev Agent Record

### Implementation Plan
1. Confirmar que `launch_ecosystem_agent` usa el modelo `Ecosystem`
2. Confirmar que la UI agrupada expone `Open All`
3. Ejecutar `cargo check` y `npm run build`

### Completion Notes
- No hizo falta añadir nueva logica en esta iteracion porque la story quedo absorbida por el refactor del modelo base en 8.1.
- `Open All` ya no depende del primer proyecto del grupo ni deriva el root desde paths de proyectos.
- La configuracion del agente por defecto existe a nivel de ecosistema, aunque la gestion manual completa del ecosistema se implementara en 8.5.

### Debug Log
- `cargo check` OK
- `npm run build` OK
