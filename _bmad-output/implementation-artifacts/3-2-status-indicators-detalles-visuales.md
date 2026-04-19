---
story_key: 3-2-status-indicators-detalles-visuales
status: done
epic: 3
epic_title: Polish Visual — The Command Matrix
---

# Story 3.2: Status Indicators + Detalles Visuales

## Story

As a user,
I want visual polish on status indicators and interactive elements,
So that the app feels alive and professional.

## Acceptance Criteria

**Given** un indicador de estado (env badge)
**When** se renderiza
**Then** es un "Glow Tag": dot de 6px del color de status + texto `label-sm`
**And** aplica "Subtle Glow": `surface_tint` al 10% con blur de 12px detrás del dot

**Given** un botón primario (Continue)
**When** se renderiza
**Then** tiene fondo `primary_container`, texto `on_primary_container`
**And** sin rounded corners (sm o none)

**Given** un botón secundario (VSCode, Cursor, agentes)
**When** se renderiza
**Then** fondo transparente con "Ghost Border" en `outline-variant` al 15%

**Given** un input de formulario está en focus
**When** el usuario hace clic en el input
**Then** el "Ghost Border" cambia a 80% de opacidad con color primary (#00e475)
**And** no hay anillos gruesos de focus

## Tasks/Subtasks

- [x] Task 1: Refinar Glow Tags
  - [x] Subtask 1.1: Dot de 6px con glow via `box-shadow`
  - [x] Subtask 1.2: Variantes success, warning y error definidas en CSS
  - [x] Subtask 1.3: `ProjectCard` muestra badge `glow-tag` para `env`
- [x] Task 2: Refinar botones
  - [x] Subtask 2.1: `btn-primary` usa fondo `primary-dim` y texto `primary`
  - [x] Subtask 2.2: `btn-ghost` usa borde `outline/15` y hover `surface-active`
  - [x] Subtask 2.3: `btn-danger` usa borde `error/30` y hover `error/10`
- [x] Task 3: Refinar inputs
  - [x] Subtask 3.1: `input-field` usa `surface-lowest`
  - [x] Subtask 3.2: Todos los inputs usan `font-mono`
  - [x] Subtask 3.3: Focus sin anillos gruesos, con borde primary al 80%
- [x] Task 4: Verificar build
  - [x] Subtask 4.1: El polish visual queda integrado en componentes existentes

## Dev Notes

### Glow Tag CSS
```css
.glow-tag::before {
  content: '';
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: currentColor;
  box-shadow: 0 0 12px rgb(0 228 117 / 0.1);
}
```

### No hacer
- ❌ No agregar nuevas features funcionales
- ❌ No cambiar estructura de componentes
- ✅ Solo refinamiento visual según DESIGN.md

## File List

| File | Action |
|------|--------|
| `src/styles/globals.css` | Modified (glow tags, buttons, inputs, dialogs) |
| `src/components/ProjectCard.tsx` | Modified (badge de entorno y botones finales) |
| `src/components/ProjectList.tsx` | Modified (toolbar/search/empty states con clases finales) |
| `src/components/AddProjectDialog.tsx` | Modified (focus/input polish) |
| `src/components/SessionHistory.tsx` | Modified (lista de sesiones y estados hover) |

## Change Log

- Glow tags implementados y aplicados a badges de entorno
- Sistema de botones primary/ghost/danger consolidado
- Inputs y dialogs alineados con el polish final del design system
- Session history hereda hover states y densidad visual consistentes

## Dev Agent Record

### Implementation Plan
1. Cerrar estados visuales pendientes de badges, botones e inputs
2. Reusar clases globales para evitar estilos duplicados
3. Aplicar polish final en cards, formularios y sesiones

### Completion Notes
- El badge de entorno termino siendo el principal uso de glow tags
- El boton `Continue` quedo como CTA primaria directa usando `defaultAgent`
- El refinement visual se mezclo con 3.1, por eso este artifact se completo despues de haber quedado el codigo listo

### Debug Log
- Sin incidencias relevantes registradas en este artifact
