---
date: 2024-12-15
status: draft
target_command: create_plan
---

# Recipes Page Redesign

## Goal

Redesign the recipes page to add ingredient filtering, multiple organization views, and a split-screen detail panel—bringing it to feature parity with the meal plan page's organizational capabilities while improving recipe browsing UX.

## Background

The current recipes page is a simple grid of recipe cards with basic name/tag search. The meal plan page has more sophisticated organization (View by: Category/Recipe toggle). Users want similar filtering and organization on the recipes page, plus a split-screen detail view instead of navigating to a separate page.

This is currently a UI mockup—no backend/database exists yet. This work focuses on frontend implementation with mock data.

## Requirements

### Must Have

**Filtering**
- 3 autocomplete input boxes for ingredient filtering
- Autocomplete searches across all recipe ingredients
- AND logic: recipes must contain ALL selected ingredients
- Filters work alongside existing name/tag search

**Organization Views**
- "View by" toggle similar to meal plan's shopping list
- Grouping options: Tag category, Protein, Starch (pastas, rices, potatoes)
- Default view remains the current ungrouped grid

**Split-Screen Detail Panel**
- Clicking a recipe opens detail panel on the right side
- Recipe list shrinks (fewer columns) to accommodate panel
- Clicking another recipe swaps panel content (no close/reopen)
- Close button returns to full-width recipe grid
- Panel shows full recipe information (title, description, image, time, servings, tags, ingredients, instructions)

**Detail Panel Actions**
- Edit recipe button (opens recipe editing UI)
- Add to meal plan button
- Close panel button

**Recipe Creation/Editing**
- UI changes to recipe creation/editing as needed for the new design

### Out of Scope

- Import from URL functionality changes
- Meal plan page changes
- Backend/Rust implementation
- SQLite schema changes
- Any data persistence (mock data only)

## Affected Areas

- **Users/Personas**: Anyone browsing or managing recipes
- **Systems/Components**:
  - `src/lib/components/Recipes.svelte` (or recipes/ directory)
  - `src/lib/stores/recipes.ts`
  - Recipe-related type definitions
  - Possibly shared components for autocomplete, view toggles
- **Data**: Mock recipe data with structured ingredients for filtering

## Vertical Slice Analysis

**Layers involved**: UI only (Svelte components, stores, mock data)

**Decomposition approach**: Component-by-component

**Rationale**: Since this is UI-only with mock data, we can build and manually test each component in isolation:
1. Ingredient autocomplete filter component
2. View by toggle and grouping logic
3. Split-screen layout with detail panel
4. Detail panel content and actions
5. Integration and polish

Each piece can be visually verified as built.

## Success Criteria

### Manual Verification
- [ ] 3 ingredient autocomplete boxes filter recipes correctly (AND logic)
- [ ] View by toggle switches between: Default, Tag category, Protein, Starch groupings
- [ ] Clicking recipe opens detail panel on right, list shrinks to fewer columns
- [ ] Clicking different recipe swaps panel content without closing
- [ ] Close button returns to full-width grid
- [ ] Detail panel shows complete recipe info
- [ ] Edit button opens recipe editing UI
- [ ] Add to meal plan button is present and functional
- [ ] Responsive behavior is reasonable

## Open Questions for Planning

- What is the current component structure in `src/lib/components/recipes/`?
- How does the meal plan page implement its "View by" toggle? Can we reuse patterns?
- What mock data structure exists for recipes? Does it include structured ingredients?
- Are there existing autocomplete or dropdown components to leverage?
- What does the current recipe editing UI look like?

## Constraints

- Frontend only—no Rust/backend changes
- Must use existing tech stack (Svelte 5, TypeScript, Tailwind CSS v4)
- Should follow existing code patterns in the codebase

---

**To execute**: `/create_plan ai_docs/prompts/2024-12-15-recipes-page-redesign.md`
