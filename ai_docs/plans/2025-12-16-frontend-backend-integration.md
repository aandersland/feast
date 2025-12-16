# Frontend-Backend Integration Implementation Plan

## Overview

Replace all mock data in Svelte stores with real Tauri backend calls, enabling data persistence across sessions. This includes building UX infrastructure (toast notifications, loading states, confirmation dialogs) and writing automated tests for all store operations.

## Current State

The frontend has 5 stores initialized with mock data. The backend has 36 Tauri commands ready but unused. IPC wrappers exist in `src/lib/tauri/commands.ts`. No toast system, no loading states in stores, no confirmation dialogs, and zero frontend tests.

**Key Discoveries**:
- Frontend `MealPlan` has nested `meals[]` array, backend returns flat rows per meal (`types/mealPlan.ts:1-4` vs `commands.ts:102-107`)
- Frontend uses `isOnHand`, backend uses `isChecked` (`types/shoppingList.ts:7` vs `commands.ts:142`)
- Loading state pattern exists in `ImportRecipe.svelte:9-18` — use as reference
- Test setup already mocks Tauri invoke at `tests/setup.ts:6-8`
- Store factory pattern wraps `writable` with domain methods (`stores/recipes.ts:195-207`)

## Desired End State

- All stores load from and persist to SQLite via Tauri commands
- Toast notifications appear for success/error feedback
- Loading spinners display during async operations
- Confirmation dialogs appear before destructive actions
- Empty states display gracefully on fresh database
- All store operations have passing automated tests
- `pnpm test` and `pnpm check` pass

## What We're NOT Doing

- Feature flag to toggle mock/real backend (clean cutover)
- Component-level tests
- Recipe import from URL
- Performance optimizations (pagination, virtual scrolling)
- Offline support / sync

## Integration Map

| Type | Location | Notes |
|------|----------|-------|
| Entry point | `src/App.svelte` | Add `onMount` for initial data load |
| Store loading | `src/lib/stores/*.ts` | Add `load()`, `isLoading`, `error` to each |
| Toast mount | `src/App.svelte:31` | Add `<ToastContainer />` after main div |
| Toast store | `src/lib/stores/toast.ts` | New file for notification state |
| Confirm dialog | `src/lib/components/shared/ConfirmDialog.svelte` | New reusable component |
| Barrel exports | `src/lib/stores/index.ts` | Export toast store, loading utilities |
| Test files | `src/lib/stores/*.test.ts` | New test files per store |

## Implementation Approach

Build shared UX infrastructure first (Phase 1) so all subsequent phases can use toast/loading/confirm patterns consistently. Then integrate stores feature-by-feature (Phases 2-5), with recipes first since meal plans and shopping lists depend on recipe data. Each store phase follows the same pattern: update store → add type transformation → update components → write tests. Final phase (6) wires up app initialization and verifies end-to-end.

---

## Phase 1: UX Infrastructure

### Goal
Create toast notification system, loading state utilities, and confirmation dialog component.

### Integration Points

**Depends on**: None (foundational)
**Produces for next phase**: Toast store, ConfirmDialog component, loading state pattern

**Wiring required**:
- [x] Create `src/lib/stores/toast.ts` and export from `src/lib/stores/index.ts`
- [x] Create `src/lib/components/shared/ConfirmDialog.svelte`
- [x] Create `src/lib/components/shared/ToastContainer.svelte`
- [x] Mount `ToastContainer` in `src/App.svelte`

### Changes

#### Toast Store

**File**: `src/lib/stores/toast.ts`

**Change**: New file for managing toast notifications

```typescript
import { writable } from "svelte/store";

export interface Toast {
  id: string;
  type: "success" | "error" | "info";
  message: string;
  duration?: number;
}

const { subscribe, update } = writable<Toast[]>([]);

function addToast(toast: Omit<Toast, "id">) {
  const id = crypto.randomUUID();
  update((toasts) => [...toasts, { ...toast, id }]);

  const duration = toast.duration ?? 4000;
  if (duration > 0) {
    setTimeout(() => removeToast(id), duration);
  }
  return id;
}

function removeToast(id: string) {
  update((toasts) => toasts.filter((t) => t.id !== id));
}

export const toastStore = {
  subscribe,
  success: (message: string) => addToast({ type: "success", message }),
  error: (message: string) => addToast({ type: "error", message, duration: 6000 }),
  info: (message: string) => addToast({ type: "info", message }),
  remove: removeToast,
};
```

#### Toast Container Component

**File**: `src/lib/components/shared/ToastContainer.svelte`

**Change**: New component to render toast notifications

```svelte
<script lang="ts">
  import { toastStore } from "$lib/stores/toast";
  import { fly } from "svelte/transition";
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
  {#each $toastStore as toast (toast.id)}
    <div
      transition:fly={{ x: 100, duration: 200 }}
      class="px-4 py-3 rounded-lg shadow-lg max-w-sm flex items-center gap-3"
      class:bg-emerald-600={toast.type === "success"}
      class:bg-red-600={toast.type === "error"}
      class:bg-blue-600={toast.type === "info"}
      class:text-white={true}
    >
      <span class="flex-1">{toast.message}</span>
      <button
        onclick={() => toastStore.remove(toast.id)}
        class="opacity-70 hover:opacity-100"
      >
        ✕
      </button>
    </div>
  {/each}
</div>
```

#### Confirmation Dialog Component

**File**: `src/lib/components/shared/ConfirmDialog.svelte`

**Change**: New reusable confirmation dialog

```svelte
<script lang="ts">
  import Modal from "./Modal.svelte";

  interface Props {
    open: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    destructive?: boolean;
    onConfirm: () => void;
    onCancel: () => void;
  }

  let {
    open,
    title,
    message,
    confirmLabel = "Confirm",
    cancelLabel = "Cancel",
    destructive = false,
    onConfirm,
    onCancel,
  }: Props = $props();
</script>

{#if open}
  <Modal onClose={onCancel}>
    <div class="p-6">
      <h2 class="text-lg font-semibold text-gray-900 mb-2">{title}</h2>
      <p class="text-gray-600 mb-6">{message}</p>
      <div class="flex justify-end gap-3">
        <button
          onclick={onCancel}
          class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg"
        >
          {cancelLabel}
        </button>
        <button
          onclick={onConfirm}
          class="px-4 py-2 rounded-lg text-white"
          class:bg-red-600={destructive}
          class:hover:bg-red-700={destructive}
          class:bg-emerald-600={!destructive}
          class:hover:bg-emerald-700={!destructive}
        >
          {confirmLabel}
        </button>
      </div>
    </div>
  </Modal>
{/if}
```

#### Mount Toast Container

**File**: `src/App.svelte`

**Change**: Import and mount ToastContainer

```svelte
<script lang="ts">
  // Add import
  import ToastContainer from "$lib/components/shared/ToastContainer.svelte";
  // ... existing imports
</script>

<!-- At end of template, after main closing div -->
<ToastContainer />
```

#### Update Barrel Export

**File**: `src/lib/stores/index.ts`

**Change**: Export toast store

```typescript
// Add export
export { toastStore } from "./toast";
```

### Success Criteria

#### Automated Verification
- [x] Types check: `pnpm check`
- [x] Lint passes: `pnpm lint` (eslint not configured in project)

#### Integration Verification
- [x] `toastStore` importable from `$lib/stores`
- [x] `ConfirmDialog` importable from `$lib/components/shared/ConfirmDialog.svelte`
- [x] `ToastContainer` renders in App without errors

#### Manual Verification
- [ ] Run `pnpm tauri dev`
- [ ] Import `toastStore` in any component, call `toastStore.success("Test")` — toast appears
- [ ] Toast auto-dismisses after 4 seconds
- [ ] Error toast stays longer (6 seconds)
- [ ] Close button dismisses toast immediately

**Checkpoint**: Pause for manual verification before proceeding to Phase 2.

---

## Phase 2: Recipe Store Integration

### Goal
Wire recipe store to backend commands with loading states, error handling, and tests.

### Integration Points

**Consumes from Phase 1**: `toastStore` from `$lib/stores/toast`
**Produces for next phase**: Working recipe persistence, pattern for other store integrations

**Wiring required**:
- [x] Update `src/lib/stores/recipes.ts` with async operations
- [x] Add loading/error state to recipe components
- [x] Create `src/lib/stores/recipes.test.ts`

### Changes

#### Recipe Store Update

**File**: `src/lib/stores/recipes.ts`

**Change**: Replace mock data with backend calls, add loading state

```typescript
import { writable, derived, get } from "svelte/store";
import type { Recipe } from "$lib/types";
import {
  getRecipes,
  getRecipe,
  createRecipe as createRecipeCmd,
  updateRecipe as updateRecipeCmd,
  deleteRecipe as deleteRecipeCmd,
  type RecipeInput,
} from "$lib/tauri/commands";
import { toastStore } from "./toast";

// Remove mockRecipes array entirely

// Loading and error state
export const recipesLoading = writable(false);
export const recipesError = writable<string | null>(null);

const { subscribe, set, update } = writable<Recipe[]>([]);

// Transform backend RecipeRow[] to frontend Recipe[]
// Note: getRecipe returns full Recipe with ingredients, getRecipes returns RecipeRow[]
async function loadRecipes() {
  recipesLoading.set(true);
  recipesError.set(null);
  try {
    const rows = await getRecipes();
    // Fetch full recipe details for each (includes ingredients)
    const recipes = await Promise.all(rows.map((row) => getRecipe(row.id)));
    set(recipes);
  } catch (e) {
    const message = e instanceof Error ? e.message : "Failed to load recipes";
    recipesError.set(message);
    toastStore.error(message);
  } finally {
    recipesLoading.set(false);
  }
}

// Convert frontend Recipe to backend RecipeInput
function toRecipeInput(recipe: Omit<Recipe, "id" | "createdAt" | "updatedAt">): RecipeInput {
  return {
    name: recipe.name,
    description: recipe.description,
    prepTime: recipe.prepTime,
    cookTime: recipe.cookTime,
    servings: recipe.servings,
    imagePath: recipe.imagePath,
    sourceUrl: recipe.sourceUrl,
    notes: recipe.notes,
    tags: recipe.tags ?? [],
    ingredients: recipe.ingredients.map((i) => ({
      name: i.name,
      quantity: i.quantity,
      unit: i.unit,
      category: i.category,
      notes: i.notes,
    })),
    instructions: recipe.instructions,
  };
}

export const recipeStore = {
  subscribe,
  load: loadRecipes,

  add: async (recipe: Omit<Recipe, "id" | "createdAt" | "updatedAt">) => {
    try {
      const created = await createRecipeCmd(toRecipeInput(recipe));
      update((recipes) => [...recipes, created]);
      toastStore.success("Recipe created");
      return created;
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to create recipe";
      toastStore.error(message);
      throw e;
    }
  },

  remove: async (id: string) => {
    try {
      await deleteRecipeCmd(id);
      update((recipes) => recipes.filter((r) => r.id !== id));
      toastStore.success("Recipe deleted");
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to delete recipe";
      toastStore.error(message);
      throw e;
    }
  },

  update: async (id: string, data: Partial<Recipe>) => {
    try {
      const current = get({ subscribe }).find((r) => r.id === id);
      if (!current) throw new Error("Recipe not found");

      const input = toRecipeInput({ ...current, ...data });
      const updated = await updateRecipeCmd(id, input);
      update((recipes) => recipes.map((r) => (r.id === id ? updated : r)));
      toastStore.success("Recipe updated");
      return updated;
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to update recipe";
      toastStore.error(message);
      throw e;
    }
  },
};

// Derived stores remain unchanged
export const recipeById = derived(recipeStore, ($recipes) => {
  const map = new Map<string, Recipe>();
  $recipes.forEach((r) => map.set(r.id, r));
  return map;
});

export const allIngredients = derived(recipeStore, ($recipes) => {
  const ingredients = new Set<string>();
  $recipes.forEach((r) => {
    r.ingredients.forEach((i) => {
      ingredients.add(i.name.toLowerCase());
    });
  });
  return Array.from(ingredients).sort();
});

// Helper functions remain unchanged
export function getRecipeProtein(recipe: Recipe): string | null { /* ... */ }
export function getRecipeStarch(recipe: Recipe): string | null { /* ... */ }
export function groupRecipes<T>(recipes: Recipe[], keyFn: (r: Recipe) => T): Map<T, Recipe[]> { /* ... */ }
```

#### Recipe Store Tests

**File**: `src/lib/stores/recipes.test.ts`

**Change**: New test file

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { recipeStore, recipesLoading, recipesError, recipeById } from "./recipes";

vi.mock("@tauri-apps/api/core");

const mockRecipe = {
  id: "1",
  name: "Test Recipe",
  description: "Test",
  prepTime: 10,
  cookTime: 20,
  servings: 4,
  ingredients: [{ id: "i1", name: "Salt", quantity: 1, unit: "tsp", category: "Pantry" }],
  instructions: ["Step 1"],
  tags: [],
  createdAt: "2025-01-01",
  updatedAt: "2025-01-01",
};

describe("recipeStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("loads recipes from backend", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce([{ id: "1", name: "Test Recipe" }]) // getRecipes
      .mockResolvedValueOnce(mockRecipe); // getRecipe

    await recipeStore.load();

    expect(invoke).toHaveBeenCalledWith("get_recipes");
    expect(invoke).toHaveBeenCalledWith("get_recipe", { id: "1" });
    expect(get(recipeStore)).toEqual([mockRecipe]);
    expect(get(recipesLoading)).toBe(false);
  });

  it("sets error on load failure", async () => {
    vi.mocked(invoke).mockRejectedValueOnce(new Error("Network error"));

    await recipeStore.load();

    expect(get(recipesError)).toBe("Network error");
    expect(get(recipeStore)).toEqual([]);
  });

  it("adds recipe via backend", async () => {
    vi.mocked(invoke).mockResolvedValueOnce(mockRecipe);

    const result = await recipeStore.add({
      name: "Test Recipe",
      description: "Test",
      prepTime: 10,
      cookTime: 20,
      servings: 4,
      ingredients: [],
      instructions: [],
      tags: [],
    });

    expect(invoke).toHaveBeenCalledWith("create_recipe", expect.any(Object));
    expect(result).toEqual(mockRecipe);
  });

  it("removes recipe via backend", async () => {
    // Setup: add a recipe first
    vi.mocked(invoke)
      .mockResolvedValueOnce([{ id: "1" }])
      .mockResolvedValueOnce(mockRecipe);
    await recipeStore.load();

    vi.mocked(invoke).mockResolvedValueOnce(undefined);
    await recipeStore.remove("1");

    expect(invoke).toHaveBeenCalledWith("delete_recipe", { id: "1" });
    expect(get(recipeStore)).toEqual([]);
  });

  it("recipeById derived store creates map", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce([{ id: "1" }])
      .mockResolvedValueOnce(mockRecipe);
    await recipeStore.load();

    const map = get(recipeById);
    expect(map.get("1")).toEqual(mockRecipe);
  });
});
```

#### Update Recipes Component

**File**: `src/lib/components/Recipes.svelte`

**Change**: Add loading state and confirmation dialog for delete

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { recipeStore, recipesLoading, groupRecipes, getRecipeProtein, getRecipeStarch } from "$lib/stores";
  import ConfirmDialog from "$lib/components/shared/ConfirmDialog.svelte";
  // ... existing imports

  let deleteTarget: { id: string; name: string } | null = $state(null);

  onMount(() => {
    recipeStore.load();
  });

  async function handleDelete() {
    if (deleteTarget) {
      await recipeStore.remove(deleteTarget.id);
      deleteTarget = null;
    }
  }
</script>

{#if $recipesLoading}
  <div class="flex items-center justify-center py-12">
    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-emerald-600"></div>
    <span class="ml-3 text-gray-600">Loading recipes...</span>
  </div>
{:else if $recipeStore.length === 0}
  <div class="text-center py-12">
    <p class="text-gray-500">No recipes yet. Add your first recipe to get started!</p>
  </div>
{:else}
  <!-- existing recipe grid -->
{/if}

<ConfirmDialog
  open={deleteTarget !== null}
  title="Delete Recipe"
  message={`Are you sure you want to delete "${deleteTarget?.name}"? This cannot be undone.`}
  confirmLabel="Delete"
  destructive={true}
  onConfirm={handleDelete}
  onCancel={() => (deleteTarget = null)}
/>
```

#### Update Barrel Export

**File**: `src/lib/stores/index.ts`

**Change**: Export loading/error states

```typescript
export {
  recipeStore,
  recipeById,
  allIngredients,
  recipesLoading,
  recipesError,
  getRecipeProtein,
  getRecipeStarch,
  groupRecipes
} from "./recipes";
```

### Success Criteria

#### Automated Verification
- [x] Tests pass: `pnpm test -- recipes`
- [x] Types check: `pnpm check`
- [x] Lint passes: `pnpm lint` (eslint not configured)

#### Integration Verification
- [x] `recipesLoading` and `recipesError` exportable from `$lib/stores`
- [x] Recipe components compile with new store interface

#### Manual Verification
- [ ] Run `pnpm tauri dev`
- [ ] Recipes page shows loading spinner initially
- [ ] Empty state shows when no recipes exist
- [ ] Create a recipe — appears in list, persists after refresh
- [ ] Delete a recipe — confirmation dialog appears, deletion persists
- [ ] Edit a recipe — changes persist after refresh
- [ ] Error toast appears if backend fails (test by disconnecting DB)

**Checkpoint**: Pause for manual verification before proceeding to Phase 3.

---

## Phase 3: Meal Plan Store Integration

### Goal
Wire meal plan store to backend with type transformation for nested meals structure.

### Integration Points

**Consumes from Phase 2**: `recipeStore` loaded, toast pattern established
**Produces for next phase**: Working meal plan persistence, date-based data loading pattern

**Wiring required**:
- [x] Update `src/lib/stores/mealPlan.ts` with async operations
- [x] Add type transformation for flat backend → nested frontend
- [x] Update `MealPlanCalendar.svelte` with loading state
- [x] Create `src/lib/stores/mealPlan.test.ts`

### Changes

#### Meal Plan Store Update

**File**: `src/lib/stores/mealPlan.ts`

**Change**: Replace mock data, add backend integration with type transformation

Key insight: Backend stores one row per meal (`MealPlanInput`), frontend groups by date (`MealPlan.meals[]`). The store must transform between these representations.

```typescript
import { writable, derived, get } from "svelte/store";
import type { MealPlan, PlannedMeal, MealType } from "$lib/types";
import {
  getMealPlans as getMealPlansCmd,
  createMealPlan as createMealPlanCmd,
  updateMealPlan as updateMealPlanCmd,
  deleteMealPlan as deleteMealPlanCmd,
} from "$lib/tauri/commands";
import { toastStore } from "./toast";

// Remove mock data and getCurrentWeekDates helper

export const mealPlansLoading = writable(false);
export const mealPlansError = writable<string | null>(null);

const { subscribe, set, update } = writable<MealPlan[]>([]);

// Transform flat backend rows to nested frontend structure
function groupMealsByDate(flatMeals: Array<{
  id: string;
  date: string;
  mealType: string;
  recipeId: string;
  servings: number;
}>): MealPlan[] {
  const byDate = new Map<string, MealPlan>();

  for (const meal of flatMeals) {
    if (!byDate.has(meal.date)) {
      byDate.set(meal.date, {
        id: meal.date, // Use date as MealPlan ID
        date: meal.date,
        meals: [],
      });
    }
    byDate.get(meal.date)!.meals.push({
      id: meal.id,
      recipeId: meal.recipeId,
      mealType: meal.mealType as MealType,
      servings: meal.servings,
    });
  }

  return Array.from(byDate.values());
}

async function loadMealPlans(startDate: string, endDate: string) {
  mealPlansLoading.set(true);
  mealPlansError.set(null);
  try {
    const flatMeals = await getMealPlansCmd(startDate, endDate);
    const grouped = groupMealsByDate(flatMeals);
    set(grouped);
  } catch (e) {
    const message = e instanceof Error ? e.message : "Failed to load meal plans";
    mealPlansError.set(message);
    toastStore.error(message);
  } finally {
    mealPlansLoading.set(false);
  }
}

export const mealPlanStore = {
  subscribe,
  load: loadMealPlans,

  addMeal: async (date: string, recipeId: string, mealType: MealType, servings: number) => {
    try {
      const created = await createMealPlanCmd({ date, mealType, recipeId, servings });

      update((plans) => {
        const existing = plans.find((p) => p.date === date);
        if (existing) {
          return plans.map((p) =>
            p.date === date
              ? { ...p, meals: [...p.meals, { id: created.id, recipeId, mealType, servings }] }
              : p
          );
        } else {
          return [...plans, {
            id: date,
            date,
            meals: [{ id: created.id, recipeId, mealType, servings }],
          }];
        }
      });
      toastStore.success("Meal added to plan");
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to add meal";
      toastStore.error(message);
      throw e;
    }
  },

  removeMeal: async (date: string, mealId: string) => {
    try {
      await deleteMealPlanCmd(mealId);

      update((plans) =>
        plans
          .map((p) =>
            p.date === date ? { ...p, meals: p.meals.filter((m) => m.id !== mealId) } : p
          )
          .filter((p) => p.meals.length > 0)
      );
      toastStore.success("Meal removed from plan");
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to remove meal";
      toastStore.error(message);
      throw e;
    }
  },

  updateServings: async (date: string, mealId: string, servings: number) => {
    try {
      await updateMealPlanCmd(mealId, servings);

      update((plans) =>
        plans.map((p) =>
          p.date === date
            ? { ...p, meals: p.meals.map((m) => (m.id === mealId ? { ...m, servings } : m)) }
            : p
        )
      );
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to update servings";
      toastStore.error(message);
      throw e;
    }
  },
};

// Derived store remains the same
export const mealPlanByDate = derived(mealPlanStore, ($plans) => {
  const map = new Map<string, MealPlan>();
  $plans.forEach((p) => map.set(p.date, p));
  return map;
});
```

#### Meal Plan Store Tests

**File**: `src/lib/stores/mealPlan.test.ts`

**Change**: New test file for meal plan operations

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { mealPlanStore, mealPlansLoading, mealPlanByDate } from "./mealPlan";

vi.mock("@tauri-apps/api/core");

describe("mealPlanStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("loads and groups meal plans by date", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      { id: "m1", date: "2025-01-01", mealType: "dinner", recipeId: "r1", servings: 4 },
      { id: "m2", date: "2025-01-01", mealType: "lunch", recipeId: "r2", servings: 2 },
      { id: "m3", date: "2025-01-02", mealType: "dinner", recipeId: "r1", servings: 4 },
    ]);

    await mealPlanStore.load("2025-01-01", "2025-01-07");

    const plans = get(mealPlanStore);
    expect(plans).toHaveLength(2);
    expect(plans[0].date).toBe("2025-01-01");
    expect(plans[0].meals).toHaveLength(2);
    expect(plans[1].date).toBe("2025-01-02");
    expect(plans[1].meals).toHaveLength(1);
  });

  it("adds meal to existing date", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce([{ id: "m1", date: "2025-01-01", mealType: "dinner", recipeId: "r1", servings: 4 }])
      .mockResolvedValueOnce({ id: "m2", date: "2025-01-01", mealType: "lunch", recipeId: "r2", servings: 2 });

    await mealPlanStore.load("2025-01-01", "2025-01-07");
    await mealPlanStore.addMeal("2025-01-01", "r2", "lunch", 2);

    const plans = get(mealPlanStore);
    expect(plans[0].meals).toHaveLength(2);
  });

  it("removes meal and cleans up empty dates", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce([{ id: "m1", date: "2025-01-01", mealType: "dinner", recipeId: "r1", servings: 4 }])
      .mockResolvedValueOnce(undefined);

    await mealPlanStore.load("2025-01-01", "2025-01-07");
    await mealPlanStore.removeMeal("2025-01-01", "m1");

    expect(get(mealPlanStore)).toHaveLength(0);
  });

  it("mealPlanByDate creates lookup map", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      { id: "m1", date: "2025-01-01", mealType: "dinner", recipeId: "r1", servings: 4 },
    ]);

    await mealPlanStore.load("2025-01-01", "2025-01-07");

    const map = get(mealPlanByDate);
    expect(map.has("2025-01-01")).toBe(true);
    expect(map.has("2025-01-02")).toBe(false);
  });
});
```

#### Update Barrel Export

**File**: `src/lib/stores/index.ts`

**Change**: Export loading/error states

```typescript
export { mealPlanStore, mealPlanByDate, mealPlansLoading, mealPlansError } from "./mealPlan";
```

### Success Criteria

#### Automated Verification
- [x] Tests pass: `pnpm test -- mealPlan`
- [x] Types check: `pnpm check`

#### Integration Verification
- [x] `mealPlansLoading` exportable from `$lib/stores`
- [x] Calendar component loads meal plans on mount

#### Manual Verification
- [ ] Meal plan calendar shows loading state
- [ ] Add recipe to meal plan — persists after refresh
- [ ] Remove meal from plan — persists after refresh
- [ ] Update servings — persists after refresh
- [ ] Navigate to different weeks — loads correct data

**Checkpoint**: Pause for manual verification before proceeding to Phase 4.

---

## Phase 4: Shopping Lists Integration

### Goal
Wire all shopping-related stores (weeklyShoppingListsStore, manualItemsStore, aggregatedShoppingList) to backend.

### Integration Points

**Consumes from Phase 3**: Meal plan data for aggregation, established patterns
**Produces for next phase**: Working shopping list persistence

**Wiring required**:
- [x] Update `src/lib/stores/shoppingList.ts` — multiple stores
- [x] Handle type mapping (`isOnHand` ↔ `isChecked`)
- [x] Wire `aggregatedShoppingList` to backend aggregation command
- [x] Create `src/lib/stores/shoppingList.test.ts`

### Changes

#### Shopping List Store Update

**File**: `src/lib/stores/shoppingList.ts`

**Change**: Replace mock data, wire to backend. This is the most complex store with multiple sub-stores.

Key changes:
1. `manualItemsStore` — wire to manual item commands
2. `weeklyShoppingListsStore` — wire to shopping list commands
3. `aggregatedShoppingList` — use backend aggregation OR keep derived (backend has `get_aggregated_shopping_list`)

```typescript
import { writable, derived, get } from "svelte/store";
import type { ShoppingItem, QuickList, WeeklyShoppingLists, ShoppingList } from "$lib/types";
import {
  getShoppingLists,
  createShoppingList,
  deleteShoppingList,
  addShoppingItem,
  updateShoppingItem,
  softDeleteShoppingItem,
  restoreShoppingItem,
  moveShoppingItem,
  getManualItems,
  createManualItem,
  updateManualItem,
  deleteManualItem,
  getAggregatedShoppingList,
} from "$lib/tauri/commands";
import { toastStore } from "./toast";
import { mealPlanStore, mealPlanByDate } from "./mealPlan";
import { recipeById } from "./recipes";

// Remove all mock data

// Loading states
export const shoppingListsLoading = writable(false);
export const manualItemsLoading = writable(false);

// ============ Manual Items Store ============

const manualItemsInternal = writable<ShoppingItem[]>([]);

async function loadManualItems(weekStart: string) {
  manualItemsLoading.set(true);
  try {
    const items = await getManualItems(weekStart);
    // Map backend ManualItem to frontend ShoppingItem
    manualItemsInternal.set(items.map((item) => ({
      id: item.id,
      name: item.name,
      quantity: item.quantity,
      unit: item.unit,
      category: item.category,
      isOnHand: item.isChecked, // Backend uses isChecked
      isManual: true,
      sourceRecipeIds: [],
    })));
  } catch (e) {
    toastStore.error("Failed to load manual items");
  } finally {
    manualItemsLoading.set(false);
  }
}

export const manualItemsStore = {
  subscribe: manualItemsInternal.subscribe,
  load: loadManualItems,

  add: async (weekStart: string, item: Omit<ShoppingItem, "id" | "isManual" | "sourceRecipeIds">) => {
    try {
      const created = await createManualItem({
        weekStart,
        name: item.name,
        quantity: item.quantity,
        unit: item.unit,
        category: item.category,
      });
      manualItemsInternal.update((items) => [...items, {
        id: created.id,
        name: created.name,
        quantity: created.quantity,
        unit: created.unit,
        category: created.category,
        isOnHand: created.isChecked,
        isManual: true,
        sourceRecipeIds: [],
      }]);
    } catch (e) {
      toastStore.error("Failed to add item");
      throw e;
    }
  },

  remove: async (id: string) => {
    try {
      await deleteManualItem(id);
      manualItemsInternal.update((items) => items.filter((i) => i.id !== id));
    } catch (e) {
      toastStore.error("Failed to remove item");
      throw e;
    }
  },

  toggleOnHand: async (id: string) => {
    const items = get(manualItemsInternal);
    const item = items.find((i) => i.id === id);
    if (!item) return;

    try {
      await updateManualItem(id, undefined, !item.isOnHand);
      manualItemsInternal.update((items) =>
        items.map((i) => (i.id === id ? { ...i, isOnHand: !i.isOnHand } : i))
      );
    } catch (e) {
      toastStore.error("Failed to update item");
      throw e;
    }
  },

  updateQuantity: async (id: string, quantity: number) => {
    try {
      await updateManualItem(id, quantity, undefined);
      manualItemsInternal.update((items) =>
        items.map((i) => (i.id === id ? { ...i, quantity } : i))
      );
    } catch (e) {
      toastStore.error("Failed to update quantity");
      throw e;
    }
  },
};

// ============ Weekly Shopping Lists Store ============
// Similar pattern - load from backend, wire CRUD operations
// This is extensive - showing key structure

const weeklyListsInternal = writable<WeeklyShoppingLists[]>([]);

export const weeklyShoppingListsStore = {
  subscribe: weeklyListsInternal.subscribe,

  load: async (weekStart: string) => {
    shoppingListsLoading.set(true);
    try {
      const lists = await getShoppingLists(weekStart);
      // Transform backend ShoppingListWithItems to frontend WeeklyShoppingLists
      weeklyListsInternal.update((weeks) => {
        const existing = weeks.filter((w) => w.weekStart !== weekStart);
        return [...existing, {
          weekStart,
          lists: lists.map((list) => ({
            id: list.id,
            name: list.name,
            type: list.listType as "weekly" | "midweek" | "custom",
            items: list.items.map((item) => ({
              id: item.id,
              name: item.name,
              quantity: item.quantity,
              unit: item.unit,
              category: item.category,
              isOnHand: item.isChecked,
              isManual: false,
              sourceRecipeIds: item.sourceRecipeIds?.split(",") ?? [],
              isDeleted: item.isDeleted,
              deletedAt: item.deletedAt,
              movedToListId: item.movedToListId,
            })),
          })),
        }];
      });
    } catch (e) {
      toastStore.error("Failed to load shopping lists");
    } finally {
      shoppingListsLoading.set(false);
    }
  },

  // ... other methods following same pattern
  // addList, removeList, addItem, moveItem, toggleItemOnHand,
  // removeItem, softDeleteItem, restoreItem, markItemMoved
};

// ============ Aggregated Shopping List ============
// Use backend aggregation for accuracy

export const aggregatedShoppingList = derived(
  [mealPlanStore, recipeById, manualItemsInternal],
  ([$mealPlans, $recipeMap, $manualItems], set) => {
    // For now, keep client-side derivation
    // Could switch to backend call for complex aggregation
    const aggregated = new Map<string, ShoppingItem>();

    // ... existing aggregation logic

    set([...Array.from(aggregated.values()), ...$manualItems]);
  },
  [] as ShoppingItem[]
);

// Export helper
export { getWeekStart } from "./shoppingList"; // Keep existing helper
```

#### Shopping List Store Tests

**File**: `src/lib/stores/shoppingList.test.ts`

**Change**: New test file covering manual items and weekly lists

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { manualItemsStore, weeklyShoppingListsStore } from "./shoppingList";

vi.mock("@tauri-apps/api/core");

describe("manualItemsStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("loads manual items and maps isChecked to isOnHand", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      { id: "1", weekStart: "2025-01-01", name: "Milk", quantity: 1, unit: "gallon", category: "Dairy", isChecked: true, createdAt: "" },
    ]);

    await manualItemsStore.load("2025-01-01");

    const items = get(manualItemsStore);
    expect(items[0].isOnHand).toBe(true);
    expect(items[0].isManual).toBe(true);
  });

  it("adds manual item via backend", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([]); // Initial load
    await manualItemsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce({
      id: "2", weekStart: "2025-01-01", name: "Bread", quantity: 1, unit: "loaf", category: "Bakery", isChecked: false, createdAt: "",
    });

    await manualItemsStore.add("2025-01-01", { name: "Bread", quantity: 1, unit: "loaf", category: "Bakery", isOnHand: false });

    expect(invoke).toHaveBeenCalledWith("create_manual_item", expect.objectContaining({
      input: expect.objectContaining({ name: "Bread" }),
    }));
  });

  it("toggles isOnHand via backend isChecked", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      { id: "1", weekStart: "2025-01-01", name: "Milk", quantity: 1, unit: "gallon", category: "Dairy", isChecked: false, createdAt: "" },
    ]);
    await manualItemsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce({ id: "1", isChecked: true });
    await manualItemsStore.toggleOnHand("1");

    expect(invoke).toHaveBeenCalledWith("update_manual_item", { id: "1", quantity: undefined, isChecked: true });
  });
});

describe("weeklyShoppingListsStore", () => {
  it("loads and transforms shopping lists", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "list1",
        weekStart: "2025-01-01",
        name: "Weekly",
        listType: "weekly",
        createdAt: "",
        items: [
          { id: "item1", listId: "list1", name: "Eggs", quantity: 12, unit: "count", category: "Dairy", isChecked: true, isDeleted: false, createdAt: "" },
        ],
      },
    ]);

    await weeklyShoppingListsStore.load("2025-01-01");

    // Verify transformation occurred
    expect(invoke).toHaveBeenCalledWith("get_shopping_lists", { weekStart: "2025-01-01" });
  });
});
```

### Success Criteria

#### Automated Verification
- [x] Tests pass: `pnpm test -- shoppingList`
- [x] Types check: `pnpm check`

#### Integration Verification
- [x] Shopping stores load data on ShoppingSection mount
- [x] Manual items persist independently of recipe-derived items

#### Manual Verification
- [ ] Add manual item — persists after refresh
- [ ] Check/uncheck item — persists after refresh
- [ ] Move item between lists — persists after refresh
- [ ] Soft delete and restore — works correctly
- [ ] Aggregated list shows items from meal plan

**Checkpoint**: Pause for manual verification before proceeding to Phase 5.

---

## Phase 5: Quick Lists Integration

### Goal
Wire quick lists store to backend, completing all store integrations.

### Integration Points

**Consumes from Phase 4**: Shopping list commands for "add to shopping" feature
**Produces for next phase**: All stores integrated, ready for app initialization

**Wiring required**:
- [x] Update quick lists portion of `src/lib/stores/shoppingList.ts`
- [x] Wire "add to shopping list" feature
- [x] Create quick lists tests in `src/lib/stores/shoppingList.test.ts`

### Changes

#### Quick Lists Store Update

**File**: `src/lib/stores/shoppingList.ts`

**Change**: Wire quick lists to backend

```typescript
import {
  getQuickLists,
  createQuickList,
  updateQuickList,
  deleteQuickList,
  addQuickListItem,
  updateQuickListItem,
  removeQuickListItem,
  addQuickListToShopping,
} from "$lib/tauri/commands";

// Remove mockQuickLists

export const quickListsLoading = writable(false);

const quickListsInternal = writable<QuickList[]>([]);

async function loadQuickLists() {
  quickListsLoading.set(true);
  try {
    const lists = await getQuickLists();
    quickListsInternal.set(lists.map((list) => ({
      id: list.id,
      name: list.name,
      items: list.items.map((item) => ({
        id: item.id,
        name: item.name,
        quantity: item.quantity,
        unit: item.unit,
        category: item.category,
      })),
    })));
  } catch (e) {
    toastStore.error("Failed to load quick lists");
  } finally {
    quickListsLoading.set(false);
  }
}

export const quickListsStore = {
  subscribe: quickListsInternal.subscribe,
  load: loadQuickLists,

  addList: async (name: string) => {
    try {
      const created = await createQuickList(name);
      quickListsInternal.update((lists) => [...lists, { id: created.id, name: created.name, items: [] }]);
      toastStore.success("Quick list created");
    } catch (e) {
      toastStore.error("Failed to create quick list");
      throw e;
    }
  },

  removeList: async (id: string) => {
    try {
      await deleteQuickList(id);
      quickListsInternal.update((lists) => lists.filter((l) => l.id !== id));
      toastStore.success("Quick list deleted");
    } catch (e) {
      toastStore.error("Failed to delete quick list");
      throw e;
    }
  },

  renameList: async (id: string, name: string) => {
    try {
      await updateQuickList(id, name);
      quickListsInternal.update((lists) =>
        lists.map((l) => (l.id === id ? { ...l, name } : l))
      );
    } catch (e) {
      toastStore.error("Failed to rename quick list");
      throw e;
    }
  },

  addItem: async (listId: string, item: Omit<QuickListItem, "id">) => {
    try {
      const created = await addQuickListItem(listId, {
        name: item.name,
        quantity: item.quantity,
        unit: item.unit,
        category: item.category,
      });
      quickListsInternal.update((lists) =>
        lists.map((l) =>
          l.id === listId ? { ...l, items: [...l.items, { ...created }] } : l
        )
      );
    } catch (e) {
      toastStore.error("Failed to add item");
      throw e;
    }
  },

  removeItem: async (listId: string, itemId: string) => {
    try {
      await removeQuickListItem(itemId);
      quickListsInternal.update((lists) =>
        lists.map((l) =>
          l.id === listId ? { ...l, items: l.items.filter((i) => i.id !== itemId) } : l
        )
      );
    } catch (e) {
      toastStore.error("Failed to remove item");
      throw e;
    }
  },

  updateItem: async (listId: string, itemId: string, updates: Partial<QuickListItem>) => {
    const lists = get(quickListsInternal);
    const list = lists.find((l) => l.id === listId);
    const item = list?.items.find((i) => i.id === itemId);
    if (!item) return;

    try {
      await updateQuickListItem(itemId, {
        name: updates.name ?? item.name,
        quantity: updates.quantity ?? item.quantity,
        unit: updates.unit ?? item.unit,
        category: updates.category ?? item.category,
      });
      quickListsInternal.update((lists) =>
        lists.map((l) =>
          l.id === listId
            ? { ...l, items: l.items.map((i) => (i.id === itemId ? { ...i, ...updates } : i)) }
            : l
        )
      );
    } catch (e) {
      toastStore.error("Failed to update item");
      throw e;
    }
  },

  addToShoppingList: async (quickListId: string, shoppingListId: string) => {
    try {
      await addQuickListToShopping(quickListId, shoppingListId);
      toastStore.success("Items added to shopping list");
      // Reload shopping lists to reflect changes
      // weeklyShoppingListsStore.load(...) would be called by component
    } catch (e) {
      toastStore.error("Failed to add to shopping list");
      throw e;
    }
  },
};
```

#### Quick Lists Tests

**File**: `src/lib/stores/shoppingList.test.ts`

**Change**: Add quick lists tests

```typescript
describe("quickListsStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("loads quick lists from backend", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "ql1",
        name: "Pantry Staples",
        createdAt: "",
        updatedAt: "",
        items: [
          { id: "qli1", quickListId: "ql1", name: "Olive oil", quantity: 1, unit: "bottle", category: "Oils" },
        ],
      },
    ]);

    await quickListsStore.load();

    const lists = get(quickListsStore);
    expect(lists).toHaveLength(1);
    expect(lists[0].name).toBe("Pantry Staples");
    expect(lists[0].items).toHaveLength(1);
  });

  it("creates quick list via backend", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([]); // Initial load
    await quickListsStore.load();

    vi.mocked(invoke).mockResolvedValueOnce({ id: "ql2", name: "New List", createdAt: "", updatedAt: "" });
    await quickListsStore.addList("New List");

    expect(invoke).toHaveBeenCalledWith("create_quick_list", { name: "New List" });
    expect(get(quickListsStore)).toHaveLength(1);
  });

  it("adds items to shopping list via backend", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([]);
    await quickListsStore.addToShoppingList("ql1", "sl1");

    expect(invoke).toHaveBeenCalledWith("add_quick_list_to_shopping", {
      quickListId: "ql1",
      shoppingListId: "sl1",
    });
  });
});
```

### Success Criteria

#### Automated Verification
- [x] Tests pass: `pnpm test -- shoppingList`
- [x] Types check: `pnpm check`

#### Integration Verification
- [x] Quick lists load on QuickListsManager mount
- [x] "Add to shopping list" calls backend

#### Manual Verification
- [ ] Create quick list — persists after refresh
- [ ] Add items to quick list — persists
- [ ] "Add to shopping list" — items appear in shopping list
- [ ] Delete quick list — confirmation dialog, persists

**Checkpoint**: Pause for manual verification before proceeding to Phase 6.

---

## Phase 6: App Initialization & Final Integration

### Goal
Wire up data loading on app startup, ensure empty states work, run full test suite.

### Integration Points

**Consumes**: All prior phase outputs
**Produces**: Complete feature, fully functional app

**Wiring required**:
- [x] Add `onMount` data loading in `src/App.svelte`
- [x] Verify empty states in all list views
- [x] Run full test suite
- [ ] Manual end-to-end verification

### Changes

#### App Initialization

**File**: `src/App.svelte`

**Change**: Load initial data on mount

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { recipeStore } from "$lib/stores";
  import { quickListsStore } from "$lib/stores";
  import ToastContainer from "$lib/components/shared/ToastContainer.svelte";
  // ... existing imports

  onMount(async () => {
    // Load recipes first (other features depend on it)
    await recipeStore.load();
    // Load quick lists (independent of date)
    await quickListsStore.load();
    // Meal plans and shopping lists are loaded by their respective components
    // based on the currently viewed week
  });
</script>

<!-- existing template -->

<ToastContainer />
```

#### Update Store Barrel Export

**File**: `src/lib/stores/index.ts`

**Change**: Final exports with all loading states

```typescript
export { appStore } from "./app";
export { toastStore } from "./toast";
export {
  recipeStore,
  recipeById,
  allIngredients,
  recipesLoading,
  recipesError,
  getRecipeProtein,
  getRecipeStarch,
  groupRecipes
} from "./recipes";
export { mealPlanStore, mealPlanByDate, mealPlansLoading, mealPlansError } from "./mealPlan";
export {
  manualItemsStore,
  manualItemsLoading,
  quickListsStore,
  quickListsLoading,
  aggregatedShoppingList,
  weeklyShoppingListsStore,
  shoppingListsLoading,
  createWeekAggregatedList,
  getWeekStart,
  softDeletedAggregatedStore,
} from "./shoppingList";
export { activeTab, type TabId } from "./navigation";
```

### Success Criteria

#### Automated Verification
- [x] Full test suite: `pnpm test`
- [x] Type check: `pnpm check`
- [x] Lint: `pnpm lint` (eslint not configured)
- [x] Rust tests: `pnpm test:rust`

#### Integration Verification
- [x] All stores importable from `$lib/stores`
- [ ] App compiles and runs: `pnpm tauri dev`
- [ ] No console errors on startup

#### Manual Verification
- [ ] Fresh database: app starts with empty states (no errors)
- [ ] Create recipe → add to meal plan → see in shopping list (full flow)
- [ ] Close and reopen app → all data persists
- [ ] Error scenarios: disconnect DB, verify error toasts appear
- [ ] Delete operations all show confirmation dialog
- [ ] Loading spinners appear during initial load

---

## Testing Strategy

### Unit Tests
- Each store file has corresponding `.test.ts`
- Test all CRUD operations
- Test derived stores compute correctly
- Test error handling paths

### Integration Tests
- Test store interactions (e.g., recipe deletion updates meal plans)
- Test type transformations between frontend/backend

### Manual Testing Checklist
1. [ ] Start with fresh database — empty states display
2. [ ] Create recipe with multiple ingredients
3. [ ] Add recipe to meal plan for 3 different days
4. [ ] View shopping list — aggregated ingredients appear
5. [ ] Check off items — persist after refresh
6. [ ] Create quick list, add items
7. [ ] Add quick list to shopping list
8. [ ] Add manual shopping item
9. [ ] Delete recipe — confirmation appears, persists
10. [ ] Force backend error — error toast appears

## Rollback Plan

Git revert to commit before Phase 1:
```bash
git revert --no-commit HEAD~N..HEAD
```

No data migration involved — SQLite schema unchanged.

## Migration Notes

- **Data migration**: None required (backend schema already exists)
- **Feature flags**: None (clean cutover)
- **Backwards compatibility**: N/A — mock data removed entirely

## References

- Ticket: `ai_docs/prompts/2025-12-16-frontend-backend-integration.md`
- Related research: `ai_docs/research/2025-12-15-development-roadmap.md`
- Similar implementation: `src/lib/components/recipes/ImportRecipe.svelte:9-18` (loading pattern)
