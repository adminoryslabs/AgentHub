# PRD — Dev Control Center (MVP)

## 1. Overview

Dev Control Center es una aplicación de escritorio que centraliza la gestión de proyectos de desarrollo, ejecución de agentes de IA y contexto de trabajo, con el objetivo de reducir fricción operativa y permitir un workflow agent-centric.

Es una herramienta personal para **2 usuarios** (Mario + socio), sincronizada entre múltiples máquinas (home + trabajo), con memorias compartidas vía Neon PostgreSQL.

## 2. Objetivos

### Objetivo principal

Reducir el tiempo y esfuerzo necesario para:

- Cambiar de proyecto
- Retomar contexto sin reconstruirlo manualmente
- Ejecutar agentes de IA con contexto previo

### Objetivos específicos

- Centralizar acceso a proyectos en un dashboard visual de escritorio
- Permitir ejecución rápida de herramientas (VSCode, Cursor, Claude Code, OpenCode, QwenCode) mediante comandos locales
- Recuperar contexto previo automáticamente desde memorias compartidas (Neon)
- Soportar 3 entornos: Windows, WSL, Mac
- Servir como base para experimentación con agentes (QwenCode)

## 3. No objetivos

- **Auto-scan de proyectos** — La gestión es manual, no se necesita detección automática
- **Métricas de evaluación de agentes** — No se necesitan métricas de calidad, velocidad o iteraciones
- **Auth compleja** — Token simple compartido entre 2 usuarios, no OAuth ni multi-tenant
- **Terminal embebido en la app** — Los agentes se ejecutan en terminales externos del SO, no dentro de la aplicación
- **Backend / VPS dedicado** — La app se conecta directamente a Neon desde cada máquina
- **UI tipo SaaS** — Herramienta personal, alta densidad, minimalista
- **Automatización autónoma avanzada**
- **Abstracción genérica de agentes** — Se soportan 3 agentes concretos, no un framework genérico

## 4. Usuarios

- **Mario** (usuario principal, perfil dev — usa Windows + WSL)
- **Socio de trabajo** (perfil dev — usa Mac, comparte algunos proyectos)
- Ambos acceden desde múltiples máquinas (home + trabajo)
- Ambos comparten el contexto de memorias vía Neon

## 5. Funcionalidades

### 5.1 Gestión de proyectos

Descripción: Permite registrar, visualizar y gestionar proyectos manualmente.

#### CRUD manual

Formulario para crear/editar proyecto:

- **name** — Nombre del proyecto
- **path** — Ruta en el filesystem
- **environment** — `windows` | `wsl` | `mac`
- **preferredEditor** — `vscode` | `cursor`
- **defaultAgent** — `qwencode` | `claude` | `opencode`
- **tags** (opcional)

#### Persistencia

Archivo local: `projects.json`

**Modelo de datos:**

```json
{
  "id": "uuid",
  "name": "payments-api",
  "path": "/home/mario/dev/payments-api",
  "env": "wsl",
  "preferredEditor": "vscode",
  "defaultAgent": "qwencode",
  "tags": ["backend"],
  "lastOpenedAt": "datetime"
}
```

### 5.2 Acciones rápidas por proyecto

Cada proyecto tiene **5 botones de acción concretos** que ejecutan comandos locales:

| Botón | Qué hace |
|-------|----------|
| **VSCode** | Abre el proyecto en VS Code (comando local del SO) |
| **Cursor** | Abre el proyecto en Cursor (comando local del SO) |
| **Claude Code** | Abre terminal externo con Claude Code |
| **OpenCode** | Abre terminal externo con OpenCode |
| **QwenCode** | Abre terminal externo con QwenCode |

**Implementación:** `child_process.exec()` — se ejecuta en la máquina local del usuario, no en un servidor remoto.

**Soporte multi-entorno (comandos por SO):**

| Acción | Windows | WSL | Mac |
|--------|---------|-----|-----|
| VSCode | `code .` | `wsl code .` | `code .` |
| Cursor | `cursor .` | `wsl cursor .` | `cursor .` |
| Terminal | `start cmd /k` | `wsl bash` | `open -a Terminal` |
| Agente | directo | vía `wsl` | directo |

**Mapeo de rutas:** Para proyectos en WSL abiertos desde Windows, convertir rutas con `wslpath -w`.

### 5.3 Continuar trabajo (CORE FEATURE)

Descripción: Permite retomar el flujo de trabajo sin reconstruir contexto manualmente. Es el feature central del producto.

**Flujo:**

1. Usuario hace clic en "Continuar trabajo"
2. Selecciona agente: Claude Code | OpenCode | QwenCode
3. La app:
   - Obtiene memorias desde Neon (read-only en MVP)
   - Construye contexto con última actividad y pendientes
   - Lanza agente en terminal externo con contexto inyectado

**Ejemplo de contexto inyectado:**

```
Último trabajo:
- Refactor auth
- Problema con JWT expirado

Pendientes:
- Validar middleware
- Revisar tests

Continúa desde aquí
```

### 5.4 Integración con memorias (Neon)

Descripción: Conexión directa a base de datos PostgreSQL existente (Neon) para obtener contexto por proyecto.

**Funcionalidades (MVP — read-only):**

- Obtener últimas memorias por proyecto
- Mostrar resumen en UI (última actividad, último agente, timestamp)
- Usar memorias como bootstrap para "Continuar trabajo"
- **Extensibilidad:** La arquitectura debe permitir write en el futuro (forzar guardar memoria), pero no se implementa en el MVP

**Modelo esperado (referencial):**

```json
{
  "projectId": "uuid",
  "summary": "Refactor auth",
  "details": "...",
  "createdAt": "datetime"
}
```

**Conexión:** La app se conecta directamente a Neon usando un driver PostgreSQL. La URL de conexión se almacena en un archivo `.env` local en cada máquina.

### 5.5 Visualización de estado

Cada project card muestra:

- Última actividad (resumen corto desde Neon)
- Último agente usado
- Timestamp de última interacción

### 5.6 Soporte multi-entorno (Windows + WSL + Mac)

**Requisito clave:** Cada proyecto define su entorno.

**Comportamiento por SO:**

| Elemento | Windows | WSL | Mac |
|----------|---------|-----|-----|
| Comandos | nativos | con prefijo `wsl` | nativos |
| Terminal | `cmd` / `wt` | `bash` | `Terminal.app` |
| Editores | `code`, `cursor` | `wsl code`, `wsl cursor` | `code`, `cursor` |
| Agentes | directo | vía `wsl` | directo |

## 6. UI/UX

**Lineamientos:**

- Dark mode
- Minimalista, alta densidad de información
- Enfocado en velocidad (cockpit, no SaaS)

**Componentes:**

### Project Card

- Nombre
- Estado
- Última actividad (resumen, agente, timestamp)
- Botones:
  - **Continue** (primary) → selector de agente → lanza en terminal externo con contexto
  - **VSCode**
  - **Cursor**
  - **Claude Code**
  - **OpenCode**
  - **QwenCode**

### Barra superior

- Título de la app
- Add Project

## 7. Arquitectura

**Tipo:** Aplicación de escritorio multiplataforma

**Framework:** Tauri v2

- **Frontend embebido:** React (Vite) — la UI del dashboard
- **Core:** Rust (Tauri commands) — maneja ejecución local y conexiones de datos
- **Comunicación:** IPC directo (llamadas de función entre React y Rust, sin HTTP)

**Responsabilidades del core (Rust):**

- CRUD de proyectos en `projects.json` local
- Ejecutar comandos locales (`child_process` equivalente en Rust) para abrir editores y agentes
- Conexión directa a Neon (driver PostgreSQL nativo) para leer memorias
- Construir contexto y pasarlo al agente al lanzar

**Persistencia:**

- `projects.json` — configuración local de proyectos por máquina
- Neon (PostgreSQL) — lectura de memorias compartidas (extensible a write futuro)

**Configuración:**

- Archivo `.env` local con `NEON_DATABASE_URL`
- Token simple para auth futura (opcional en MVP)

## 8. Comandos Tauri (IPC)

La app expone los siguientes comandos IPC (equivalente a endpoints en web):

| Comando | Descripción |
|---------|-------------|
| `get_projects` | Lista proyectos desde `projects.json` |
| `create_project` | Crea un nuevo proyecto |
| `update_project` | Edita un proyecto existente |
| `delete_project` | Elimina un proyecto |
| `get_memories(project_id)` | Obtiene memorias de Neon |
| `get_project_status(project_id)` | Última actividad, agente, timestamp |
| `open_editor(project_id, editor)` | Abre editor (vscode, cursor) |
| `launch_agent(project_id, agent)` | Lanza agente (claude, opencode, qwen) |
| `continue_work(project_id, agent)` | Lee Neon + lanza agente con contexto |

## 9. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| `child_process` en WSL requiere prefijo `wsl` en comandos | Capa de abstracción por `env` del proyecto |
| Mapeo de rutas WSL ↔ Windows para editores | `wslpath -w` o tabla de mapeo por proyecto |
| Queries lentas a Neon afectan la carga del dashboard | Indexar por `project_id + timestamp` |
| Credenciales de Neon expuestas en `.env` local | Limitar permisos del usuario de DB, acceso solo lectura en MVP |
| Compilación cross-platform de Tauri | Tauri soporta Windows, Mac, Linux nativamente — un codebase, 3 builds |
| Scope creep | Evitar features no críticas (auto-scan, métricas, terminal embebido, auth compleja) |

## 10. Roadmap

| Fase | Descripción |
|------|-------------|
| **Fase 1 — Skeleton** | Setup Tauri + React, CRUD de proyectos en `projects.json`, conexión lectura a Neon, renderizar lista de proyectos |
| **Fase 2 — Acciones locales** | Comandos para abrir VSCode/Cursor, lanzar 3 agentes en terminal externo con `child_process`, frontend con botones de acción |
| **Fase 3 — Continue Work** | Integración completa: leer Neon → construir contexto → lanzar agente con contexto inyectado en terminal externo |
| **Fase 4 — Dashboard completo** | Status cards con última actividad, timestamps, resúmenes desde Neon |
| **Fase 5 — Polish** | Tema visual (The Command Matrix), atajos de teclado, manejo de errores, compilación cross-platform, soporte write a Neon (opcional) |

## 11. Criterios de éxito

El sistema es exitoso si:

- Puedes abrir cualquier proyecto en < 2 clicks
- Puedes retomar contexto sin pensar (Continue Work funciona sin fricción)
- Reduces fricción al cambiar de proyecto
- Usas el dashboard diariamente
- Tu socio puede usarlo sin explicación previa
- La app funciona nativamente en Windows, WSL y Mac sin configuración manual compleja
