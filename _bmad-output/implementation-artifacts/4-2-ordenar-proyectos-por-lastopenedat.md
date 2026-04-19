---
story_key: 4-2-ordenar-proyectos-por-lastopenedat
status: done
epic: 4
epic_title: Mejoras de Usabilidad
---

# Story 4.2: Ordenar proyectos por `lastOpenedAt`

## Story

As a user,
I want my most recently used projects to appear first,
So that I can find my active work without scrolling.

## Tasks/Subtasks

- [x] Task 1: Ordenar proyectos en frontend
  - [x] Subtask 1.1: `useMemo` con sort por `lastOpenedAt DESC`
  - [x] Subtask 1.2: Proyectos sin timestamp quedan al final

## Dev Notes

- La ordenacion vive en `ProjectList.tsx` y se aplica antes de filtrar.

## File List

| File | Action |
|------|--------|
| `src/components/ProjectList.tsx` | Modified (sort por `lastOpenedAt`) |

## Change Log

- El dashboard prioriza proyectos abiertos recientemente
