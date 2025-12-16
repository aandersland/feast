---
date: 2025-12-16T12:00:00-05:00
status: draft
target_command: create_plan
source: ai_docs/research/2025-12-15-development-roadmap.md (Phase 3)
---

# Frontend-Backend Integration

## Goal

Replace all mock data in Svelte stores with real Tauri backend calls, enabling data persistence across sessions. Include supporting UX infrastructure (toasts, loading states, confirmation dialogs).

## Background

The Feast app has a complete mocked frontend UI with 5 Svelte stores using in-memory writables. Phase 1 (database schema) and Phase 2 (Tauri commands) are complete per commit `08a1745`. Tauri IPC wrappers exist in `src/lib/tauri/commands.ts` but are unused. Zero frontend store tests exist currently.

## Requirements

### Must Have

**Store Integrations (in order):**
1. Recipe Store - wire `recipeStore` to recipe CRUD commands
2. Meal Plan Store - wire `mealPlanStore` to meal plan commands
3. Shopping List Store - wire `weeklyShoppingListsStore` to shopping list commands
4. Quick Lists Store - wire `quickListsStore` to quick list commands
5. Manual Items Store - wire `manualItemsStore` to manual item commands

**UX Infrastructure:**
- Toast notification system (simple custom Svelte component)
- Loading spinners/states during data fetches
- Confirmation dialogs for destructive actions (deletes)
- Empty state handling for first-launch/empty database scenarios

**Testing:**
- Automated tests for each store's operations
- Tests for Tauri IPC wrappers with mocked invoke

### Out of Scope

- Feature flag to toggle between mock and real backend (clean cutover)
- Component-level tests (per roadmap: LOW priority)
- Recipe import from URL feature
- Performance optimizations (pagination, virtual scrolling)
- Offline support / sync

## Affected Areas

- **Systems/Components**:
  - `src/lib/stores/` - all 5 stores
  - `src/lib/tauri/commands.ts` - IPC wrappers
  - `src/lib/components/` - add loading/error states to existing components
  - New: toast system, confirmation dialog components
- **Data**: All entities (recipes, meal plans, shopping lists, quick lists, manual items) transition from in-memory to SQLite persistence

## Vertical Slice Analysis

**Layers involved**:
- Svelte stores (state management)
- Tauri IPC layer (already exists, needs wiring)
- UI components (loading states, toasts, dialogs)
- Backend commands (already complete)

**Decomposition approach**: Feature-by-feature (one store at a time)

**Rationale**:
- Each store is relatively independent
- Allows incremental testing and verification
- Recipes should be first since Meal Plans and Shopping Lists depend on recipe data
- UX infrastructure (toasts, loading states) should be built first as shared foundation

**Suggested order**:
1. UX infrastructure (toasts, loading states, confirmation dialogs)
2. Recipe Store integration + tests
3. Meal Plan Store integration + tests
4. Shopping List Store integration + tests
5. Quick Lists Store integration + tests
6. Manual Items Store integration + tests

## Success Criteria

### Automated Verification
- [ ] All store operations have passing tests
- [ ] Tauri IPC wrappers have tests with mocked invoke
- [ ] `pnpm test` passes
- [ ] `pnpm check` passes (type checking)

### Manual Verification
- [ ] Create a recipe, close app, reopen - recipe persists
- [ ] Add recipe to meal plan, verify it appears on calendar
- [ ] Generate shopping list from meal plan, check/uncheck items persist
- [ ] Create quick list, add items, add to shopping list
- [ ] Add manual shopping items, verify persistence
- [ ] Delete operations show confirmation dialog
- [ ] Loading spinners appear during data fetch
- [ ] Error toast appears on failed operation
- [ ] Empty states display appropriately on fresh database

## Open Questions for Planning

- What is the current structure of Tauri IPC wrappers in `commands.ts`?
- How do existing stores handle derived stores (`recipeById`, `mealPlanByDate`, etc.) - will these need updates?
- What's the pattern for async operations in Svelte 5 stores?
- Where should the toast/notification component live in the component hierarchy?

## Constraints

- Use simple custom Svelte component for toasts (no external library)
- Clean cutover from mock data (remove mock data entirely)
- Follow existing code patterns in the codebase

---

**To execute**: `/create_plan ai_docs/prompts/2025-12-16-frontend-backend-integration.md`
