---
story_key: 8-5-gestion-manual-de-ecosistemas
status: done
epic: 8
epic_title: Ecosistemas (Padre/Hijos)
---

# Story 8.5: Gestion manual de ecosistemas

## Story

As a user,
I want to create, edit, and delete ecosystems manually,
So that I can manage grouping and ecosystem metadata even without reimporting folders.

## Acceptance Criteria

**Given** la barra superior o un menú de gestión
**When** el usuario hace clic en "Manage Ecosystems"
**Then** se abre un dialog con la lista de ecosistemas existentes y sus proyectos

**Given** el dialog de ecosistemas
**When** el usuario edita un ecosistema
**Then** puede cambiar `name`, `rootPath` y `defaultAgent`

**Given** el dialog de ecosistemas
**When** el usuario asigna proyectos existentes a un ecosistema
**Then** puede hacerlo con checkboxes o una interacción equivalente simple

**Given** el dialog de ecosistemas
**When** el usuario elimina un ecosistema
**Then** los proyectos quedan como "Ungrouped" y no se eliminan

## Tasks/Subtasks

- [x] Task 1: Exponer operaciones manuales de ecosistemas al frontend
  - [x] Subtask 1.1: Wrappers `create/update/delete` en `invoke.ts`
  - [x] Subtask 1.2: Reusar el backend existente de ecosistemas
- [x] Task 2: Crear dialog `Manage Ecosystems`
  - [x] Subtask 2.1: Listado de ecosistemas y modo `new`
  - [x] Subtask 2.2: Formulario editable de `name`, `rootPath`, `env`, `defaultAgent`
  - [x] Subtask 2.3: Borrado manual de ecosistema
- [x] Task 3: Permitir asignacion manual de proyectos
  - [x] Subtask 3.1: Checkboxes para proyectos existentes
  - [x] Subtask 3.2: Respetar validaciones de `env` y `rootPath`
- [x] Task 4: Resolver normalizacion Windows/WSL para paths canónicos
  - [x] Subtask 4.1: Persistir rutas `wsl` como `/...` aunque entren como UNC
  - [x] Subtask 4.2: Mantener scan/import funcionales en Windows usando una ruta accesible al host
- [x] Task 5: Verificar story y tracking
  - [x] Subtask 5.1: Build/checks limpios
  - [x] Subtask 5.2: Actualizar artifact y sprint status

## Dev Notes

- Esta story cierra el gap funcional detectado tras 8.4: editar ecosistemas ya existentes y cambiar su `defaultAgent`.
- El fix de normalizacion WSL/Windows se incorporo aqui porque impacta tanto la edicion manual como el flujo previo de import desde carpeta.
- La persistencia canónica para `env = wsl` pasa a ser siempre ruta POSIX `/...`.

## File List

| File | Action |
|------|--------|
| `_bmad-output/implementation-artifacts/8-5-gestion-manual-de-ecosistemas.md` | Created (artifact de story) |
| `dev-control-center/src-tauri/src/commands/projects.rs` | Modified (normalizacion canónica de paths para storage) |
| `dev-control-center/src-tauri/src/commands/ecosystems.rs` | Modified (scan/import/update con soporte Windows+WSL y validacion de asignaciones) |
| `dev-control-center/src/lib/invoke.ts` | Modified (wrappers `create/update/delete_ecosystem`) |
| `dev-control-center/src/components/ManageEcosystemsDialog.tsx` | Created (gestion manual de ecosistemas) |
| `dev-control-center/src/components/TopBar.tsx` | Modified (acceso al dialog `Manage Ecosystems`) |

## Change Log

- Artifact inicial creado para la nueva Story 8.5
- Se añadió el dialog `Manage Ecosystems` para crear, editar y borrar ecosistemas manualmente
- Se añadió asignacion manual de proyectos por checkboxes, respetando `env` y `rootPath`
- Se normalizan rutas WSL en Windows para persistir siempre `/...` aunque entren como UNC
- Scan/import siguen funcionando en Windows usando una ruta accesible al host solo durante la operacion de filesystem

## Dev Agent Record

### Implementation Plan
1. Añadir wrappers frontend y dialog de gestion manual
2. Conectar edicion, borrado y asignacion de proyectos
3. Normalizar rutas WSL/Windows en backend
4. Ejecutar `cargo check` y `npm run build`

### Completion Notes
- El dialog permite crear ecosistemas vacios o editar existentes sin depender del flujo de import desde carpeta.
- Los proyectos ya asignados pueden desasociarse; los nuevos solo se pueden asociar si comparten `env` y caen dentro de `rootPath`.
- La persistencia canónica de rutas `wsl` evita que queden guardadas rutas UNC que luego rompen `Open All`, editores o agentes.
- En Windows, el backend convierte temporalmente rutas WSL a una ruta accesible por el host para poder hacer scan y validaciones de filesystem.

### Debug Log
- `cargo check` OK
- `npm run build` OK
