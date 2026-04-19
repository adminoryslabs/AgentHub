---
story_key: 3-1-design-tokens-layout-base
status: done
epic: 3
epic_title: Polish Visual — The Command Matrix
---

# Story 3.1: Design Tokens + Layout Base

## Story

As a user,
I want a consistent, high-density dark UI,
So that the app feels like a professional command center.

## Acceptance Criteria

**Given** la app se renderiza
**When** se aplican los estilos globales
**Then** el background principal es `#101419`
**And** las tarjetas usan `surface-container-low`
**And** las zonas interactivas usan `surface-container-highest` en hover
**And** no hay bordes sólidos de 1px para separar secciones (No-Line Rule)
**And** el padding máximo es 12-16px, radius 0.25rem

**Given** hay texto de encabezado
**When** se renderiza
**Then** usa Space Grotesk
**And** el texto de UI usa Inter
**And** los inputs y datos técnicos usan JetBrains Mono

**Given** hay un contenedor que necesita perímetro
**When** se aplica "Ghost Border"
**Then** usa `outline-variant` (#424754) al 15% de opacidad

## Tasks/Subtasks

- [x] Task 1: Configurar design tokens en Tailwind
  - [x] Subtask 1.1: Paleta semanticamente nombrada en `tailwind.config.js`
  - [x] Subtask 1.2: Familias `headline`, `ui` y `mono` registradas
  - [x] Subtask 1.3: Escala compacta de spacing (`compact`, `tight`, `card`)
  - [x] Subtask 1.4: Border radius por defecto en `0.25rem`
- [x] Task 2: Aplicar tokens a componentes base
  - [x] Subtask 2.1: `App.tsx` usa `bg-surface`
  - [x] Subtask 2.2: `ProjectCard` usa `card`, `btn-ghost` y layout denso
  - [x] Subtask 2.3: `AddProjectDialog` usa `input-field` y `dialog-*`
  - [x] Subtask 2.4: `ProjectList` usa toolbar y grid con espaciado compacto
  - [x] Subtask 2.5: Toasts y dialogs comparten tokens globales
- [x] Task 3: Consolidar utilidades visuales
  - [x] Subtask 3.1: `globals.css` define `card`, `ghost-border`, `btn-*`, `input-field`, `dialog-*`
  - [x] Subtask 3.2: Se eliminan hardcodes principales de layout y superficies
- [x] Task 4: Verificar build
  - [x] Subtask 4.1: La app compila y los tokens quedan consumidos por frontend

## Dev Notes

### Design System — "The Command Matrix" (DESIGN.md)

**Colores:**
```
background/surface:        #101419
primary (success/action):  #00e475
secondary (information):   #adc6ff
tertiary (warning):        #ffb95f
error:                     #ef4444
outline-variant:           #424754

surface-container-lowest:  #0d1117
surface-container-low:     #141920
surface-container:         #1a2030
surface-container-high:    #202840
surface-container-highest: #283050
```

**Tipografía (Google Fonts):**
- Space Grotesk → headlines, títulos
- Inter → UI text, labels
- JetBrains Mono → inputs, código, datos técnicos

**No-Line Rule:** Prohibido bordes sólidos de 1px para separar secciones. Usar shifts de background. Ghost borders solo cuando sea necesario, al 15% opacidad.

**Alta densidad:** Padding máximo 12-16px, radius 0.25rem, sin "marketing air".

### Tailwind config approach
En vez de colores arbitrarios (`bg-[#101419]`), usar clases semánticas:
```js
// tailwind.config.js
theme: {
  extend: {
    colors: {
      surface: '#101419',
      'surface-low': '#0d1117',
      'surface-lowest': '#080b10',
      'surface-high': '#141920',
      'surface-active': '#1a2030',
      'surface-hover': '#202840',
      primary: '#00e475',
      secondary: '#adc6ff',
      tertiary: '#ffb95f',
      error: '#ef4444',
      outline: '#424754',
    }
  }
}
```

### No hacer en esta story
- ❌ No agregar nuevas features funcionales
- ❌ No implementar Glow Tags ni Subtle Glow (Story 3.2)
- ✅ Solo refactor visual — reemplazar colores hardcoded por tokens

## File List

| File | Action |
|------|--------|
| `tailwind.config.js` | Modified (design tokens semanticos) |
| `src/styles/globals.css` | Modified (component classes base) |
| `src/App.tsx` | Modified (surface token en app shell) |
| `src/components/ProjectCard.tsx` | Modified (card/actions con tokens) |
| `src/components/ProjectList.tsx` | Modified (grid/toolbar con tokens) |
| `src/components/AddProjectDialog.tsx` | Modified (inputs/dialog con tokens) |
| `src/components/TopBar.tsx` | Modified (header alineado al design system) |
| `src/contexts/UIContext.tsx` | Modified (toasts con clases semanticas) |

## Change Log

- Design tokens de color, tipografia, spacing y radius centralizados en Tailwind
- Clases reutilizables en `globals.css` para cards, botones, inputs, dialogs y toasts
- App shell y componentes principales migrados a tokens semanticos
- Story 3.1 y 3.2 terminaron quedando muy acopladas y se implementaron casi en la misma pasada

## Dev Agent Record

### Implementation Plan
1. Definir tokens visuales en Tailwind
2. Crear utilidades visuales compartidas en `globals.css`
3. Migrar layout base y componentes al nuevo vocabulario visual
4. Verificar build del frontend

### Completion Notes
- La mayor parte del trabajo de esta story quedo consolidada en `tailwind.config.js` y `globals.css`
- El layout base ya no depende de colores inline para superficies y estados comunes
- Parte del polish de status/buttons se documento despues como 3.2, pero ya estaba codificado junto con esta base

### Debug Log
- Sin incidencias relevantes registradas en este artifact
