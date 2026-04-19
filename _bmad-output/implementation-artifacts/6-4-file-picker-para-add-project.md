---
story_key: 6-4-file-picker-para-add-project
status: done
epic: 6
epic_title: Quick Wins
---

# Story 6.4: File picker para Add Project

## Story

As a user,
I want to select a folder using a native file dialog instead of typing the path,
So that I can add projects faster and avoid typos.

## Tasks/Subtasks

- [x] Task 1: Exponer command `pick_directory`
  - [x] Subtask 1.1: Dialog nativo con `rfd` fuera de WSL
  - [x] Subtask 1.2: Fallback via PowerShell en WSL
  - [x] Subtask 1.3: Conversion de rutas Windows a WSL con `wslpath`
- [x] Task 2: Integrar boton `Browse` en Add Project
  - [x] Subtask 2.1: Completar input `path` con la carpeta elegida

## File List

| File | Action |
|------|--------|
| `src-tauri/src/commands/projects.rs` | Modified (command `pick_directory`) |
| `src-tauri/src/lib.rs` | Modified (registro de `pick_directory`) |
| `src/components/AddProjectDialog.tsx` | Modified (boton Browse) |
| `src/lib/invoke.ts` | Modified (wrapper `pickDirectory`) |

## Change Log

- Selector nativo de carpetas integrado en Add Project
- Compatibilidad explicita con escenarios WSL
