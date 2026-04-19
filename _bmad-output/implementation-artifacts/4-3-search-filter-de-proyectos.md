---
story_key: 4-3-search-filter-de-proyectos
status: done
epic: 4
epic_title: Mejoras de Usabilidad
---

# Story 4.3: Search/Filter de proyectos

## Story

As a user,
I want to search projects by name, path, or tags,
So that I can find a specific project quickly when I have many.

## Tasks/Subtasks

- [x] Task 1: Agregar estado de busqueda en `ProjectList`
  - [x] Subtask 1.1: `searchQuery` controlado por input
- [x] Task 2: Filtrar resultados en tiempo real
  - [x] Subtask 2.1: Match por nombre
  - [x] Subtask 2.2: Match por path
  - [x] Subtask 2.3: Match por tags
- [x] Task 3: Mejorar UX del filtro
  - [x] Subtask 3.1: Count de resultados
  - [x] Subtask 3.2: Boton para limpiar busqueda
  - [x] Subtask 3.3: Empty state cuando no hay matches

## File List

| File | Action |
|------|--------|
| `src/components/ProjectList.tsx` | Modified (toolbar de busqueda y filtro) |

## Change Log

- Filtro en tiempo real por nombre, ruta y tags
- Contador de resultados y accion de limpieza
- Mensaje de estado vacio para queries sin matches
