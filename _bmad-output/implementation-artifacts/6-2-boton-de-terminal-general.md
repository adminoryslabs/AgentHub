---
story_key: 6-2-boton-de-terminal-general
status: done
epic: 6
epic_title: Quick Wins
---

# Story 6.2: Boton de terminal general

## Story

As a user,
I want to quickly open a WSL or PowerShell terminal from the app,
So that I can run commands without switching contexts manually.

## Tasks/Subtasks

- [x] Task 1: Exponer command `open_global_terminal`
  - [x] Subtask 1.1: Soporte `wsl`
  - [x] Subtask 1.2: Soporte `powershell`
  - [x] Subtask 1.3: Fallbacks para macOS/Linux
- [x] Task 2: Agregar accesos en top bar
  - [x] Subtask 2.1: Boton `Terminal WSL`
  - [x] Subtask 2.2: Boton `Terminal PS`

## File List

| File | Action |
|------|--------|
| `src-tauri/src/commands/actions.rs` | Modified (command `open_global_terminal`) |
| `src/components/TopBar.tsx` | Created/Modified (acciones globales) |
| `src/lib/invoke.ts` | Modified (wrapper `openGlobalTerminal`) |

## Change Log

- Apertura rapida de terminales globales desde la barra superior
