# Design System Document: The Command Matrix

## 1. Overview & Creative North Star
This design system is engineered for the high-performance developer. It rejects the airy, "lifestyle" aesthetics of modern SaaS in favor of **The Command Matrix**—a Creative North Star defined by data density, terminal-inspired precision, and observability-first architecture. 

We break the "template" look by utilizing intentional asymmetry and a "HUD" (Heads-Up Display) mentality. Instead of wide gutters and oversized margins, we embrace a compact, information-rich environment where every pixel serves a functional purpose. The experience should feel like a sophisticated cockpit: dark, focused, and vibrating with live data.

## 2. Colors
Our palette is rooted in a deep, void-like foundation, punctuated by high-chroma status accents that cut through the darkness.

### Functional Palette (Material Design Convention)
- **Background/Surface:** `#101419` (The primary canvas)
- **Primary (Success/Action):** `#00e475` (Vibrant green for stable states)
- **Secondary (Information):** `#adc6ff` (Soft blue for neutral telemetry)
- **Tertiary (Warning):** `#ffb95f` (Amber orange for caution/active load)
- **Outline Variant:** `#424754` (Used for ghost-borders)

### The "No-Line" Rule
Standard UI relies on 1px solid borders to separate sections. In this system, **1px solid borders are prohibited for sectioning.** Boundaries must be defined through:
1. **Background Shifts:** A `surface-container-low` section sitting on a `surface` background.
2. **Tonal Transitions:** Using `surface-container-highest` to draw the eye to interactive zones.
3. **Negative Space:** Trusting the Spacing Scale to create "invisible" columns.

### Signature Textures
To avoid a flat, "Bootstrap-dark" feel, apply a **Subtle Glow** to primary status indicators. Use the `surface_tint` at 10% opacity with a 12px blur behind critical data points to simulate the phosphorus glow of a vintage CRT monitor.

## 3. Typography
We use a dual-font strategy to balance editorial authority with technical clarity.

- **Display & Headlines:** **Space Grotesk.** This font brings a "tech-industrial" personality. Use `display-lg` (3.5rem) for hero metrics to create a striking contrast against micro-data.
- **UI & Technical Data:** **Inter** (or **JetBrains Mono** for code-specific strings). Use `label-sm` (0.6875rem) for secondary metadata. High density is achieved by keeping body text at `body-md` (0.875rem), ensuring we fit more information per viewport.

**Hierarchy Strategy:** 
Large, thin headlines (`headline-lg`) should sit immediately adjacent to tiny, all-caps labels (`label-md`). This "High-Low" pairing is the hallmark of professional observability tools.

## 4. Elevation & Depth
Depth in this system is achieved through **Tonal Layering** rather than traditional drop shadows.

- **The Layering Principle:** 
    - Base Level: `surface`
    - Inset/Deep Elements (Terminal inputs): `surface-container-lowest`
    - Raised Elements (Cards): `surface-container-low`
    - Active/Hovered Elements: `surface-container-high`
- **The "Ghost Border" Fallback:** If a container requires a perimeter for accessibility, use the `outline-variant` token at **15% opacity**. This creates a "barely there" frame that feels like light reflecting off an edge rather than a drawn line.
- **Glassmorphism:** For floating menus or command palettes, use `surface_container` with a `20px` backdrop-blur and 60% opacity. This allows the high-density data underneath to remain visible, maintaining the "HUD" feel.

## 5. Components

### Buttons
- **Primary:** Filled with `primary_container`. Text in `on_primary_container`. No rounded corners; use `sm` (0.125rem) or `none`.
- **Secondary:** Transparent background with a "Ghost Border."
- **Tertiary:** Text-only, using `primary` color, strictly in `label-md` for a "system link" look.

### Compact Cards
Forbid divider lines. Use `surface-container-low` for the card body and `surface-container-highest` for the header strip. 
- **Radius:** Strictly `DEFAULT` (0.25rem).
- **Padding:** Tight. `12px` to `16px` max.

### Status Chips
Chips should not have backgrounds. They should be "Glow Tags": a small 6px dot of the status color (`primary`, `tertiary`, or `error`) followed by `label-sm` text.

### Input Fields
- **Background:** `surface_container_lowest`.
- **Focus State:** No thick rings. Change the "Ghost Border" opacity from 15% to 80% using the `primary` color.
- **Font:** Always use `JetBrains Mono` for inputs to reinforce the developer-centric nature of the tool.

### Data Bars (Progress)
Horizontal bars should be thin (4px). Use a "Soft Glow" on the filled portion of the bar to indicate "live" energy.

## 6. Do's and Don'ts

### Do:
- **Do** lean into asymmetry. A 3-column layout where the middle column is twice as wide as the others creates a custom, "command center" feel.
- **Do** use `label-sm` for units (e.g., "ms", "kb/s") positioned immediately after large numbers.
- **Do** utilize `surface-container-lowest` for background "wells" where code or logs are displayed.

### Don't:
- **Don't** use large, friendly icons. Use small, 16px geometric icons with thin strokes.
- **Don't** use standard "Drop Shadows." They feel muddy in a high-density dark UI. Use background color shifts instead.
- **Don't** add "Marketing Air." Avoid large gaps of empty space. If a section looks empty, consider if more telemetry or metadata can be surfaced.
- **Don't** use 100% opaque, high-contrast white text for everything. Use `on_surface_variant` for labels to keep the visual noise low.