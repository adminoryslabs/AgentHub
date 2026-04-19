---
story_key: 2-3-frontend-botones-de-accion-en-projectcard
status: done
epic: 2
epic_title: Acciones Locales - Abrir Herramientas
---

# Story 2.3: Frontend - Botones de Accion en ProjectCard

## Story

As a user,
I want to see action buttons on each project card,
So that I can launch editors or agents with one click.

## Tasks/Subtasks

- [x] Task 1: Agregar acciones principales al `ProjectCard`
  - [x] Subtask 1.1: CTA `Continue with {defaultAgent}`
  - [x] Subtask 1.2: Botones VSCode y Cursor
  - [x] Subtask 1.3: Botones Claude Code, OpenCode y QwenCode
- [x] Task 2: Conectar acciones con IPC de Tauri
  - [x] Subtask 2.1: `openEditor()` integrado
  - [x] Subtask 2.2: `launchAgent()` integrado
  - [x] Subtask 2.3: Manejo de errores via callbacks/toasts
- [x] Task 3: Alinear UI con Command Matrix
  - [x] Subtask 3.1: `btn-primary` para Continue
  - [x] Subtask 3.2: `btn-ghost` para el resto de acciones

## Dev Notes

- La implementacion final no usa `AgentSelector` modal. `Continue` ejecuta directamente `defaultAgent`.
- El card tambien termino incorporando `Terminal` y `SessionHistory`, cambios que luego quedaron documentados en epics 5 y 6.

## File List

| File | Action |
|------|--------|
| `src/components/ProjectCard.tsx` | Modified (botonera principal) |
| `src/components/ProjectList.tsx` | Modified (callbacks de exito/error) |
| `src/lib/invoke.ts` | Modified (wrappers de acciones) |

## Change Log

- Project cards con CTA primaria y accesos rapidos a editores/agentes
- Errores y exitos reportados via sistema de toasts
- La UX final prioriza un clic directo sobre selector intermedio

## Dev Agent Record

### Completion Notes
- Story cerrada con small deviation respecto al plan: no se implemento `AgentSelector`, se uso `defaultAgent` como flujo directo.
