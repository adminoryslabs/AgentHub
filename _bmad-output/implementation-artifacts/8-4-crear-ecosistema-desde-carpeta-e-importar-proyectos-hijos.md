---
story_key: 8-4-crear-ecosistema-desde-carpeta-e-importar-proyectos-hijos
status: done
epic: 8
epic_title: Ecosistemas (Padre/Hijos)
---

# Story 8.4: Crear ecosistema desde carpeta e importar proyectos hijos

## Story

As a user,
I want to register an ecosystem folder and import its child projects,
So that I can onboard a whole workspace quickly.

## Acceptance Criteria

**Given** el flujo de alta
**When** el usuario elige "Add Ecosystem Folder"
**Then** puede introducir `name`, `rootPath`, `environment` y `defaultAgent`

**Given** una carpeta root valida
**When** la app la escanea
**Then** detecta las subcarpetas directas candidatas a proyecto

**Given** el resultado del escaneo
**When** el usuario confirma la importacion
**Then** puede seleccionar que subcarpetas se registran como proyectos hijos del ecosistema

**Given** una subcarpeta ya registrada como proyecto
**When** forma parte del escaneo
**Then** la app evita duplicados y muestra el conflicto de forma clara

## Tasks/Subtasks

- [x] Task 1: Exponer scan/import de ecosystem folder en backend
  - [x] Subtask 1.1: Escanear subcarpetas directas candidatas
  - [x] Subtask 1.2: Informar conflictos por rutas ya registradas
  - [x] Subtask 1.3: Importar seleccionadas creando ecosistema y proyectos hijos
- [x] Task 2: Crear dialog de alta de ecosistema con preview
  - [x] Subtask 2.1: Formulario con `name`, `rootPath`, `environment`, `defaultAgent`
  - [x] Subtask 2.2: Accion `Scan` y listado con checkboxes
  - [x] Subtask 2.3: Deshabilitar seleccion de entradas duplicadas
- [x] Task 3: Integrar el nuevo flujo con el dashboard actual
  - [x] Subtask 3.1: Boton `Add Ecosystem Folder`
  - [x] Subtask 3.2: Refrescar proyectos tras importar
- [x] Task 4: Verificar la story
  - [x] Subtask 4.1: Build/checks limpios
  - [x] Subtask 4.2: Actualizar artifact y sprint status

## Dev Notes

- Alcance minimo: escaneo de subcarpetas directas, sin recursion profunda.
- Los proyectos importados se crean con valores por defecto consistentes: `preferredEditor = vscode`, `defaultAgent = ecosystem.defaultAgent`, `tags = []`.
- Los conflictos por path duplicado se muestran en preview y no pueden ser seleccionados para import.

## File List

| File | Action |
|------|--------|
| `_bmad-output/implementation-artifacts/8-4-crear-ecosistema-desde-carpeta-e-importar-proyectos-hijos.md` | Created (artifact de story) |
| `dev-control-center/src-tauri/src/commands/ecosystems.rs` | Modified (scan/import de ecosystem folder) |
| `dev-control-center/src-tauri/src/lib.rs` | Modified (registro de nuevos commands) |
| `dev-control-center/src/lib/invoke.ts` | Modified (wrappers y tipos de scan/import) |
| `dev-control-center/src/components/AddEcosystemFolderDialog.tsx` | Created (dialog con formulario, preview y seleccion) |
| `dev-control-center/src/components/ProjectList.tsx` | Modified (boton y wiring de `Add Ecosystem Folder`) |
| `dev-control-center/src/components/AddProjectDialog.tsx` | Modified (recarga ecosistemas al abrir) |

## Change Log

- Artifact inicial creado para la nueva Story 8.4
- Se añadieron commands backend para escanear roots e importar proyectos hijos
- La UI ahora permite crear un ecosistema desde carpeta con preview de subcarpetas directas
- Los paths ya registrados aparecen como conflicto y no se pueden seleccionar para importar

## Dev Agent Record

### Implementation Plan
1. Añadir commands de scan/import en backend
2. Crear dialog de `Add Ecosystem Folder` con preview y checkboxes
3. Integrar el flujo en `ProjectList`
4. Ejecutar `cargo check` y `npm run build`

### Completion Notes
- El escaneo se limita a subcarpetas directas para mantener la complejidad en nivel medio y evitar heuristicas recursivas en esta iteracion.
- El import crea el ecosistema y luego los proyectos hijos seleccionados usando defaults consistentes: `preferredEditor = vscode`, `defaultAgent = ecosystem.defaultAgent`, `tags = []`.
- Los conflictos por path duplicado se detectan antes de importar y tambien se revalidan en backend al confirmar.
- Tras importar, el dashboard refresca los proyectos y vuelve a resolver los grupos desde `Ecosystem`.

### Debug Log
- `cargo check` OK
- `npm run build` OK
