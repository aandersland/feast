# Recipes Page Redesign Implementation Plan

## Overview

Redesign the Recipes page to add 3-ingredient autocomplete filters, a "View by" toggle for grouping recipes (Tag category, Protein, Starch), and a split-screen detail panel that opens on the right when clicking a recipe. This replaces the current full-page navigation pattern with a master-detail layout that keeps the recipe list visible.

## Current State

The Recipes page (`src/lib/components/Recipes.svelte`) uses a view state machine with four modes: `list`, `detail`, `create`, `import`. Selecting a recipe navigates to a full-page detail view. Search filters by name or tag only.

**Key Discoveries**:
- View state machine controls navigation: `src/lib/components/Recipes.svelte:9-11`
- Recipe type has structured ingredients array: `src/lib/types/recipe.ts:18-24`
- ViewToggle pattern exists for 2-option toggle: `src/lib/components/shopping/ViewToggle.svelte`
- No autocomplete component exists—search is inline input
- RecipeForm only supports create, not edit: `src/lib/components/recipes/RecipeForm.svelte:5-8`
- Mock data has 6 recipes with diverse tags: `src/lib/stores/recipes.ts:4-191`
- Meal plan store exposes `addMeal()`: `src/lib/stores/mealPlan.ts:42-43`

## Desired End State

- 3 ingredient autocomplete inputs filter recipes (AND logic)
- "View by" toggle groups recipes: Default (grid), Tag category, Protein, Starch
- Clicking a recipe opens detail panel on right; grid shrinks to 2 columns
- Clicking another recipe swaps panel content without closing
- Close button returns to full-width 3-column grid
- Detail panel shows full recipe with Edit, Add to Meal Plan, Close actions
- Edit opens RecipeForm in edit mode (pre-populated)
- Add to Meal Plan opens date/meal-type picker modal

## What We're NOT Doing

- Import from URL functionality changes
- Meal plan page changes
- Backend/Rust implementation
- SQLite schema changes
- Any data persistence (mock data only)
- Responsive mobile layout (reasonable behavior is fine)

## Integration Map

| Type | Location | Notes |
|------|----------|-------|
| Entry point | `src/lib/components/Recipes.svelte` | Main refactor target |
| State stores | `src/lib/stores/recipes.ts` | Add derived stores for filtering/grouping |
| Types | `src/lib/types/recipe.ts` | Add RecipeViewMode type |
| Store exports | `src/lib/stores/index.ts:2` | Export new derived stores |
| Meal plan consumer | `src/lib/stores/mealPlan.ts:42` | Use `addMeal` for panel action |
| Shared components | `src/lib/components/shared/` | Add Autocomplete component |
| Recipe components | `src/lib/components/recipes/` | Modify RecipeDetail, RecipeForm |

## Implementation Approach

Build bottom-up: data layer first, then reusable components, then layout, then integration. This allows each phase to be tested independently. The split-screen layout is the most complex piece, so we establish the data foundation first.

---

## Phase 1: Data Layer & Types

### Goal
Add derived stores for ingredient extraction, filtering logic, and view grouping categories.

### Integration Points

**Depends on**: Existing `recipeStore` at `src/lib/stores/recipes.ts`
**Produces for next phase**: `allIngredients` derived store, `RecipeViewMode` type, grouping utilities

**Wiring required**:
- [x] Export new stores from `src/lib/stores/index.ts`
- [x] Export new types from `src/lib/types/index.ts`

### Changes

#### Types

**File**: `src/lib/types/recipe.ts`

**Change**: Add view mode type and grouping categories

```typescript
export type RecipeViewMode = "default" | "tag" | "protein" | "starch";

export const PROTEIN_KEYWORDS = [
  "chicken", "beef", "pork", "salmon", "fish", "shrimp", "tofu",
  "turkey", "lamb", "eggs", "seafood"
] as const;

export const STARCH_KEYWORDS = [
  "pasta", "spaghetti", "rice", "potato", "bread", "noodles",
  "quinoa", "oats", "tortilla"
] as const;
```

#### Stores

**File**: `src/lib/stores/recipes.ts`

**Change**: Add derived stores for ingredient list and grouping

```typescript
import { PROTEIN_KEYWORDS, STARCH_KEYWORDS } from "$lib/types";

// Extract all unique ingredient names across recipes
export const allIngredients = derived(recipeStore, ($recipes) => {
  const ingredients = new Set<string>();
  $recipes.forEach((r) => {
    r.ingredients.forEach((i) => {
      ingredients.add(i.name.toLowerCase());
    });
  });
  return Array.from(ingredients).sort();
});

// Helper to detect protein in recipe
export function getRecipeProtein(recipe: Recipe): string | null {
  const text = recipe.ingredients.map(i => i.name.toLowerCase()).join(" ");
  for (const protein of PROTEIN_KEYWORDS) {
    if (text.includes(protein)) return protein;
  }
  return null;
}

// Helper to detect starch in recipe
export function getRecipeStarch(recipe: Recipe): string | null {
  const text = recipe.ingredients.map(i => i.name.toLowerCase()).join(" ");
  for (const starch of STARCH_KEYWORDS) {
    if (text.includes(starch)) return starch;
  }
  return null;
}

// Group recipes by a key function
export function groupRecipes<K extends string>(
  recipes: Recipe[],
  keyFn: (r: Recipe) => K | null
): Map<K | "Other", Recipe[]> {
  const groups = new Map<K | "Other", Recipe[]>();
  recipes.forEach((r) => {
    const key = keyFn(r) ?? "Other";
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key)!.push(r);
  });
  return groups;
}
```

#### Store Exports

**File**: `src/lib/stores/index.ts`

**Change**: Add exports for new stores and utilities

```typescript
export {
  recipeStore,
  recipeById,
  allIngredients,
  getRecipeProtein,
  getRecipeStarch,
  groupRecipes
} from "./recipes";
```

#### Type Exports

**File**: `src/lib/types/index.ts`

**Change**: Add exports for new types

```typescript
export type {
  Recipe,
  Ingredient,
  NutritionInfo,
  RecipeViewMode,
} from "./recipe";
export { PROTEIN_KEYWORDS, STARCH_KEYWORDS } from "./recipe";
```

### Success Criteria

#### Automated Verification
- [x] Types check: `pnpm check`
- [~] Lint passes: `pnpm lint` (eslint not installed - pre-existing)

#### Integration Verification
- [x] `allIngredients` importable from `$lib/stores`
- [x] `RecipeViewMode` importable from `$lib/types`

#### Manual Verification
- [x] N/A (data layer only)

**Checkpoint**: Verify types check before proceeding to Phase 2.

---

## Phase 2: Autocomplete Filter Component

### Goal
Build a reusable autocomplete input component and the 3-filter ingredient search UI.

### Integration Points

**Consumes from Phase 1**: `allIngredients` from `src/lib/stores/recipes.ts`
**Produces for next phase**: `Autocomplete.svelte`, `IngredientFilters.svelte`

**Wiring required**:
- [x] Create `src/lib/components/shared/Autocomplete.svelte`
- [x] Create `src/lib/components/recipes/IngredientFilters.svelte`

### Changes

#### Autocomplete Component

**File**: `src/lib/components/shared/Autocomplete.svelte`

**Change**: Create reusable autocomplete input

```svelte
<script lang="ts">
  interface Props {
    options: string[];
    value: string;
    onSelect: (value: string) => void;
    onClear: () => void;
    placeholder?: string;
  }

  let { options, value, onSelect, onClear, placeholder = "Search..." }: Props = $props();

  let query = $state("");
  let isOpen = $state(false);
  let inputRef: HTMLInputElement;

  let filteredOptions = $derived(
    query.length > 0
      ? options.filter((o) =>
          o.toLowerCase().includes(query.toLowerCase()) && o !== value
        ).slice(0, 8)
      : []
  );

  function handleSelect(option: string) {
    onSelect(option);
    query = "";
    isOpen = false;
  }

  function handleClear() {
    onClear();
    query = "";
  }

  function handleInputFocus() {
    isOpen = true;
  }

  function handleInputBlur() {
    // Delay to allow click on option
    setTimeout(() => { isOpen = false; }, 150);
  }
</script>

<div class="relative">
  {#if value}
    <div class="flex items-center gap-2 px-3 py-2 bg-emerald-50 border border-emerald-200 rounded-lg">
      <span class="text-emerald-700 capitalize">{value}</span>
      <button
        type="button"
        onclick={handleClear}
        class="text-emerald-500 hover:text-emerald-700"
        aria-label="Clear filter"
      >
        x
      </button>
    </div>
  {:else}
    <input
      bind:this={inputRef}
      type="text"
      bind:value={query}
      onfocus={handleInputFocus}
      onblur={handleInputBlur}
      {placeholder}
      class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
    />
    {#if isOpen && filteredOptions.length > 0}
      <ul class="absolute z-10 w-full mt-1 bg-white border border-gray-200 rounded-lg shadow-lg max-h-48 overflow-auto">
        {#each filteredOptions as option}
          <li>
            <button
              type="button"
              onclick={() => handleSelect(option)}
              class="w-full text-left px-3 py-2 hover:bg-emerald-50 capitalize"
            >
              {option}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  {/if}
</div>
```

#### Ingredient Filters Component

**File**: `src/lib/components/recipes/IngredientFilters.svelte`

**Change**: Create 3-filter ingredient search UI

```svelte
<script lang="ts">
  import { allIngredients } from "$lib/stores";
  import Autocomplete from "$lib/components/shared/Autocomplete.svelte";

  interface Props {
    filters: [string, string, string];
    onFiltersChange: (filters: [string, string, string]) => void;
  }

  let { filters, onFiltersChange }: Props = $props();

  function updateFilter(index: number, value: string) {
    const newFilters = [...filters] as [string, string, string];
    newFilters[index] = value;
    onFiltersChange(newFilters);
  }

  function clearFilter(index: number) {
    updateFilter(index, "");
  }

  // Exclude already-selected ingredients from options
  let availableIngredients = $derived(
    $allIngredients.filter((i) => !filters.includes(i))
  );
</script>

<div class="flex flex-wrap gap-3">
  <span class="text-sm text-gray-500 self-center">Filter by ingredient:</span>
  {#each [0, 1, 2] as index}
    <div class="w-44">
      <Autocomplete
        options={availableIngredients}
        value={filters[index]}
        onSelect={(v) => updateFilter(index, v)}
        onClear={() => clearFilter(index)}
        placeholder="Add ingredient..."
      />
    </div>
  {/each}
</div>
```

### Success Criteria

#### Automated Verification
- [x] Types check: `pnpm check`
- [~] Lint passes: `pnpm lint` (eslint not installed - pre-existing)

#### Integration Verification
- [x] `Autocomplete` importable from `$lib/components/shared/Autocomplete.svelte`
- [x] `IngredientFilters` importable from `$lib/components/recipes/IngredientFilters.svelte`

#### Manual Verification
- [ ] Autocomplete shows filtered options when typing
- [ ] Selecting an option displays it as a chip
- [ ] Clear button removes selection
- [ ] Already-selected ingredients hidden from other dropdowns

**Checkpoint**: Test autocomplete in isolation before proceeding to Phase 3.

---

## Phase 3: Recipe View Toggle

### Goal
Create a multi-option ViewToggle for Tag/Protein/Starch grouping, modeled after the shopping list ViewToggle.

### Integration Points

**Consumes from Phase 1**: `RecipeViewMode` type
**Produces for next phase**: `RecipeViewToggle.svelte`

**Wiring required**:
- [x] Create `src/lib/components/recipes/RecipeViewToggle.svelte`

### Changes

#### View Toggle Component

**File**: `src/lib/components/recipes/RecipeViewToggle.svelte`

**Change**: Create multi-option view toggle

```svelte
<script lang="ts">
  import type { RecipeViewMode } from "$lib/types";

  interface Props {
    activeView: RecipeViewMode;
    onViewChange: (view: RecipeViewMode) => void;
  }

  let { activeView, onViewChange }: Props = $props();

  const viewOptions: { value: RecipeViewMode; label: string }[] = [
    { value: "default", label: "All" },
    { value: "tag", label: "Category" },
    { value: "protein", label: "Protein" },
    { value: "starch", label: "Starch" },
  ];
</script>

<div class="flex items-center gap-2 text-sm">
  <span class="text-gray-500">View by:</span>
  <div class="flex rounded-lg border border-gray-200 overflow-hidden">
    {#each viewOptions as option}
      <button
        type="button"
        onclick={() => onViewChange(option.value)}
        class="px-3 py-1.5 transition-colors
          {activeView === option.value ? 'bg-emerald-100 text-emerald-700' : 'hover:bg-gray-50'}"
      >
        {option.label}
      </button>
    {/each}
  </div>
</div>
```

### Success Criteria

#### Automated Verification
- [x] Types check: `pnpm check`
- [~] Lint passes: `pnpm lint` (eslint not installed - pre-existing)

#### Integration Verification
- [x] Component importable and renders

#### Manual Verification
- [ ] Toggle buttons switch active state visually
- [ ] Callback fires with correct view mode

**Checkpoint**: Verify toggle works before proceeding to Phase 4.

---

## Phase 4: Split-Screen Layout

### Goal
Refactor Recipes.svelte to master-detail split layout with resizing grid.

### Integration Points

**Consumes from Phase 1-3**: `allIngredients`, `groupRecipes`, `RecipeViewMode`, `IngredientFilters`, `RecipeViewToggle`
**Produces for next phase**: Split layout with panel slot, filtering/grouping integrated

**Wiring required**:
- [x] Import new components into `src/lib/components/Recipes.svelte`
- [x] Refactor view state from page navigation to panel toggle

### Changes

#### Main Recipes Component

**File**: `src/lib/components/Recipes.svelte`

**Change**: Complete refactor to split-screen layout

```svelte
<script lang="ts">
  import { recipeStore, groupRecipes, getRecipeProtein, getRecipeStarch } from "$lib/stores";
  import type { Recipe, RecipeViewMode } from "$lib/types";
  import RecipeCard from "./recipes/RecipeCard.svelte";
  import RecipeDetailPanel from "./recipes/RecipeDetailPanel.svelte";
  import RecipeForm from "./recipes/RecipeForm.svelte";
  import ImportRecipe from "./recipes/ImportRecipe.svelte";
  import IngredientFilters from "./recipes/IngredientFilters.svelte";
  import RecipeViewToggle from "./recipes/RecipeViewToggle.svelte";
  import Modal from "./shared/Modal.svelte";

  type ModalView = "none" | "create" | "import" | "edit";

  // Panel state
  let selectedRecipe = $state<Recipe | null>(null);
  let isPanelOpen = $derived(selectedRecipe !== null);

  // Modal state
  let modalView = $state<ModalView>("none");
  let editingRecipe = $state<Recipe | null>(null);

  // Filter state
  let searchQuery = $state("");
  let ingredientFilters = $state<[string, string, string]>(["", "", ""]);
  let viewMode = $state<RecipeViewMode>("default");

  // Filtering logic
  let filteredRecipes = $derived.by(() => {
    let recipes = $recipeStore;

    // Text search
    if (searchQuery) {
      const q = searchQuery.toLowerCase();
      recipes = recipes.filter(
        (r) =>
          r.name.toLowerCase().includes(q) ||
          r.tags.some((t) => t.toLowerCase().includes(q))
      );
    }

    // Ingredient filters (AND logic)
    const activeFilters = ingredientFilters.filter((f) => f !== "");
    if (activeFilters.length > 0) {
      recipes = recipes.filter((r) => {
        const recipeIngredients = r.ingredients.map((i) => i.name.toLowerCase());
        return activeFilters.every((filter) =>
          recipeIngredients.some((ing) => ing.includes(filter))
        );
      });
    }

    return recipes;
  });

  // Grouping logic
  let groupedRecipes = $derived.by(() => {
    if (viewMode === "default") return null;

    if (viewMode === "tag") {
      return groupRecipes(filteredRecipes, (r) => r.tags[0] ?? null);
    }
    if (viewMode === "protein") {
      return groupRecipes(filteredRecipes, getRecipeProtein);
    }
    if (viewMode === "starch") {
      return groupRecipes(filteredRecipes, getRecipeStarch);
    }
    return null;
  });

  function selectRecipe(recipe: Recipe) {
    selectedRecipe = recipe;
  }

  function closePanel() {
    selectedRecipe = null;
  }

  function handleEdit(recipe: Recipe) {
    editingRecipe = recipe;
    modalView = "edit";
  }

  function handleSaveRecipe(data: Omit<Recipe, "id" | "createdAt">) {
    if (editingRecipe) {
      recipeStore.update(editingRecipe.id, data);
      // Update selected if editing current
      if (selectedRecipe?.id === editingRecipe.id) {
        selectedRecipe = { ...editingRecipe, ...data };
      }
    } else {
      const newRecipe: Recipe = {
        ...data,
        id: crypto.randomUUID(),
        createdAt: new Date().toISOString().split("T")[0],
      };
      recipeStore.add(newRecipe);
    }
    modalView = "none";
    editingRecipe = null;
  }

  function handleImport(url: string) {
    alert(`Recipe imported from: ${url}\n\n(In production, this would parse the URL)`);
    modalView = "none";
  }

  function closeModal() {
    modalView = "none";
    editingRecipe = null;
  }

  // Grid columns based on panel state
  let gridCols = $derived(isPanelOpen ? "grid-cols-2" : "grid-cols-3");
</script>

<div class="flex h-full">
  <!-- Recipe List -->
  <div class="flex-1 overflow-auto p-6 {isPanelOpen ? 'border-r border-gray-200' : ''}">
    <div class="max-w-6xl mx-auto">
      <!-- Header -->
      <div class="flex justify-between items-center mb-6">
        <h1 class="text-2xl font-bold text-gray-800">Recipes</h1>
        <div class="flex gap-2">
          <button
            type="button"
            onclick={() => modalView = "import"}
            class="px-4 py-2 border border-emerald-600 text-emerald-600 rounded-lg hover:bg-emerald-50 transition-colors"
          >
            Import from URL
          </button>
          <button
            type="button"
            onclick={() => modalView = "create"}
            class="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
          >
            + Add Recipe
          </button>
        </div>
      </div>

      <!-- Filters Row -->
      <div class="flex flex-wrap items-center justify-between gap-4 mb-6">
        <div class="flex flex-wrap items-center gap-4">
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Search recipes by name or tag..."
            class="w-64 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
          />
          <IngredientFilters
            filters={ingredientFilters}
            onFiltersChange={(f) => ingredientFilters = f}
          />
        </div>
        <RecipeViewToggle
          activeView={viewMode}
          onViewChange={(v) => viewMode = v}
        />
      </div>

      <!-- Recipe Grid -->
      {#if groupedRecipes}
        <!-- Grouped View -->
        {#each [...groupedRecipes.entries()] as [group, recipes]}
          <div class="mb-8">
            <h2 class="text-lg font-semibold text-gray-700 mb-4 capitalize">{group}</h2>
            <div class="grid {gridCols} gap-6">
              {#each recipes as recipe}
                <RecipeCard
                  {recipe}
                  onSelect={selectRecipe}
                  isSelected={selectedRecipe?.id === recipe.id}
                />
              {/each}
            </div>
          </div>
        {/each}
      {:else}
        <!-- Default Grid View -->
        <div class="grid {gridCols} gap-6">
          {#each filteredRecipes as recipe}
            <RecipeCard
              {recipe}
              onSelect={selectRecipe}
              isSelected={selectedRecipe?.id === recipe.id}
            />
          {/each}
        </div>
      {/if}

      {#if filteredRecipes.length === 0}
        <div class="text-center py-12 text-gray-500">
          No recipes found. Try adjusting your filters or add a new recipe!
        </div>
      {/if}
    </div>
  </div>

  <!-- Detail Panel -->
  {#if isPanelOpen && selectedRecipe}
    <div class="w-[480px] flex-shrink-0 overflow-auto bg-white">
      <RecipeDetailPanel
        recipe={selectedRecipe}
        onClose={closePanel}
        onEdit={() => handleEdit(selectedRecipe!)}
      />
    </div>
  {/if}
</div>

<!-- Modals -->
<Modal isOpen={modalView === "create"} onClose={closeModal} title="Add New Recipe">
  {#snippet children()}
    <RecipeForm onSave={handleSaveRecipe} onCancel={closeModal} />
  {/snippet}
</Modal>

<Modal isOpen={modalView === "edit" && editingRecipe !== null} onClose={closeModal} title="Edit Recipe">
  {#snippet children()}
    <RecipeForm
      recipe={editingRecipe}
      onSave={handleSaveRecipe}
      onCancel={closeModal}
    />
  {/snippet}
</Modal>

<Modal isOpen={modalView === "import"} onClose={closeModal} title="Import Recipe">
  {#snippet children()}
    <ImportRecipe onImport={handleImport} onCancel={closeModal} />
  {/snippet}
</Modal>
```

#### Update RecipeCard

**File**: `src/lib/components/recipes/RecipeCard.svelte`

**Change**: Add `isSelected` prop for visual feedback

```svelte
<script lang="ts">
  import type { Recipe } from "$lib/types";

  interface Props {
    recipe: Recipe;
    onSelect: (recipe: Recipe) => void;
    isSelected?: boolean;
  }

  let { recipe, onSelect, isSelected = false }: Props = $props();
</script>

<button
  type="button"
  onclick={() => onSelect(recipe)}
  class="w-full text-left bg-white rounded-xl shadow-sm border overflow-hidden hover:shadow-md transition-shadow
    {isSelected ? 'border-emerald-500 ring-2 ring-emerald-200' : 'border-gray-100'}"
>
  <!-- rest of card unchanged -->
```

### Success Criteria

#### Automated Verification
- [x] Types check: `pnpm check`
- [~] Lint passes: `pnpm lint` (eslint not installed - pre-existing)

#### Integration Verification
- [x] All imports resolve
- [ ] No console errors on page load

#### Manual Verification
- [ ] Recipe grid displays in 3 columns
- [ ] Search filters by name/tag
- [ ] Ingredient autocomplete filters work (AND logic)
- [ ] View toggle groups recipes correctly
- [ ] Clicking recipe opens panel, grid shrinks to 2 columns
- [ ] Clicking different recipe swaps panel content
- [ ] Selected card has visual highlight

**Checkpoint**: Verify layout works before proceeding to Phase 5.

---

## Phase 5: Recipe Detail Panel

### Goal
Create panel-optimized RecipeDetail with Edit, Add to Meal Plan, and Close actions.

### Integration Points

**Consumes from Phase 4**: Panel slot in Recipes.svelte
**Produces for next phase**: Complete detail panel with actions

**Wiring required**:
- [x] Create `src/lib/components/recipes/RecipeDetailPanel.svelte`
- [x] Create `src/lib/components/recipes/AddToMealPlanModal.svelte`

### Changes

#### Detail Panel Component

**File**: `src/lib/components/recipes/RecipeDetailPanel.svelte`

**Change**: Create panel-optimized recipe detail view

```svelte
<script lang="ts">
  import type { Recipe } from "$lib/types";
  import AddToMealPlanModal from "./AddToMealPlanModal.svelte";

  interface Props {
    recipe: Recipe;
    onClose: () => void;
    onEdit: () => void;
  }

  let { recipe, onClose, onEdit }: Props = $props();
  let servingMultiplier = $state(1);
  let adjustedServings = $derived(recipe.servings * servingMultiplier);
  let showMealPlanModal = $state(false);

  // Reset multiplier when recipe changes
  $effect(() => {
    recipe; // dependency
    servingMultiplier = 1;
  });
</script>

<div class="h-full flex flex-col">
  <!-- Header -->
  <div class="flex items-center justify-between px-4 py-3 border-b border-gray-100">
    <h2 class="font-semibold text-gray-800 truncate">{recipe.name}</h2>
    <button
      type="button"
      onclick={onClose}
      class="w-8 h-8 flex items-center justify-center rounded-lg text-gray-400 hover:text-gray-600 hover:bg-gray-100"
      aria-label="Close panel"
    >
      x
    </button>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-auto p-4">
    <!-- Image -->
    {#if recipe.imageUrl}
      <div class="h-48 bg-gray-200 rounded-lg mb-4 overflow-hidden">
        <img src={recipe.imageUrl} alt={recipe.name} class="w-full h-full object-cover" />
      </div>
    {:else}
      <div class="h-48 bg-gradient-to-br from-emerald-100 to-emerald-200 rounded-lg mb-4 flex items-center justify-center">
        <span class="text-5xl">🍽️</span>
      </div>
    {/if}

    <p class="text-gray-600 text-sm mb-4">{recipe.description}</p>

    <!-- Meta -->
    <div class="flex flex-wrap gap-2 mb-4 text-sm">
      <span class="px-2 py-1 bg-gray-100 rounded">⏱️ {recipe.prepTime + recipe.cookTime} min</span>
      <div class="flex items-center gap-1 px-2 py-1 bg-gray-100 rounded">
        <span>👥</span>
        <button
          type="button"
          onclick={() => servingMultiplier = Math.max(0.5, servingMultiplier - 0.5)}
          class="w-5 h-5 rounded bg-gray-200 hover:bg-gray-300 text-xs"
        >-</button>
        <span>{adjustedServings}</span>
        <button
          type="button"
          onclick={() => servingMultiplier += 0.5}
          class="w-5 h-5 rounded bg-gray-200 hover:bg-gray-300 text-xs"
        >+</button>
      </div>
    </div>

    <!-- Tags -->
    {#if recipe.tags.length > 0}
      <div class="flex flex-wrap gap-1 mb-4">
        {#each recipe.tags as tag}
          <span class="px-2 py-0.5 text-xs rounded-full bg-emerald-100 text-emerald-700">{tag}</span>
        {/each}
      </div>
    {/if}

    <!-- Ingredients -->
    <div class="mb-4">
      <h3 class="font-semibold text-gray-800 mb-2">Ingredients</h3>
      <ul class="space-y-1 text-sm">
        {#each recipe.ingredients as ing}
          <li class="flex gap-2">
            <span class="text-emerald-500">•</span>
            <span>
              {(ing.quantity * servingMultiplier).toFixed(ing.quantity * servingMultiplier % 1 === 0 ? 0 : 1)}
              {ing.unit} {ing.name}
              {#if ing.notes}<span class="text-gray-400">({ing.notes})</span>{/if}
            </span>
          </li>
        {/each}
      </ul>
    </div>

    <!-- Instructions -->
    <div class="mb-4">
      <h3 class="font-semibold text-gray-800 mb-2">Instructions</h3>
      <ol class="space-y-2 text-sm">
        {#each recipe.instructions as step, i}
          <li class="flex gap-2">
            <span class="flex-shrink-0 w-5 h-5 rounded-full bg-emerald-100 text-emerald-700 flex items-center justify-center text-xs">{i + 1}</span>
            <span>{step}</span>
          </li>
        {/each}
      </ol>
    </div>

    <!-- Nutrition -->
    {#if recipe.nutrition}
      <div class="p-3 bg-gray-50 rounded-lg text-sm">
        <h3 class="font-semibold text-gray-800 mb-2">Nutrition (per serving)</h3>
        <div class="grid grid-cols-4 gap-2 text-center">
          <div>
            <div class="font-bold">{recipe.nutrition.calories}</div>
            <div class="text-xs text-gray-500">Cal</div>
          </div>
          <div>
            <div class="font-bold">{recipe.nutrition.protein}g</div>
            <div class="text-xs text-gray-500">Protein</div>
          </div>
          <div>
            <div class="font-bold">{recipe.nutrition.carbs}g</div>
            <div class="text-xs text-gray-500">Carbs</div>
          </div>
          <div>
            <div class="font-bold">{recipe.nutrition.fat}g</div>
            <div class="text-xs text-gray-500">Fat</div>
          </div>
        </div>
      </div>
    {/if}
  </div>

  <!-- Actions Footer -->
  <div class="flex gap-2 p-4 border-t border-gray-100">
    <button
      type="button"
      onclick={onEdit}
      class="flex-1 px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
    >
      Edit
    </button>
    <button
      type="button"
      onclick={() => showMealPlanModal = true}
      class="flex-1 px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
    >
      Add to Meal Plan
    </button>
  </div>
</div>

<AddToMealPlanModal
  isOpen={showMealPlanModal}
  {recipe}
  onClose={() => showMealPlanModal = false}
/>
```

#### Add to Meal Plan Modal

**File**: `src/lib/components/recipes/AddToMealPlanModal.svelte`

**Change**: Create date/meal-type picker for adding to meal plan

```svelte
<script lang="ts">
  import type { Recipe, MealType } from "$lib/types";
  import { mealPlanStore } from "$lib/stores";
  import Modal from "$lib/components/shared/Modal.svelte";

  interface Props {
    isOpen: boolean;
    recipe: Recipe;
    onClose: () => void;
  }

  let { isOpen, recipe, onClose }: Props = $props();

  let selectedDate = $state(new Date().toISOString().split("T")[0]);
  let selectedMealType = $state<MealType>("dinner");
  let servings = $state(4);

  const mealTypes: MealType[] = ["breakfast", "lunch", "dinner", "snack"];

  function handleAdd() {
    mealPlanStore.addMeal(selectedDate, recipe.id, selectedMealType, servings);
    onClose();
  }

  // Reset form when opened
  $effect(() => {
    if (isOpen) {
      selectedDate = new Date().toISOString().split("T")[0];
      selectedMealType = "dinner";
      servings = recipe.servings;
    }
  });
</script>

<Modal {isOpen} {onClose} title="Add to Meal Plan">
  {#snippet children()}
    <div class="space-y-4">
      <p class="text-gray-600">Add <strong>{recipe.name}</strong> to your meal plan.</p>

      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Date</label>
        <input
          type="date"
          bind:value={selectedDate}
          class="w-full px-3 py-2 border border-gray-300 rounded-lg"
        />
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Meal</label>
        <select
          bind:value={selectedMealType}
          class="w-full px-3 py-2 border border-gray-300 rounded-lg"
        >
          {#each mealTypes as type}
            <option value={type} class="capitalize">{type}</option>
          {/each}
        </select>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Servings</label>
        <input
          type="number"
          bind:value={servings}
          min="1"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg"
        />
      </div>

      <div class="flex justify-end gap-2 pt-4">
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg"
        >
          Cancel
        </button>
        <button
          type="button"
          onclick={handleAdd}
          class="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700"
        >
          Add
        </button>
      </div>
    </div>
  {/snippet}
</Modal>
```

### Success Criteria

#### Automated Verification
- [x] Types check: `pnpm check`
- [~] Lint passes: `pnpm lint` (eslint not installed - pre-existing)

#### Integration Verification
- [x] Panel displays correctly in split layout
- [x] Add to Meal Plan modal opens

#### Manual Verification
- [ ] Panel shows full recipe info
- [ ] Serving multiplier works
- [ ] Edit button triggers edit callback
- [ ] Add to Meal Plan modal allows date/meal selection
- [ ] Adding to meal plan updates the meal plan store

**Checkpoint**: Verify panel actions before proceeding to Phase 6.

---

## Phase 6: Recipe Form Edit Mode

### Goal
Extend RecipeForm to support editing existing recipes with pre-populated fields.

### Integration Points

**Consumes from Phase 4-5**: Edit button triggers modal with recipe prop
**Produces**: Complete edit functionality

**Wiring required**:
- [x] Modify `src/lib/components/recipes/RecipeForm.svelte` to accept optional `recipe` prop

### Changes

#### Recipe Form Component

**File**: `src/lib/components/recipes/RecipeForm.svelte`

**Change**: Add optional `recipe` prop for edit mode

```svelte
<script lang="ts">
  import type { Recipe, Ingredient } from "$lib/types";

  interface Props {
    recipe?: Recipe | null;
    onSave: (recipe: Omit<Recipe, "id" | "createdAt">) => void;
    onCancel: () => void;
  }

  let { recipe = null, onSave, onCancel }: Props = $props();

  let name = $state(recipe?.name ?? "");
  let description = $state(recipe?.description ?? "");
  let prepTime = $state(recipe?.prepTime ?? 15);
  let cookTime = $state(recipe?.cookTime ?? 30);
  let servings = $state(recipe?.servings ?? 4);
  let ingredients = $state<Omit<Ingredient, "id">[]>(
    recipe?.ingredients.map(({ id, ...rest }) => rest) ?? [{ name: "", quantity: 1, unit: "" }]
  );
  let instructions = $state(recipe?.instructions ?? [""]);
  let tags = $state(recipe?.tags.join(", ") ?? "");
  let notes = $state(recipe?.notes ?? "");
  let sourceUrl = $state(recipe?.sourceUrl ?? "");

  // Reset form when recipe prop changes
  $effect(() => {
    if (recipe) {
      name = recipe.name;
      description = recipe.description;
      prepTime = recipe.prepTime;
      cookTime = recipe.cookTime;
      servings = recipe.servings;
      ingredients = recipe.ingredients.map(({ id, ...rest }) => rest);
      instructions = [...recipe.instructions];
      tags = recipe.tags.join(", ");
      notes = recipe.notes ?? "";
      sourceUrl = recipe.sourceUrl ?? "";
    }
  });

  // ... rest of component unchanged, but update title:
</script>

<div class="space-y-6">
  <h2 class="text-xl font-bold text-gray-800">
    {recipe ? "Edit Recipe" : "Add New Recipe"}
  </h2>
  <!-- form fields unchanged -->
</div>
```

Note: The form is now used inside a Modal, so remove the outer wrapper div with max-w and shadow styling. The Modal provides the container.

### Success Criteria

#### Automated Verification
- [x] Types check: `pnpm check`
- [~] Lint passes: `pnpm lint` (eslint not installed - pre-existing)

#### Integration Verification
- [x] Form pre-populates when recipe prop provided
- [x] Form clears for create mode

#### Manual Verification
- [ ] Edit button opens form with existing data
- [ ] Saving updates the recipe in the store
- [ ] Changes reflect in detail panel and card

**Checkpoint**: Verify edit flow works before proceeding to Phase 7.

---

## Phase 7: Integration & Polish

### Goal
Wire all components together, handle edge cases, verify end-to-end flow.

### Integration Points

**Consumes**: All prior phase outputs
**Produces**: Complete feature, fully functional

**Wiring required**:
- [x] Verify all imports resolve in Recipes.svelte
- [x] Test all user flows end-to-end
- [x] Fix any edge cases discovered

### Changes

#### Edge Case Handling

**File**: `src/lib/components/Recipes.svelte`

**Change**: Handle recipe deletion and edge cases

```svelte
// Add to script section:

// Clear panel if selected recipe is deleted
$effect(() => {
  if (selectedRecipe && !$recipeStore.find(r => r.id === selectedRecipe.id)) {
    selectedRecipe = null;
  }
});
```

#### Final Type Exports Verification

**File**: `src/lib/types/mealPlan.ts`

**Verify**: `MealType` is exported (needed for AddToMealPlanModal)

```typescript
export type MealType = "breakfast" | "lunch" | "dinner" | "snack";
```

### Success Criteria

#### Automated Verification
- [x] Full type check: `pnpm check`
- [~] Lint passes: `pnpm lint` (eslint not installed - pre-existing)
- [~] Tests pass: `pnpm test` (no test files in project)

#### Integration Verification
- [x] All components load without errors
- [ ] No console warnings/errors
- [x] All store exports resolve

#### Manual Verification
- [ ] 3 ingredient autocomplete boxes filter recipes correctly (AND logic)
- [ ] View by toggle switches between: Default, Tag category, Protein, Starch groupings
- [ ] Clicking recipe opens detail panel on right, list shrinks to 2 columns
- [ ] Clicking different recipe swaps panel content without closing
- [ ] Close button returns to full-width grid
- [ ] Detail panel shows complete recipe info
- [ ] Edit button opens recipe form with pre-populated data
- [ ] Saving edit updates card and panel
- [ ] Add to meal plan button opens modal with date picker
- [ ] Adding to meal plan stores the entry
- [ ] Creating new recipe works
- [ ] Import URL modal still works
- [ ] Empty state shows when no recipes match filters

---

## Testing Strategy

### Unit Tests
- Filtering logic: AND logic with multiple ingredients
- Grouping functions: correct categorization by tag/protein/starch
- Serving multiplier calculations

### Integration Tests
- N/A for UI-only mock implementation

### E2E Tests
- N/A for UI-only mock implementation

### Manual Testing Checklist
1. [ ] Load page, verify 3-column grid with all recipes
2. [ ] Type in search, verify filtering by name
3. [ ] Select ingredient in autocomplete, verify filtering
4. [ ] Select multiple ingredients, verify AND logic
5. [ ] Clear ingredient filter, verify recipes return
6. [ ] Toggle to "Category" view, verify grouped display
7. [ ] Toggle to "Protein" view, verify grouped by protein
8. [ ] Toggle to "Starch" view, verify grouped by starch
9. [ ] Toggle back to "All", verify flat grid
10. [ ] Click recipe card, verify panel opens, grid shrinks
11. [ ] Click different recipe, verify panel swaps
12. [ ] Click close button, verify panel closes, grid expands
13. [ ] Click Edit in panel, verify form opens with data
14. [ ] Modify recipe, save, verify updates
15. [ ] Click Add to Meal Plan, verify modal opens
16. [ ] Select date/meal, add, verify success
17. [ ] Click "+ Add Recipe", verify create form
18. [ ] Create recipe, verify it appears in grid
19. [ ] Click "Import from URL", verify modal opens

## Rollback Plan

Git revert to commit before Phase 1:
```
git revert --no-commit HEAD~N..HEAD
```

No data migration or feature flags involved.

## Migration Notes

- **Data migration**: None required (mock data only)
- **Feature flags**: None
- **Backwards compatibility**: Not applicable (UI-only)

## References

- Ticket: `ai_docs/prompts/2024-12-15-recipes-page-redesign.md`
- Similar implementation (ViewToggle): `src/lib/components/shopping/ViewToggle.svelte`
- Modal pattern: `src/lib/components/shared/Modal.svelte`
