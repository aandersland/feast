# Design System Standardization Implementation Plan

## Overview

Apply the Meal Plan page's refined design patterns consistently across Dashboard, Recipes, and Quick Lists pages. This is a presentation-layer-only refactoring task that updates Tailwind classes in 20 component files to achieve visual consistency across the app.

## Current State

The Meal Plan page has been iterated on and contains the most refined design patterns. Other pages were built earlier and have:
- Inconsistent typography (missing responsive sizing)
- Non-standard button variants
- Static grid layouts instead of responsive patterns
- Inconsistent focus states on form inputs
- Missing hover/transition effects

**Key Discoveries**:
- Design standard established in `src/lib/components/mealplan/MealPlanCalendar.svelte:63-116`
- Standard card pattern: `bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden`
- Standard primary button: `px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors`
- Standard page title: `text-xl sm:text-2xl font-bold text-gray-800`
- Standard container: `max-w-[1800px] 3xl:max-w-[2400px] mx-auto px-2 sm:px-4 2xl:px-6`
- Standard responsive grid: `grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3 sm:gap-4`

## Desired End State

All pages visually consistent with Meal Plan page standards:
- Responsive typography across all headings
- Consistent button styling (primary, secondary, ghost, dashed-add variants)
- Responsive grid layouts
- Consistent form input focus states (`focus:ring-2 focus:ring-emerald-500`)
- Consistent hover/transition effects
- Standard container widths and padding

## What We're NOT Doing

- Creating new shared component abstractions (keeping inline Tailwind)
- Dark mode support
- Animation/motion design beyond transitions
- Icon system standardization
- Any functionality or logic changes
- Modifying the Meal Plan page (it's the reference)
- Modifying shopping/* components (part of Meal Plan reference)

## Integration Map

| Type | Location | Notes |
|------|----------|-------|
| Entry point | Inline `class=` attributes | Direct Tailwind class edits |
| Registration | N/A | No new registrations |
| Exports | N/A | No new exports |
| Consumers | N/A | Styling only |
| Events | N/A | None required |

## Implementation Approach

Work page-by-page to enable focused visual testing after each phase. Start with shared components (foundation), then TabNavigation (global), then each page in isolation. Each phase can be visually verified independently.

---

## Phase 1: Shared Components

### Goal
Update foundational shared components (Modal, Autocomplete) to ensure consistency in components used across multiple pages.

### Integration Points

**Depends on**: None (foundation)
**Produces for next phase**: Consistent modal and autocomplete styling

**Wiring required**:
- [x] N/A - inline class changes only

### Changes

#### Modal.svelte

**File**: `src/lib/components/shared/Modal.svelte`

**Change**: Add transition classes to close button for consistency

```svelte
<!-- Line 45: Add transition-colors -->
class="w-8 h-8 flex items-center justify-center rounded-lg text-gray-400 hover:text-gray-600 hover:bg-gray-100 transition-colors"
```

#### Autocomplete.svelte

**File**: `src/lib/components/shared/Autocomplete.svelte`

**Change**: Standardize dropdown styling with shadow and transition

```svelte
<!-- Line 69: Update dropdown classes -->
class="absolute z-10 w-full mt-1 bg-white border border-gray-200 rounded-lg shadow-lg max-h-48 overflow-auto"

<!-- Line 75: Add transition to option buttons -->
class="w-full text-left px-3 py-2 hover:bg-emerald-50 capitalize transition-colors"
```

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`

#### Integration Verification
- [ ] N/A - no new exports

#### Manual Verification
- [ ] Modal close button has smooth hover transition
- [ ] Autocomplete dropdown options have smooth hover transition
- [ ] Focus states work correctly on autocomplete input

**Checkpoint**: Pause for manual verification before proceeding to Phase 2.

---

## Phase 2: TabNavigation

### Goal
Ensure main navigation tabs match the established tab pattern from ShoppingListTabs.

### Integration Points

**Consumes from Phase 1**: None directly
**Produces for next phase**: Consistent global navigation

**Wiring required**:
- [x] N/A - inline class changes only

### Changes

#### TabNavigation.svelte

**File**: `src/lib/components/TabNavigation.svelte`

**Change**: Already matches standard. Verify no changes needed.

Current implementation at line 18-26 already uses:
- `text-emerald-600` for active
- `text-gray-500 hover:text-gray-700 hover:bg-gray-50` for inactive
- `h-0.5 bg-emerald-600` underline for active indicator

No changes required - already consistent with ShoppingListTabs pattern.

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`

#### Integration Verification
- [ ] N/A

#### Manual Verification
- [ ] Tab navigation styling matches ShoppingListTabs pattern
- [ ] Active tab has emerald underline
- [ ] Hover states work on inactive tabs

**Checkpoint**: Pause for manual verification before proceeding to Phase 3.

---

## Phase 3: Dashboard Page

### Goal
Update Dashboard and its subcomponents to use standard typography, responsive layouts, and consistent card styling.

### Integration Points

**Consumes from Phase 1**: Modal (if any modals added later)
**Produces for next phase**: None (independent page)

**Wiring required**:
- [x] N/A - inline class changes only

### Changes

#### Dashboard.svelte

**File**: `src/lib/components/Dashboard.svelte`

**Change**: Update container max-width and grid to be responsive

```svelte
<!-- Line 7: Update container -->
<div class="space-y-6 max-w-[1800px] 3xl:max-w-[2400px] mx-auto px-2 sm:px-4 2xl:px-6">

<!-- Line 11: Update grid to be responsive -->
<div class="grid grid-cols-1 lg:grid-cols-2 gap-4 sm:gap-6">
```

#### QuickStats.svelte

**File**: `src/lib/components/dashboard/QuickStats.svelte`

**Change**: Add responsive grid and hover states to stat cards

```svelte
<!-- Line 15: Update grid -->
<div class="grid grid-cols-1 sm:grid-cols-3 gap-3 sm:gap-4">

<!-- Lines 16, 21, 26: Add hover to each card -->
<div class="bg-white rounded-xl shadow-sm border border-gray-100 p-4 hover:shadow-md transition-shadow">
```

#### WeeklyCalendar.svelte

**File**: `src/lib/components/dashboard/WeeklyCalendar.svelte`

**Change**: Update section title to match standard typography

```svelte
<!-- Line 35: Update heading -->
<h2 class="text-lg sm:text-xl font-semibold text-gray-800">This Week</h2>
```

#### RecentRecipes.svelte

**File**: `src/lib/components/dashboard/RecentRecipes.svelte`

**Change**: Update heading and add transition to link

```svelte
<!-- Line 14: Update heading -->
<h2 class="text-lg sm:text-xl font-semibold text-gray-800">Recent Recipes</h2>

<!-- Line 18: Add transition to "View all" link -->
class="text-sm text-emerald-600 hover:text-emerald-700 transition-colors"
```

#### Dashboard.svelte - Quick Actions Card

**File**: `src/lib/components/Dashboard.svelte`

**Change**: Update Quick Actions card heading

```svelte
<!-- Line 16: Update heading -->
<h2 class="text-lg sm:text-xl font-semibold text-gray-800 mb-4">Quick Actions</h2>
```

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`

#### Integration Verification
- [ ] N/A

#### Manual Verification
- [ ] Dashboard layout responsive on mobile/tablet/desktop
- [ ] Stat cards have hover shadow effect
- [ ] Section headings are consistent size
- [ ] Quick Actions buttons have proper hover states
- [ ] "View all" link has smooth color transition

**Checkpoint**: Pause for manual verification before proceeding to Phase 4.

---

## Phase 4: Recipes Page

### Goal
Update Recipes page and all recipe-related subcomponents for consistent styling.

### Integration Points

**Consumes from Phase 1**: Modal.svelte (used by RecipeForm modals)
**Produces for next phase**: None (independent page)

**Wiring required**:
- [x] N/A - inline class changes only

### Changes

#### Recipes.svelte

**File**: `src/lib/components/Recipes.svelte`

**Change**: Update page title, container, and grid layout

```svelte
<!-- Line 128: Update container -->
<div class="max-w-[1800px] 3xl:max-w-[2400px] mx-auto px-2 sm:px-4 2xl:px-6">

<!-- Line 131: Update page title -->
<h1 class="text-xl sm:text-2xl font-bold text-gray-800">Recipes</h1>

<!-- Line 122: Update gridCols derived -->
let gridCols = $derived(isPanelOpen ? "grid-cols-1 lg:grid-cols-2" : "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4");

<!-- Lines 176, 189: Update grids -->
<div class="grid {gridCols} gap-4 sm:gap-6">
```

#### RecipeCard.svelte

**File**: `src/lib/components/recipes/RecipeCard.svelte`

**Change**: Add transition to card hover state

```svelte
<!-- Line 16: Add transition-shadow -->
class="w-full text-left bg-white rounded-xl shadow-sm border overflow-hidden hover:shadow-md transition-shadow
  {isSelected ? 'border-emerald-500 ring-2 ring-emerald-200' : 'border-gray-100'}"
```

#### RecipeDetailPanel.svelte

**File**: `src/lib/components/recipes/RecipeDetailPanel.svelte`

**Change**: Update Edit button to use consistent secondary style

```svelte
<!-- Line 141: Update Edit button to secondary style -->
class="flex-1 px-4 py-2 border border-emerald-600 text-emerald-600 rounded-lg hover:bg-emerald-50 transition-colors"
```

#### RecipeForm.svelte

**File**: `src/lib/components/recipes/RecipeForm.svelte`

**Change**: Update modal heading, add focus rings to ingredient inputs

```svelte
<!-- Line 75: Update heading (remove - it's inside Modal which has title) -->
<!-- DELETE lines 75: The Modal already provides title, this creates duplication -->

<!-- Lines 137-150: Add focus rings to ingredient inputs -->
class="w-20 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
class="w-24 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"

<!-- Lines 180-183: Add focus rings to instruction inputs -->
class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
```

#### ImportRecipe.svelte

**File**: `src/lib/components/recipes/ImportRecipe.svelte`

**Change**: Remove redundant card wrapper (it's inside Modal), update heading

```svelte
<!-- Line 22-24: Remove card wrapper, keep form content -->
<div>
  <p class="text-gray-500 mb-6">
    Paste a link to a recipe from any website. We'll extract the ingredients, instructions, and more.
  </p>
  <!-- ... rest of form ... -->
</div>

<!-- Note: Remove h2 heading as Modal provides title -->
```

#### AddToMealPlanModal.svelte

**File**: `src/lib/components/recipes/AddToMealPlanModal.svelte`

**Change**: Add focus rings to form inputs

```svelte
<!-- Line 45: Add focus ring to date input -->
class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"

<!-- Line 52: Add focus ring to select -->
class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"

<!-- Line 64: Add focus ring to number input -->
class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"

<!-- Line 75: Add transition to Cancel button -->
class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"

<!-- Line 81: Add transition to Add button -->
class="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
```

#### RecipeViewToggle.svelte

**File**: `src/lib/components/recipes/RecipeViewToggle.svelte`

**Change**: Already matches ViewToggle pattern - no changes needed

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`

#### Integration Verification
- [ ] N/A

#### Manual Verification
- [ ] Recipes page layout responsive on all breakpoints
- [ ] Recipe cards have hover shadow effect
- [ ] Edit button uses emerald outline style
- [ ] All form inputs show emerald focus ring
- [ ] Modal forms don't have redundant cards/headings
- [ ] View toggle matches Shopping's ViewToggle style

**Checkpoint**: Pause for manual verification before proceeding to Phase 5.

---

## Phase 5: Quick Lists Page

### Goal
Update QuickListsManager and all quicklists-related subcomponents for consistent styling.

### Integration Points

**Consumes from Phase 1**: Modal.svelte (used by AddQuickListModal)
**Produces for next phase**: None (final phase)

**Wiring required**:
- [x] N/A - inline class changes only

### Changes

#### QuickListsManager.svelte

**File**: `src/lib/components/QuickListsManager.svelte`

**Change**: Update container and page title

```svelte
<!-- Line 13: Update container -->
<div class="max-w-[1800px] 3xl:max-w-[2400px] mx-auto px-2 sm:px-4 2xl:px-6">

<!-- Line 15: Update page title -->
<h2 class="text-xl sm:text-2xl font-bold text-gray-800">Quick Lists</h2>

<!-- Line 44: Update grid to be responsive -->
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
```

#### QuickListCard.svelte

**File**: `src/lib/components/quicklists/QuickListCard.svelte`

**Change**: Add transitions to buttons, standardize focus states

```svelte
<!-- Line 55: Add focus ring to edit input -->
class="flex-1 px-2 py-1 text-sm font-semibold border border-emerald-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-emerald-500"

<!-- Line 73: Add transition to delete button -->
class="p-1 text-gray-400 hover:text-red-600 rounded transition-colors"

<!-- Line 83: Add transition to expand button -->
class="p-1 text-gray-400 hover:text-gray-600 rounded transition-colors"
```

#### QuickListItemRow.svelte

**File**: `src/lib/components/quicklists/QuickListItemRow.svelte`

**Change**: Standardize input focus states and button styles

```svelte
<!-- Lines 60, 66, 72, 76: Update inputs to use focus:ring-2 -->
class="w-16 px-2 py-1 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
class="w-20 px-2 py-1 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
class="flex-1 min-w-[120px] px-2 py-1 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
class="px-2 py-1 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"

<!-- Line 86: Update Save button -->
class="px-2 py-1 text-sm bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"

<!-- Line 93: Update Cancel button -->
class="px-2 py-1 text-sm bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors"

<!-- Lines 110, 118: Add transitions to icon buttons -->
class="p-1 text-gray-400 hover:text-emerald-600 rounded transition-colors"
class="p-1 text-gray-400 hover:text-red-600 rounded transition-colors"
```

#### AddQuickListModal.svelte

**File**: `src/lib/components/quicklists/AddQuickListModal.svelte`

**Change**: Add transitions to buttons

```svelte
<!-- Line 44: Add transition to Cancel button -->
class="px-4 py-2 text-sm text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg transition-colors"

<!-- Line 50: Add transition to Create button -->
class="px-4 py-2 text-sm bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
```

#### quicklists/AddItemForm.svelte

**File**: `src/lib/components/quicklists/AddItemForm.svelte`

**Change**: Update collapsed button to use standard dashed style, add transitions

```svelte
<!-- Lines 57, 63, 69, 73: Add focus rings to inputs -->
class="w-16 px-2 py-1.5 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
class="w-20 px-2 py-1.5 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
class="flex-1 min-w-[140px] px-2 py-1.5 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
class="px-2 py-1.5 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"

<!-- Line 84: Add transition to Cancel button -->
class="px-3 py-1.5 text-sm text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg transition-colors"

<!-- Line 90: Add transition to Add button -->
class="px-3 py-1.5 text-sm bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"

<!-- Line 100: Update collapsed button to dashed style -->
class="w-full py-2 text-sm text-gray-400 hover:text-emerald-600 hover:bg-emerald-50 rounded-lg border border-dashed border-gray-300 transition-colors"
```

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`

#### Integration Verification
- [ ] N/A

#### Manual Verification
- [ ] Quick Lists page layout responsive on all breakpoints
- [ ] All buttons have smooth hover transitions
- [ ] All form inputs show emerald focus ring
- [ ] Add item button matches dashed style from shopping
- [ ] Edit/delete icons have smooth color transitions
- [ ] Empty state styling consistent

**Checkpoint**: Final verification complete.

---

## Testing Strategy

### Unit Tests
- N/A - styling changes only, no logic changes

### Integration Tests
- N/A - no new integrations

### E2E Tests
- N/A - visual changes only

### Manual Testing Checklist
1. [ ] Dashboard: responsive layout at 320px, 768px, 1024px, 1440px, 2560px
2. [ ] Dashboard: stat cards hover shadow
3. [ ] Dashboard: section headings consistent
4. [ ] Recipes: responsive grid layout at all breakpoints
5. [ ] Recipes: recipe cards hover shadow
6. [ ] Recipes: Edit button emerald outline style
7. [ ] Recipes: all modal inputs have focus rings
8. [ ] Quick Lists: responsive grid layout
9. [ ] Quick Lists: all buttons have transitions
10. [ ] Quick Lists: add item dashed button style
11. [ ] All pages: no visual regressions
12. [ ] All pages: consistent emerald color usage

## Rollback Plan

Git revert to commit before Phase 1:
```bash
git revert --no-commit HEAD~N..HEAD
```

Since these are purely visual changes with no data or logic modifications, rollback is straightforward.

## Migration Notes

- **Data migration**: None required
- **Feature flags**: None
- **Backwards compatibility**: Not applicable - visual changes only

## References

- Ticket: `ai_docs/prompts/2025-12-15-design-system-standardization.md`
- Design reference: `src/lib/components/mealplan/MealPlanCalendar.svelte`
- Button patterns: `src/lib/components/shopping/ShoppingSection.svelte:326-350`
- Grid patterns: `src/lib/components/shopping/ShoppingSection.svelte:281`
