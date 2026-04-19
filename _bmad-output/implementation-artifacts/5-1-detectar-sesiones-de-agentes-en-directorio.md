---
story_key: 5-1-detectar-sesiones-de-agentes-en-directorio
status: done
epic: 5
epic_title: Historial de Sesiones
---

# Story 5.1: Detectar sesiones de agentes en el directorio del proyecto

## Story

As a user,
I want the app to discover existing agent sessions in my project directory,
So that I can see my conversation history without manual configuration.

## Tasks/Subtasks

- [x] Task 1: Exponer command `get_sessions`
  - [x] Subtask 1.1: Request real basada en `projectPath`
  - [x] Subtask 1.2: Response con `agent`, `sessionId`, `modifiedAt`, `sizeBytes`
- [x] Task 2: Descubrir sesiones locales
  - [x] Subtask 2.1: Claude en `~/.claude/projects/<encoded>`
  - [x] Subtask 2.2: Qwen en `~/.qwen/projects/<encoded>/chats`
- [x] Task 3: Soportar scan via WSL cuando corresponde
  - [x] Subtask 3.1: Uso de `wsl test` + `wsl ls`

## Dev Notes

- Alcance real entregado: discovery de Claude y Qwen.
- No hay evidencia de discovery de OpenCode en el codigo actual.
- En paths WSL escaneados desde Windows, `modifiedAt` y `sizeBytes` quedan aproximados (`now` y `0`) porque el scan actual usa `ls` simple.

## File List

| File | Action |
|------|--------|
| `src-tauri/src/commands/sessions.rs` | Created (discovery y normalizacion de sesiones) |
| `src-tauri/src/lib.rs` | Modified (registro de `get_sessions`) |
| `src/lib/invoke.ts` | Modified (wrapper `getSessions`) |

## Change Log

- Nuevo command `get_sessions`
- Discovery para Claude y Qwen en local y escenarios WSL
- Contrato documentado segun implementacion real, no segun el plan original
