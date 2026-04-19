---
story_key: 6-1-boton-de-terminal-por-proyecto
status: done
epic: 6
epic_title: Quick Wins
---

# Story 6.1: Boton de terminal por proyecto

## Story

As a user,
I want to open a terminal in my project directory with one click,
So that I can run ad-hoc commands without manual navigation.

## Tasks/Subtasks

- [x] Task 1: Exponer command `open_terminal`
  - [x] Subtask 1.1: Validar proyecto y ruta
  - [x] Subtask 1.2: Abrir terminal por plataforma/entorno
- [x] Task 2: Integrar boton `Terminal` en `ProjectCard`
  - [x] Subtask 2.1: Callback frontend
  - [x] Subtask 2.2: Manejo de errores via toast

## Dev Notes

- Windows + WSL abre `wt.exe` ejecutando `wsl bash -ic` en la ruta del proyecto.
- Windows nativo usa `pwsh` con `--startingDirectory`.
- macOS y Linux usan terminales nativos disponibles.

## File List

| File | Action |
|------|--------|
| `src-tauri/src/commands/actions.rs` | Modified (command `open_terminal`) |
| `src/components/ProjectCard.tsx` | Modified (boton Terminal) |
| `src/lib/invoke.ts` | Modified (wrapper `openTerminal`) |

## Change Log

- Acceso rapido a terminal contextual por proyecto
