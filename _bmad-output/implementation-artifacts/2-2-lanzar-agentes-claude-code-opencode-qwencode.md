---
story_key: 2-2-lanzar-agentes-claude-code-opencode-qwencode
status: done
epic: 2
epic_title: Acciones Locales — Abrir Herramientas
---

# Story 2.2: Lanzar Agentes (Claude Code + OpenCode + QwenCode)

## Story

As a user,
I want to launch Claude Code, OpenCode, or QwenCode in an external terminal,
So that I can start working with my preferred AI agent.

## Acceptance Criteria

**Given** un proyecto con `env: "wsl"`
**When** el comando Tauri `launch_agent` se ejecuta con `{ projectId: "abc-123", agent: "claude" }`
**Then** se abre un terminal externo ejecutando `wsl claude` en el directorio del proyecto

**Given** un proyecto con `env: "windows"`
**When** el comando Tauri `launch_agent` se ejecuta con `{ projectId: "abc-123", agent: "opencode" }`
**Then** se abre un terminal externo ejecutando `opencode` en el directorio del proyecto

**Given** un proyecto con `env: "mac"`
**When** el comando Tauri `launch_agent` se ejecuta con `{ projectId: "abc-123", agent: "qwen" }`
**Then** se abre un terminal externo ejecutando `qwen` en el directorio del proyecto

**Given** el agente no está instalado en PATH
**When** se intenta ejecutar `launch_agent`
**Then** retorna `Err("Claude Code no encontrado en PATH. Instalalo primero.")`
**And** el frontend muestra un toast con el error

## Tasks/Subtasks

- [x] Task 1: Implementar comando Tauri `launch_agent`
  - [x] Subtask 1.1: `launch_agent` en `src-tauri/src/commands/actions.rs`
  - [x] Subtask 1.2: Buscar proyecto por ID
  - [x] Subtask 1.3: Construir comando según env
  - [x] Subtask 1.4: Validar PATH + ruta
  - [x] Subtask 1.5: `std::process::Command::spawn()` con `current_dir`
- [x] Task 2: Integrar en frontend
  - [x] Subtask 2.1: `launchAgent` en `src/lib/invoke.ts`
  - [x] Subtask 2.2: 3 botones (Claude Code, OpenCode, QwenCode) en ProjectCard
  - [x] Subtask 2.3: Botón "Continue" con defaultAgent
  - [x] Subtask 2.4: Toast de error
- [x] Task 3: Registrar command
  - [x] Subtask 3.1: `launch_agent` en `generate_handler!`
- [x] Task 4: Verificar build
  - [x] Subtask 4.1: `npm run tauri build` exitoso

## File List

| File | Action |
|------|--------|
| `src-tauri/src/commands/actions.rs` | Modified (added launch_agent) |
| `src-tauri/src/lib.rs` | Modified (added launch_agent handler) |
| `src/lib/invoke.ts` | Modified (added launchAgent) |
| `src/components/ProjectCard.tsx` | Modified (3 agent buttons + Continue) |
| `src/components/ProjectList.tsx` | Modified (agent toast handler) |

## Change Log

- Comando `launch_agent` con validación PATH + ruta
- 3 botones de agentes + botón Continue con defaultAgent
- Agentes se lanzan en el directorio del proyecto
- Build exitoso

## Dev Agent Record

### Completion Notes
- `launch_agent` comparte lógica con `open_editor` (same file)
- Usa `current_dir()` para setear el CWD del agente
- Toast system reutilizado via UIContext
