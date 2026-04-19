---
story_key: 6-3-abrir-settings-del-agente
status: done
epic: 6
epic_title: Quick Wins
---

# Story 6.3: Abrir settings del agente

## Story

As a user,
I want to open the agent's settings file directly from the project card,
So that I can quickly edit Claude/Qwen/OpenCode configurations.

## Tasks/Subtasks

- [x] Task 1: Exponer command `open_agent_settings`
  - [x] Subtask 1.1: Paths para Claude, Qwen y OpenCode
  - [x] Subtask 1.2: Apertura via VS Code segun plataforma
- [x] Task 2: Integrar acceso desde historial de sesiones
  - [x] Subtask 2.1: Boton `Settings` por agente
  - [x] Subtask 2.2: Toast de exito/error

## File List

| File | Action |
|------|--------|
| `src-tauri/src/commands/actions.rs` | Modified (command `open_agent_settings`) |
| `src/components/SessionHistory.tsx` | Modified (boton Settings) |
| `src/lib/invoke.ts` | Modified (wrapper `openAgentSettings`) |

## Change Log

- Apertura directa de archivos de settings para Claude, Qwen y OpenCode
