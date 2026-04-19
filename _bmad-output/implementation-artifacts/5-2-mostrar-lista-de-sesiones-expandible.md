---
story_key: 5-2-mostrar-lista-de-sesiones-expandible
status: done
epic: 5
epic_title: Historial de Sesiones
---

# Story 5.2: Mostrar lista de sesiones expandible en ProjectCard

## Story

As a user,
I want to see agent sessions grouped by agent in an expandable section of each project card,
So that I can understand my work patterns across different agents.

## Tasks/Subtasks

- [x] Task 1: Crear `SessionHistory`
  - [x] Subtask 1.1: Carga lazy al expandir
  - [x] Subtask 1.2: Estado loading y empty state
- [x] Task 2: Agrupar sesiones por agente
  - [x] Subtask 2.1: Secciones por agente con contador
  - [x] Subtask 2.2: Orden heredado desde backend por `modifiedAt DESC`
- [x] Task 3: Mostrar metadata util
  - [x] Subtask 3.1: Fecha relativa
  - [x] Subtask 3.2: Tamano formateado
  - [x] Subtask 3.3: Link de settings por agente

## Dev Notes

- La UI actual muestra Claude y Qwen segun el discovery real disponible.
- El copy `No agent sessions found` quedo implementado tal cual.

## File List

| File | Action |
|------|--------|
| `src/components/SessionHistory.tsx` | Created (UI expandible de sesiones) |
| `src/components/ProjectCard.tsx` | Modified (integra `SessionHistory`) |

## Change Log

- Historial expandible por proyecto con carga diferida
- Agrupacion por agente, fecha relativa y tamano
- Acceso directo a settings desde el historial
