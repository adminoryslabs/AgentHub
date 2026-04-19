---
story_key: 1-1-scaffold-tauri-app-configurar-permisos
status: done
epic: 1
epic_title: Setup & Gestión de Proyectos
---

# Story 1.1: Scaffold Tauri App + Configurar Permisos

## Story

As a developer,
I want a working Tauri project configured with proper permissions and dependencies,
So that I have a solid foundation for all features.

## Acceptance Criteria

**Given** un directorio vacío
**When** ejecuto `npm create tauri-app@latest dev-control-center` con React + TypeScript
**And** instalo `tailwindcss @tailwindcss/vite` y `@tauri-apps/api`
**And** instalo `sqlx` en `src-tauri/Cargo.toml` con features `runtime-tokio` y `postgres`
**And** configuro `src-tauri/capabilities/default.json` con permisos `shell:allow-execute` (allowlist de comandos) y `fs:read-write` (scope `~/.dev-control-center/**`)
**Then** `npm run tauri dev` compila y abre la ventana desktop
**And** el frontend muestra un placeholder básico

## Tasks/Subtasks

- [x] Task 1: Scaffold proyecto Tauri con `create-tauri-app`
  - [x] Subtask 1.1: Ejecutar `npm create vite@latest . -- --template react-ts` (create-tauri-app interactivo, alternativa directa)
  - [x] Subtask 1.2: React + TypeScript configurado
  - [x] Subtask 1.3: `npm run tauri build` compila exitosamente
- [x] Task 2: Instalar y configurar dependencias
  - [x] Subtask 2.1: Tailwind CSS v3 + PostCSS + autoprefixer instalados
  - [x] Subtask 2.2: `@tauri-apps/api` instalado
  - [x] Subtask 2.3: Tailwind configurado con postcss.config.js y tailwind.config.js
  - [x] Subtask 2.4: `sqlx` en Cargo.toml con features `runtime-tokio`, `postgres`, `uuid`, `chrono`, `json`
  - [x] Subtask 2.5: `serde`, `serde_json`, `uuid`, `chrono`, `dotenv`, `tokio` instalados
- [x] Task 3: Configurar permisos Tauri v2
  - [x] Subtask 3.1: `src-tauri/capabilities/default.json` creado con permisos mínimos
  - [x] Subtask 3.2: `shell:allow-execute` con allowlist: code, cursor, claude, opencode, qwen, wsl, which, where, wt, open, bash, cmd
  - [x] Subtask 3.3: `fs:allow-read-text-file` y `fs:allow-write-text-file` con scope `$HOME/.dev-control-center/**`
- [x] Task 4: Configurar estructura de carpetas base
  - [x] Subtask 4.1: `src/components/`, `src/contexts/`, `src/hooks/`, `src/lib/`, `src/styles/` creados
  - [x] Subtask 4.2: `src-tauri/src/commands/`, `src-tauri/src/models/` creados
  - [x] Subtask 4.3: `src/App.tsx` con placeholder básico y `src/main.tsx` importando globals.css
- [x] Task 5: Verificar build y dev
  - [x] Subtask 5.1: `npm run build` compila sin errores (Vite 5 + Tailwind v3)
  - [x] Subtask 5.2: `npm run tauri build` genera .deb y .rpm exitosamente

## Dev Notes

### Arquitectura
- **Framework:** Tauri v2 con React 19 + TypeScript + Vite 5
- **Starter:** Vite react-ts template + Tauri init
- **DB Driver:** `sqlx` v0.8 con `runtime-tokio` y `postgres` features
- **Styling:** Tailwind CSS v3 via PostCSS

### Nota de versiones
- Vite 8 requiere Node 22.12+. Se usó Vite 5 (compatible con Node 22.11)
- Tailwind v4 usa `@tailwindcss/vite` plugin. Se usó Tailwind v3 con PostCSS por compatibilidad con Vite 5

### Estructura de carpetas
```
dev-control-center/
├── src/
│   ├── components/
│   ├── contexts/
│   ├── hooks/
│   ├── lib/
│   ├── styles/
│   │   └── globals.css
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/
│   ├── src/
│   │   ├── commands/
│   │   ├── models/
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── capabilities/
│       └── default.json
├── package.json
├── vite.config.ts
├── tailwind.config.js
├── postcss.config.js
└── .env (no commitear — NEON_DATABASE_URL)
```

### Permisos Tauri v2
Configurados en `src-tauri/capabilities/default.json`. Allowlist de 12 comandos shell.

## File List

| File | Action |
|------|--------|
| `dev-control-center/` | Created (new project) |
| `package.json` | Modified (added tauri script) |
| `vite.config.ts` | Created (Vite 5 config) |
| `tailwind.config.js` | Created |
| `postcss.config.js` | Created |
| `src/main.tsx` | Modified (import globals.css) |
| `src/App.tsx` | Modified (placeholder) |
| `src/styles/globals.css` | Created (Tailwind directives) |
| `src/components/` | Created |
| `src/contexts/` | Created |
| `src/hooks/` | Created |
| `src/lib/` | Created |
| `src-tauri/Cargo.toml` | Modified (added sqlx, tokio, uuid, chrono, dotenv, shell, fs plugins) |
| `src-tauri/tauri.conf.json` | Modified (identifier, window size, withGlobalTauri) |
| `src-tauri/capabilities/default.json` | Modified (shell allowlist, fs scope) |
| `src-tauri/src/commands/` | Created |
| `src-tauri/src/models/` | Created |

## Change Log

- Scaffold Tauri v2 + React 19 + TypeScript + Vite 5
- Tailwind CSS v3 configurado con PostCSS
- Permisos Tauri v2 con allowlist de 12 comandos
- sqlx + tokio + uuid + chrono + dotenv añadidos a Cargo.toml
- Estructura de carpetas base creada
- Build exitoso: .deb y .rpm generados

## Dev Agent Record

### Implementation Plan
1. Scaffold con Vite react-ts + tauri init
2. Instalar dependencias npm y Rust
3. Configurar Tailwind v3 + PostCSS
4. Configurar permisos Tauri capabilities
5. Crear estructura de carpetas
6. Verificar build

### Completion Notes
- Vite 8 incompatible con Node 22.11 → downgrade a Vite 5
- Tailwind v4 requiere Vite plugin → uso de Tailwind v3 + PostCSS
- Build exitoso: binario release + .deb + .rpm generados
- AppImage falló (linuxdeploy issue en el entorno, no bloqueante)

### Debug Log
- `npm create vite@latest` timeout interactivo → resuelto con `echo "y" |`
- `tauri init --identifier` flag no existe en esta versión → identifier se setea en tauri.conf.json
- Vite 8 rolldown binding error con Node 22.11 → downgrade a Vite 5
