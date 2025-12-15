---
date: 2024-12-14
status: draft
target_command: create_plan
---

# Shopping List Redesign - Merge into Meal Plan Page

## Goal

Redesign the shopping list experience by merging it into the Meal Plan page, enabling multi-list support per week and improving information organization through a category-based card layout.

## Background

The current Shopping List page has several UX issues:
- Assumes only one list exists (no support for multiple lists per week)
- Poor information density requiring excessive vertical scrolling
- Quick Lists sidebar takes up valuable horizontal space
- No connection between meal planning and shopping despite being related workflows

Users need mid-week or end-of-week shopping runs (e.g., picking up fresh chicken later in the week), requiring multiple lists per week tied to their meal plan.

## Requirements

### Must Have

**Page Structure**
- Remove standalone "Shopping List" tab from navigation
- Merge shopping functionality into Meal Plan page
- Keep "Quick Lists" tab separate for managing quick list templates
- Shopping section spans full width below the meal plan calendar

**Multi-List Support**
- Tabbed interface for lists: Weekly (main) | Mid-week | Custom lists
- Lists automatically tied to the currently viewed week
- Ability to create additional lists for the week as needed
- Move items between lists via clickable icon on each item

**Layout & Organization**
- Collapsible left sidebar for Quick Lists (collapsed by default)
- Multi-column/row grid layout using category cards
- Standard grocery categories: Produce, Dairy, Meat, Pantry, Frozen, etc.
- Alternative view: organize by source recipe instead of category
- Category view is the default

**Item Display**
- Item name
- Quantity with both imperial and metric (e.g., "14oz / 400g")
- Source recipe visible on each item

**Week Navigation**
- Leverage existing Meal Plan week navigation/calendar
- Shopping lists follow the selected week
- Can view/plan future weeks

### Out of Scope
- Data migration (using mock data only)
- Consistency updates to other pages (deferred)
- User-defined custom categories
- Drag-and-drop between lists (use icon click instead)

## Affected Areas

- **Users/Personas**: Anyone planning meals and creating shopping lists
- **Systems/Components**:
  - `src/lib/components/ShoppingList.svelte` - to be deprecated/removed
  - `src/lib/components/MealPlan.svelte` - major additions
  - `src/lib/components/QuickListsManager.svelte` - may need adjustments
  - `src/lib/stores/shoppingList.ts` - needs multi-list support
  - `src/lib/stores/mealPlan.ts` - integration with shopping
  - Navigation/tab configuration
- **Data**: Shopping list items now associated with specific weeks and list types

## Vertical Slice Analysis

**Layers involved**: UI (Svelte components), State (Svelte stores)

**Decomposition approach**: Front-to-back / UI-first

**Rationale**: This is primarily a UI/UX redesign with mock data. The focus is on layout, component structure, and interaction patterns. Store changes support the UI needs. No backend/database changes required for this phase.

## Success Criteria

### Automated Verification
- [ ] `pnpm check` passes (TypeScript/Svelte compilation)
- [ ] `pnpm lint` passes
- [ ] `pnpm test` passes (existing tests don't break)

### Manual Verification
- [ ] Navigate to Meal Plan page
- [ ] See week calendar at top (unchanged)
- [ ] See shopping section below with tabbed lists (Weekly | Mid-week)
- [ ] View items organized by category cards in multi-column grid
- [ ] Toggle to "by recipe" view - items regroup by source recipe
- [ ] Click icon on item to move it to different list (e.g., Weekly → Mid-week)
- [ ] Expand/collapse Quick Lists sidebar
- [ ] Add items from Quick List to shopping list
- [ ] Navigate to different week - shopping lists update accordingly
- [ ] Confirm Shopping List tab is removed from main navigation
- [ ] Confirm Quick Lists tab still exists and works for template management

## Open Questions for Planning

- Current structure of MealPlan.svelte - how is the calendar implemented?
- Current store structure for shopping list - what refactoring is needed for multi-list?
- How is navigation/tabs configured - where to remove Shopping List tab?
- Are there shared components that can be reused for the card layout?

## Constraints

- Mock data only - no backend integration required
- Must work with existing Meal Plan calendar/week navigation
- Maintain Svelte 5 + TypeScript + Tailwind CSS v4 patterns

---

**To execute**: `/create_plan ai_docs/prompts/2024-12-14-shopping-list-redesign.md`
