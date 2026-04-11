---
story_key: 3-2-status-indicators-detalles-visuales
status: ready-for-dev
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

- [ ] Task 1: Refinar Glow Tags
  - [ ] Subtask 1.1: Implementar dot de 6px con box-shadow glow (12px blur, 10% opacity)
  - [ ] Subtask 1.2: Variantes: success (#00e475), warning (#ffb95f), error (#ef4444)
  - [ ] Subtask 1.3: Aplicar a env badge en ProjectCard
- [ ] Task 2: Refinar botones
  - [ ] Subtask 2.1: Primary — bg `primary/20`, text `primary`, radius `sm` o `none`
  - [ ] Subtask 2.2: Ghost — transparent bg, border `outline/15`, hover `surface-active`
  - [ ] Subtask 2.3: Danger — border `error/30`, text `error`, hover `error/10`
- [ ] Task 3: Refinar inputs
  - [ ] Subtask 3.1: bg `surface-lowest`, focus border `primary/80`
  - [ ] Subtask 3.2: Font JetBrains Mono para todos los inputs
  - [ ] Subtask 3.3: Sin anillos de focus gruesos
- [ ] Task 4: Verificar build
  - [ ] Subtask 4.1: `npm run tauri build` exitoso

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

<!-- To be filled -->

## Change Log

<!-- To be filled -->

## Dev Agent Record

### Implementation Plan
<!-- To be filled -->

### Completion Notes
<!-- To be filled -->

### Debug Log
<!-- To be filled -->
