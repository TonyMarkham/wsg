# Design System Specification: Technical Precision & Tonal Depth

## 1. Overview & Creative North Star
### The Creative North Star: "The Orchestrated Terminal"
This design system rejects the "boxed-in" nature of traditional web interfaces. Instead, it draws inspiration from high-end IDEs and mechanical instrumentation. The goal is to create a digital environment that feels like a precision-engineered tool—dense with information yet impeccably organized. 

We break the "template" look through **Intentional Asymmetry** and **Tonal Depth**. By moving away from rigid grids and 1px borders, we allow content to breathe through subtle shifts in luminosity. The interface should feel like a single, cohesive piece of hardware where elements are etched into the surface rather than pasted on top.

---

## 2. Colors: Tonal Architecture
The palette is built on a foundation of deep, obsidian blues and charcoals, punctuated by "Electric Status" accents.

### The "No-Line" Rule
**Explicit Instruction:** You are prohibited from using 1px solid borders to section off major UI areas. Boundaries must be defined solely through background color shifts. For instance, a side navigation should use `surface_container_low` sitting against a `surface` background. This creates a more sophisticated, "machined" aesthetic.

### Surface Hierarchy & Nesting
Treat the UI as a physical stack of materials. Use the `surface_container` tiers to denote importance:
- **Baseline:** `surface` (#060e20) for the primary application background.
- **Recessed:** `surface_container_low` (#091328) for persistent sidebars or utility panels.
- **Elevated:** `surface_container_high` (#141f38) for active content areas or code editors.
- **Peak:** `surface_container_highest` (#192540) for floating modals or active popovers.

### The "Glass & Gradient" Rule
To inject "visual soul," use **Glassmorphism** for floating elements (e.g., Command Palettes or Tooltips). Apply `surface_container_highest` at 80% opacity with a `20px` backdrop-blur. 

**Signature Gradients:** For primary actions, transition from `primary` (#5bfc9a) to `primary_container` (#05cb6f) at a 135-degree angle. This provides a tactile, "lit" feel that flat hex codes cannot replicate.

---

## 3. Typography: Technical Editorial
We utilize a dual-typeface system to balance high-density data with professional authority.

*   **Display & Headlines (`Space Grotesk`):** Used for large-scale data points and section headers. Its geometric quirks provide a "human-meets-machine" personality.
*   **Interface & Labels (`Inter`):** Our workhorse. Used for all UI controls and body text. It is optimized for screen readability at small sizes.
*   **Data & Code (`JetBrains Mono` - implied for dev tools):** For all technical strings, hashes, and file paths.

**The Hierarchy of Intent:**
- **Display-LG (3.5rem):** High-level metrics (e.g., total commits, uptime).
- **Headline-SM (1.5rem):** Feature titles.
- **Label-SM (0.6875rem):** Metadata and micro-copy. Use `on_surface_variant` (#a3aac4) to reduce visual noise in dense layouts.

---

## 4. Elevation & Depth: Tonal Layering
Traditional drop shadows are too "web-like" for this system. We use **Tonal Layering** to convey height.

### The Layering Principle
Depth is achieved by "stacking" container tiers. To make a card pop, do not add a shadow; instead, place a `surface_container_highest` card on a `surface_container_low` background.

### Ambient Shadows
If a floating effect is required (e.g., a context menu), use an **Ambient Shadow**:
- `box-shadow: 0 12px 40px rgba(0, 0, 0, 0.5);`
- The shadow color should never be pure black; it should be a deep navy tint derived from `surface_container_lowest`.

### The "Ghost Border" Fallback
If accessibility requires a container edge, use the **Ghost Border**: Use `outline_variant` (#40485d) at **15% opacity**. This creates a suggestion of a boundary without cluttering the scan path.

---

## 5. Components

### Buttons
- **Primary:** Gradient fill (`primary` to `primary_container`), `on_primary` text. Border-radius: `md` (0.375rem).
- **Secondary:** Surface-tinted. Background: `secondary_container`. Text: `on_secondary_container`.
- **Tertiary:** No background. Text: `secondary`. On hover, apply a `10%` opacity fill of `secondary`.

### Input Fields
- **Styling:** Use `surface_container_lowest` for the field background to create a "punched-in" look. 
- **States:** Focus state uses a 1px `primary` ghost border (20% opacity) and a subtle `primary` outer glow.

### Cards & Lists
- **The Divider Ban:** Never use horizontal rules (`<hr>`). Separate list items using `8px` of vertical white space or by alternating background tones between `surface_container` and `surface_container_low`.

### Status Accents (The GitKraken Influence)
Use these colors strictly for semantic meaning:
- **Added/Success:** `primary` (#5bfc9a)
- **Modified/Warning:** `tertiary` (#ffb148)
- **Deleted/Error:** `error` (#ff716c)
*Tip: For "Deleted" code blocks, use `error_container` as a background with `on_error_container` text for high-contrast legibility.*

### Data Density Components (New)
- **The "Activity Heatmap":** Use a scale from `surface_variant` (lowest activity) to `primary` (highest activity).
- **Monospace Tags:** For commit SHAs or branch names, use `surface_container_highest` with `label-sm` monospace text.

---

## 6. Do's and Don'ts

### Do
- **Do** embrace high density. This is a tool for experts; don't be afraid to put more data on screen if it's organized by the surface hierarchy.
- **Do** use `JetBrains Mono` for any text that can be copied/pasted into a terminal.
- **Do** use `surface_bright` sparingly for hover states to create a "lighting up" effect.

### Don't
- **Don't** use pure white (#FFFFFF) for text. Always use `on_surface` (#dee5ff) to prevent eye strain in dark mode.
- **Don't** use standard "Rounded" buttons (9999px). Stick to the `md` (0.375rem) or `sm` (0.125rem) scale to maintain a technical, architectural feel.
- **Don't** use 1px dividers. If you feel the need for a line, try a 4px gap of background color instead.

---

## 7. Roundedness Scale
- **DEFAULT:** `0.25rem` (Internal components)
- **MD:** `0.375rem` (Buttons, Input fields)
- **LG:** `0.5rem` (Cards, Main content containers)
- **XL:** `0.75rem` (Modals, Large overlays)