---
story_key: 3-1-design-tokens-layout-base
status: ready-for-dev
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

- [ ] Task 1: Configurar design tokens en Tailwind
  - [ ] Subtask 1.1: Definir colores en `tailwind.config.js` (background, primary, secondary, tertiary, outline-variant, surface variants)
  - [ ] Subtask 1.2: Configurar fonts en `globals.css` (Space Grotesk, Inter, JetBrains Mono via Google Fonts)
  - [ ] Subtask 1.3: Configurar spacing scale compacto
  - [ ] Subtask 1.4: Configurar border radius default = 0.25rem
- [ ] Task 2: Aplicar tokens a componentes existentes
  - [ ] Subtask 2.1: `App.tsx` header → usar tokens en vez de colores hardcoded
  - [ ] Subtask 2.2: `ProjectCard` → surface-container-low, ghost borders, hover states
  - [ ] Subtask 2.3: `AddProjectDialog` → inputs con tokens, focus states
  - [ ] Subtask 2.4: `ProjectList` → grid spacing con tokens
  - [ ] Subtask 2.5: `UIContext` toasts → colores de status con tokens
- [ ] Task 3: Eliminar colores hardcoded
  - [ ] Subtask 3.1: Reemplazar todos los `bg-[#101419]`, `text-[#adc6ff]`, etc. por clases de Tailwind tokens
  - [ ] Subtask 3.2: Verificar que no queden colores inline excepto donde sea intencional
- [ ] Task 4: Verificar build
  - [ ] Subtask 4.1: `npm run tauri build` exitoso

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

<!-- To be filled -->

## Change Log

<!-- To be filled -->

## Dev Agent Record

### Implementation Plan
<!-- To be filled -->

### Completion Notes
<!-- To be filled -->

### Debug Log
<!-- To be filled -->
