---
story_key: 5-3-reabrir-sesion
status: done
epic: 5
epic_title: Historial de Sesiones
---

# Story 5.3: Reabrir sesion de agente

## Story

As a user,
I want to reopen a specific agent session from the history,
So that I can continue working in that conversation context.

## Tasks/Subtasks

- [x] Task 1: Exponer command `resume_agent_session`
  - [x] Subtask 1.1: Request con `projectId`, `agent`, `sessionId`
  - [x] Subtask 1.2: Validacion de PATH y existencia del proyecto
- [x] Task 2: Resolver sintaxis de resume por agente
  - [x] Subtask 2.1: Claude usa `-r`
  - [x] Subtask 2.2: Resto usa `--resume`
- [x] Task 3: Integrar accion en UI de sesiones
  - [x] Subtask 3.1: Click sobre una sesion reabre en terminal externa
  - [x] Subtask 3.2: Toast de exito/error

## Dev Notes

- El soporte real esta preparado de forma generica, pero el discovery comprobado hoy cubre Claude y Qwen.
- La accion tambien actualiza `lastOpenedAt`.

## File List

| File | Action |
|------|--------|
| `src-tauri/src/commands/actions.rs` | Modified (resume de sesiones) |
| `src/components/SessionHistory.tsx` | Modified (click para reabrir) |
| `src/lib/invoke.ts` | Modified (wrapper `resumeAgentSession`) |

## Change Log

- Reapertura de sesiones desde el historial
- Adaptacion del flag de resume segun agente
- Integracion con toasts y tracking de uso reciente
