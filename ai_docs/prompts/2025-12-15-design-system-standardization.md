---
date: 2025-12-15
status: planned
target_command: create_plan
---

# Design System Standardization

## Goal

Apply a consistent design theme across all app pages based on the refined patterns established in the Meal Plan page, which represents the most up-to-date design standards.

## Background

The Meal Plan page has been iterated on and contains the most refined design patterns. Other pages (Dashboard, Recipes, Quick Lists) were built earlier and lack consistency with these newer standards. This prompt documents the design system to enable systematic application across the app.

## Design System Extracted from Meal Plan Page

### 1. Color Palette

**Primary Brand Color: Emerald**
- Primary action: `bg-emerald-600` / `hover:bg-emerald-700`
- Primary text: `text-emerald-600` / `hover:text-emerald-700`
- Primary highlight: `bg-emerald-50` / `bg-emerald-100`
- Focus ring: `focus:ring-emerald-500` / `focus:ring-2`
- Active states: `text-emerald-600` with `bg-emerald-600` underline

**Semantic Colors (Meal Types)**
- Breakfast: `bg-amber-100 text-amber-800`
- Lunch: `bg-blue-100 text-blue-800`
- Dinner: `bg-emerald-100 text-emerald-800`
- Snack: `bg-purple-100 text-purple-800`

**Category Colors (Shopping)**
- Produce: `bg-green-50 border-green-200`
- Meat & Seafood: `bg-red-50 border-red-200`
- Dairy & Eggs: `bg-yellow-50 border-yellow-200`
- Bakery: `bg-amber-50 border-amber-200`
- Pantry: `bg-orange-50 border-orange-200`
- Frozen: `bg-blue-50 border-blue-200`
- Beverages: `bg-cyan-50 border-cyan-200`
- Snacks: `bg-purple-50 border-purple-200`
- Other: `bg-gray-50 border-gray-200`

**Neutral Grays**
- Text primary: `text-gray-800`
- Text secondary: `text-gray-600`
- Text muted: `text-gray-500`
- Text hint: `text-gray-400`
- Borders: `border-gray-100`, `border-gray-200`, `border-gray-300`
- Backgrounds: `bg-gray-50`, `bg-gray-100`
- Dividers: `divide-gray-100`

### 2. Typography

**Headings**
- Page title: `text-xl sm:text-2xl font-bold text-gray-800`
- Section title: `text-lg sm:text-xl font-bold text-gray-800`
- Card title: `font-semibold text-gray-800 text-sm sm:text-base`
- Modal title: `text-lg font-semibold text-gray-800`

**Body Text**
- Primary: `text-sm sm:text-base text-gray-600`
- Secondary: `text-sm text-gray-500`
- Hint: `text-xs text-gray-400`
- Uppercase label: `text-xs uppercase font-medium`

### 3. Spacing & Layout

**Container**
- Max width large: `max-w-[1800px] 3xl:max-w-[2400px]`
- Max width standard: `max-w-6xl`
- Max width narrow: `max-w-4xl`
- Padding: `px-2 sm:px-4 2xl:px-6`
- Center: `mx-auto`

**Component Spacing**
- Section gap: `mb-6`, `mt-8`
- Card padding: `p-4`, `p-6`
- Item padding: `px-3 sm:px-4 py-2 sm:py-3`
- Flex gaps: `gap-2`, `gap-3 sm:gap-4`

**Responsive Breakpoints**
- Mobile-first approach
- sm: 640px
- lg: 1024px
- xl: 1280px
- 2xl: 1536px
- 3xl: 2200px (custom)

### 4. Component Patterns

**Cards**
```
bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden
```

**Colored Category Cards**
```
rounded-xl border-2 {colorClass} overflow-hidden
```

**Primary Buttons**
```
px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors
```

**Secondary/Outline Buttons**
```
px-4 py-2 border border-emerald-600 text-emerald-600 rounded-lg hover:bg-emerald-50 transition-colors
```

**Ghost Buttons**
```
p-1.5 sm:p-2 hover:bg-gray-100 rounded-lg transition-colors
```

**Add/Action Buttons (dashed)**
```
w-full py-2 text-sm text-gray-400 hover:text-emerald-600 hover:bg-emerald-50 rounded-lg border border-dashed border-gray-300 transition-colors
```

**Form Inputs**
```
w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500
```

**Tabs/Toggle**
```
px-4 py-2 text-sm font-medium transition-colors relative
Active: text-emerald-600 with absolute bottom-0 h-0.5 bg-emerald-600 underline
Inactive: text-gray-500 hover:text-gray-700 hover:bg-gray-50
```

**Toggle Groups (ViewToggle)**
```
flex rounded-lg border border-gray-200 overflow-hidden
Active: bg-emerald-100 text-emerald-700
Inactive: hover:bg-gray-50
```

**Checkboxes (circular)**
```
w-5 h-5 rounded-full border-2 flex items-center justify-center transition-colors
Checked: bg-emerald-500 border-emerald-500 text-white
Unchecked: border-gray-300 hover:border-emerald-500
```

### 5. Interactive States

**Hover Effects**
- Background highlight: `hover:bg-gray-50`, `hover:bg-emerald-50`
- Text color change: `hover:text-emerald-600`, `hover:text-gray-700`
- Shadow lift: `hover:shadow-md`
- Border highlight: `hover:border-emerald-500`

**Active/Selected States**
- Primary indicator: `bg-emerald-600` underline (2px)
- Ring highlight: `ring-2 ring-emerald-200`
- Background: `bg-emerald-100 text-emerald-700`

**Disabled/Muted States**
- Opacity: `opacity-50`, `opacity-40`
- Strikethrough: `line-through`

**Transitions**
- Standard: `transition-colors`
- All properties: `transition-all duration-200`

### 6. Modal Pattern

**Backdrop**
```
fixed inset-0 bg-black/50 z-50 flex items-center justify-center
```

**Modal Container**
```
relative bg-white rounded-xl shadow-xl max-w-lg w-full mx-4 max-h-[90vh] overflow-auto
```

**Modal Header**
```
flex items-center justify-between px-6 py-4 border-b border-gray-100
```

**Modal Body**
```
p-6
```

**Close Button**
```
w-8 h-8 flex items-center justify-center rounded-lg text-gray-400 hover:text-gray-600 hover:bg-gray-100 transition-colors
```

### 7. Grid Patterns

**Calendar Grid (7 columns)**
```
grid grid-cols-7 divide-x divide-gray-100
```

**Responsive Card Grid**
```
grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3 sm:gap-4
```

**Standard Card Grid**
```
grid grid-cols-2 gap-6
or
grid grid-cols-1 md:grid-cols-2 gap-6
```

### 8. Empty States

```svelte
<div class="text-center py-12 text-gray-500">
  {message}
</div>
```

With icon:
```svelte
<div class="bg-white rounded-xl shadow-sm border border-gray-100 p-8 text-center">
  <svg class="w-12 h-12 mx-auto text-gray-300 mb-4">...</svg>
  <h3 class="text-lg font-medium text-gray-700 mb-2">{title}</h3>
  <p class="text-gray-500 mb-4">{description}</p>
  <button class="...primary button...">{action}</button>
</div>
```

### 9. Section Dividers

```
border-t border-gray-200
mt-8 pt-8 (for major section breaks)
```

### 10. Accessibility Patterns

- Focus visible: `focus-visible:outline-2 focus-visible:outline-emerald-500`
- ARIA labels on icon buttons
- `role="dialog" aria-modal="true"` on modals
- `aria-current="page"` on active tabs
- Semantic heading hierarchy

## Requirements

### Must Have
- Document design tokens as CSS custom properties or Tailwind config
- Identify all components that need updating
- Prioritize consistency in: buttons, cards, typography, spacing, colors

### Out of Scope
- Creating new components not currently in the app
- Dark mode (future consideration)
- Animation/motion design beyond transitions
- Icon system standardization

## Affected Areas

- **Components**: Dashboard, Recipes, QuickListsManager, TabNavigation, all subcomponents
- **Systems**: Tailwind configuration, app.css
- **Data**: None (purely presentational)

## Vertical Slice Analysis

**Layers involved**: UI only (Svelte components, CSS)

**Decomposition approach**: Component-by-component standardization

**Rationale**: Changes are isolated to presentation layer, can be tested visually page-by-page

## Success Criteria

### Manual Verification
- [ ] All pages visually consistent with Meal Plan page standards
- [ ] All buttons use standardized patterns
- [ ] All cards use consistent border-radius, shadows, borders
- [ ] Typography hierarchy consistent across pages
- [ ] Interactive states (hover, active, focus) consistent
- [ ] Responsive behavior consistent across breakpoints

## Open Questions for Planning

- Should we extract reusable component classes or keep inline Tailwind?
- Should we create a shared Button component or document patterns?
- Are there any page-specific design variations that should be preserved?

## Constraints

- Maintain existing functionality while updating styles
- Prefer in-place edits over creating wrapper components
- Keep changes atomic and reviewable per-page

---

**To execute**: `/create_plan ai_docs/prompts/2025-12-15-design-system-standardization.md`
