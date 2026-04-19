---
story_key: 2-1-abrir-editores-vscode-cursor
status: done
epic: 2
epic_title: Acciones Locales — Abrir Herramientas
---

# Story 2.1: Abrir Editores (VSCode + Cursor)

## Story

As a user,
I want to open my project in VSCode or Cursor with one click,
So that I can start coding immediately.

## Acceptance Criteria

**Given** un proyecto con `env: "wsl"` y `preferredEditor: "vscode"`
**When** el comando Tauri `open_editor` se ejecuta con `{ projectId: "abc-123", editor: "vscode" }`
**Then** se ejecuta `wsl code /home/mario/dev/project` en la máquina local
**And** VSCode abre con el proyecto cargado

**Given** un proyecto con `env: "windows"` y `preferredEditor: "cursor"`
**When** el comando Tauri `open_editor` se ejecuta con `{ projectId: "abc-123", editor: "cursor" }`
**Then** se ejecuta `cursor .` con el `cwd` del proyecto
**And** Cursor abre con el proyecto cargado

**Given** un proyecto con `env: "mac"` y `preferredEditor: "vscode"`
**When** el comando Tauri `open_editor` se ejecuta con `{ projectId: "abc-123", editor: "vscode" }`
**Then** se ejecuta `code .` con el `cwd` del proyecto
**And** VSCode abre con el proyecto cargado

**Given** el editor no está instalado en PATH
**When** se intenta ejecutar `open_editor`
**Then** retorna `Err("VSCode no encontrado en PATH. Instalalo primero.")`
**And** el frontend muestra un toast con el error

**Given** la ruta del proyecto no existe en el filesystem
**When** se intenta ejecutar `open_editor`
**Then** retorna `Err("Ruta no encontrada: /path/to/project")`
**And** el frontend muestra un toast con el error

## Tasks/Subtasks

- [x] Task 1: Implementar comando Tauri `open_editor`
  - [x] Subtask 1.1: `src-tauri/src/commands/actions.rs` creado
  - [x] Subtask 1.2: Busca proyecto por ID en projects.json
  - [x] Subtask 1.3: Construye comando según env (wsl prefix, native)
  - [x] Subtask 1.4: Valida editor en PATH con `which`/`where`
  - [x] Subtask 1.5: Valida ruta del proyecto existe
  - [x] Subtask 1.6: Ejecuta con `std::process::Command::spawn()`
- [x] Task 2: Integrar en frontend
  - [x] Subtask 2.1: `openEditor` en `src/lib/invoke.ts`
  - [x] Subtask 2.2: Botones VSCode y Cursor en `ProjectCard` conectados
  - [x] Subtask 2.3: Toast de error via UIContext
- [x] Task 3: Registrar command en lib.rs
  - [x] Subtask 3.1: Módulo `commands::actions` importado
  - [x] Subtask 3.2: `open_editor` registrado en `generate_handler!`
- [x] Task 4: Verificar compilación
  - [x] Subtask 4.1: `npm run tauri build` compila exitosamente

## Dev Notes

### Comandos por entorno
| Env | Editor | Comando |
|-----|--------|---------|
| wsl | vscode | `wsl code <path>` |
| wsl | cursor | `wsl cursor <path>` |
| windows | vscode | `code <path>` |
| windows | cursor | `cursor <path>` |
| mac | vscode | `code <path>` |
| mac | cursor | `cursor <path>` |

### Borrows en Rust
- `get_editor_command` devuelve `String` (no `&'static str`) para evitar lifetime issues
- `which` toma `&str` — se pasa `&editor_cmd`
- `Command::new` toma `&str` — se pasa `&editor_cmd`

### UIContext
- Nuevo context para toasts (error/success/info)
- Toasts auto-dismiss en 4 segundos
- Se muestra en bottom-right corner

## File List

| File | Action |
|------|--------|
| `src-tauri/src/commands/actions.rs` | Created (open_editor command) |
| `src-tauri/src/lib.rs` | Modified (added actions module + handler) |
| `src/lib/invoke.ts` | Modified (added openEditor) |
| `src/contexts/UIContext.tsx` | Created (toast system) |
| `src/components/ProjectCard.tsx` | Modified (VSCode/Cursor buttons wired) |
| `src/components/ProjectList.tsx` | Modified (toast handlers) |
| `src/App.tsx` | Modified (UIProvider wrapper) |

## Change Log

- Comando `open_editor` con validación de PATH y ruta
- Botones VSCode y Cursor funcionales en ProjectCard
- Sistema de toasts (UIContext) para errores y éxitos
- Build exitoso: .deb + .rpm

## Dev Agent Record

### Implementation Plan
1. Crear actions.rs con open_editor
2. Validar PATH + ruta del proyecto
3. Ejecutar según env (wsl/native)
4. Conectar botones frontend + toast de errores
5. Verificar build

### Completion Notes
- `std::process::Command::spawn()` sin wait — el editor abre en background
- Lifetime fix: `get_editor_command` devuelve `String` en vez de `&'static str`
- UIContext reutilizable para futuras stories

### Debug Log
- Lifetime error: `&'static str` vs local `&str` → fix: retornar `String`
- Move error: `editor_cmd` movido en if/else → fix: `&editor_cmd` en Command::new
- TS: ReactNode type import → fix: `type ReactNode`
