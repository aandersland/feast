# Meal Planning UI Implementation Plan

## Overview

Build a complete, interactive UI prototype for a meal planning application with four main views: Dashboard, Recipes, Meal Plan, and Shopping List. This is a UI-first iteration phase using mock data to lock down visual design and interaction patterns before backend integration.

## Current State

The project is a freshly scaffolded Tauri v2 + Svelte 5 + Tailwind CSS v4 app with minimal UI. Only a greeting component exists.

**Key Discoveries**:
- Svelte 5 runes pattern (`$state`, `$derived`) used in `src/App.svelte:5-6`
- Store factory pattern with `writable` in `src/lib/stores/app.ts:1-20`
- Tailwind CSS v4 via `@import "tailwindcss"` in `src/app.css:1`
- Single-page architecture (no SvelteKit routing) — `src/App.svelte` is the shell
- `$lib` alias configured for clean imports
- TypeScript interfaces in `src/lib/types/` with barrel export

## Desired End State

A fully interactive UI prototype where users can:
- Navigate between Dashboard, Recipes, Meal Plan, and Shopping List tabs
- Browse recipes, view details, access create/import forms
- Add recipes to a meal plan calendar via click-to-add workflow
- View an aggregated shopping list with manual additions and quick lists
- Experience polished, modern styling with proper interactive states

All data is mock data; no backend integration.

## What We're NOT Doing

- Backend/database integration
- Tauri commands and IPC beyond existing scaffold
- Actual recipe import parsing logic
- User authentication
- Data persistence (mock data only)
- Mobile responsiveness (desktop-only)

## Integration Map

| Type | Location | Notes |
|------|----------|-------|
| Entry point | `src/App.svelte` | Becomes navigation shell with tab routing |
| Components | `src/lib/components/` | All new UI components |
| Stores | `src/lib/stores/index.ts` | Barrel export for new stores |
| Types | `src/lib/types/index.ts` | Barrel export for new interfaces |
| Styles | `src/app.css` | Custom CSS variables if needed |

## Implementation Approach

Build foundation first (types, stores, navigation), then implement each view progressively. Each phase produces a working, visually complete section. The click-to-add meal planning workflow uses native Svelte state management with modals instead of external drag-and-drop libraries.

---

## Phase 1: Foundation

### Goal
Establish TypeScript types, mock data stores, and tab navigation shell.

### Integration Points

**Depends on**: Existing scaffold
**Produces for next phase**: Types, stores, navigation shell

**Wiring required**:
- [ ] Export all types from `src/lib/types/index.ts`
- [ ] Export all stores from `src/lib/stores/index.ts`
- [ ] Replace `src/App.svelte` content with navigation shell

### Changes

#### TypeScript Types

**File**: `src/lib/types/recipe.ts`

**Change**: Define recipe-related interfaces

```typescript
export interface Recipe {
  id: string;
  name: string;
  description: string;
  ingredients: Ingredient[];
  instructions: string[];
  prepTime: number; // minutes
  cookTime: number; // minutes
  servings: number;
  nutrition?: NutritionInfo;
  sourceUrl?: string;
  notes?: string;
  tags: string[];
  imageUrl?: string;
  createdAt: string;
}

export interface Ingredient {
  id: string;
  name: string;
  quantity: number;
  unit: string;
  notes?: string;
}

export interface NutritionInfo {
  calories: number;
  protein: number;
  carbs: number;
  fat: number;
  fiber?: number;
  sodium?: number;
}
```

**File**: `src/lib/types/mealPlan.ts`

**Change**: Define meal plan interfaces

```typescript
export interface MealPlan {
  id: string;
  date: string; // ISO date string YYYY-MM-DD
  meals: PlannedMeal[];
}

export interface PlannedMeal {
  id: string;
  recipeId: string;
  mealType: MealType;
  servings: number;
}

export type MealType = 'breakfast' | 'lunch' | 'dinner' | 'snack';
```

**File**: `src/lib/types/shoppingList.ts`

**Change**: Define shopping list interfaces

```typescript
export interface ShoppingItem {
  id: string;
  name: string;
  quantity: number;
  unit: string;
  category: string;
  isOnHand: boolean;
  isManual: boolean; // true if manually added, false if from recipe
  sourceRecipeIds: string[]; // recipes this ingredient came from
}

export interface QuickList {
  id: string;
  name: string;
  items: QuickListItem[];
}

export interface QuickListItem {
  name: string;
  quantity: number;
  unit: string;
  category: string;
}
```

**File**: `src/lib/types/index.ts`

**Change**: Update barrel export

```typescript
export { appStore } from "./app";
export type { Item } from "./items";
export type {
  Recipe,
  Ingredient,
  NutritionInfo,
} from "./recipe";
export type {
  MealPlan,
  PlannedMeal,
  MealType,
} from "./mealPlan";
export type {
  ShoppingItem,
  QuickList,
  QuickListItem,
} from "./shoppingList";
```

#### Mock Data Stores

**File**: `src/lib/stores/recipes.ts`

**Change**: Create recipe store with mock data

```typescript
import { writable, derived } from "svelte/store";
import type { Recipe } from "$lib/types";

const mockRecipes: Recipe[] = [
  {
    id: "1",
    name: "Spaghetti Carbonara",
    description: "Classic Italian pasta with eggs, cheese, and pancetta",
    ingredients: [
      { id: "1", name: "Spaghetti", quantity: 400, unit: "g" },
      { id: "2", name: "Pancetta", quantity: 200, unit: "g" },
      { id: "3", name: "Eggs", quantity: 4, unit: "" },
      { id: "4", name: "Parmesan", quantity: 100, unit: "g" },
      { id: "5", name: "Black pepper", quantity: 1, unit: "tsp" },
    ],
    instructions: [
      "Cook pasta according to package directions",
      "Fry pancetta until crispy",
      "Beat eggs with parmesan",
      "Combine hot pasta with pancetta, remove from heat",
      "Add egg mixture and toss quickly",
      "Season with pepper and serve",
    ],
    prepTime: 10,
    cookTime: 20,
    servings: 4,
    nutrition: { calories: 650, protein: 28, carbs: 72, fat: 28 },
    tags: ["Italian", "Pasta", "Quick"],
    createdAt: "2024-01-15",
  },
  // Additional mock recipes...
];

function createRecipeStore() {
  const { subscribe, set, update } = writable<Recipe[]>(mockRecipes);

  return {
    subscribe,
    add: (recipe: Recipe) => update((recipes) => [...recipes, recipe]),
    remove: (id: string) => update((recipes) => recipes.filter((r) => r.id !== id)),
    update: (id: string, data: Partial<Recipe>) =>
      update((recipes) =>
        recipes.map((r) => (r.id === id ? { ...r, ...data } : r))
      ),
  };
}

export const recipeStore = createRecipeStore();

export const recipeById = derived(recipeStore, ($recipes) => {
  const map = new Map<string, Recipe>();
  $recipes.forEach((r) => map.set(r.id, r));
  return map;
});
```

**File**: `src/lib/stores/mealPlan.ts`

**Change**: Create meal plan store with mock data

```typescript
import { writable, derived } from "svelte/store";
import type { MealPlan, PlannedMeal, MealType } from "$lib/types";

// Generate current week's dates
function getCurrentWeekDates(): string[] {
  const today = new Date();
  const monday = new Date(today);
  monday.setDate(today.getDate() - today.getDay() + 1);

  return Array.from({ length: 7 }, (_, i) => {
    const date = new Date(monday);
    date.setDate(monday.getDate() + i);
    return date.toISOString().split("T")[0];
  });
}

const weekDates = getCurrentWeekDates();

const mockMealPlans: MealPlan[] = [
  {
    id: "1",
    date: weekDates[0],
    meals: [
      { id: "m1", recipeId: "1", mealType: "dinner", servings: 4 },
    ],
  },
  {
    id: "2",
    date: weekDates[2],
    meals: [
      { id: "m2", recipeId: "2", mealType: "lunch", servings: 2 },
      { id: "m3", recipeId: "3", mealType: "dinner", servings: 4 },
    ],
  },
];

function createMealPlanStore() {
  const { subscribe, set, update } = writable<MealPlan[]>(mockMealPlans);

  return {
    subscribe,
    addMeal: (date: string, recipeId: string, mealType: MealType, servings: number) =>
      update((plans) => {
        const existing = plans.find((p) => p.date === date);
        const newMeal: PlannedMeal = {
          id: crypto.randomUUID(),
          recipeId,
          mealType,
          servings,
        };

        if (existing) {
          return plans.map((p) =>
            p.date === date ? { ...p, meals: [...p.meals, newMeal] } : p
          );
        }

        return [...plans, { id: crypto.randomUUID(), date, meals: [newMeal] }];
      }),
    removeMeal: (date: string, mealId: string) =>
      update((plans) =>
        plans.map((p) =>
          p.date === date
            ? { ...p, meals: p.meals.filter((m) => m.id !== mealId) }
            : p
        ).filter((p) => p.meals.length > 0)
      ),
    updateServings: (date: string, mealId: string, servings: number) =>
      update((plans) =>
        plans.map((p) =>
          p.date === date
            ? {
                ...p,
                meals: p.meals.map((m) =>
                  m.id === mealId ? { ...m, servings } : m
                ),
              }
            : p
        )
      ),
  };
}

export const mealPlanStore = createMealPlanStore();

export const mealPlanByDate = derived(mealPlanStore, ($plans) => {
  const map = new Map<string, MealPlan>();
  $plans.forEach((p) => map.set(p.date, p));
  return map;
});
```

**File**: `src/lib/stores/shoppingList.ts`

**Change**: Create shopping list store with aggregation logic

```typescript
import { writable, derived } from "svelte/store";
import type { ShoppingItem, QuickList } from "$lib/types";
import { mealPlanStore } from "./mealPlan";
import { recipeById } from "./recipes";

const mockQuickLists: QuickList[] = [
  {
    id: "1",
    name: "Pantry Staples",
    items: [
      { name: "Olive oil", quantity: 1, unit: "bottle", category: "Oils" },
      { name: "Salt", quantity: 1, unit: "container", category: "Spices" },
      { name: "Black pepper", quantity: 1, unit: "container", category: "Spices" },
    ],
  },
  {
    id: "2",
    name: "Weekly Basics",
    items: [
      { name: "Milk", quantity: 1, unit: "gallon", category: "Dairy" },
      { name: "Eggs", quantity: 1, unit: "dozen", category: "Dairy" },
      { name: "Bread", quantity: 1, unit: "loaf", category: "Bakery" },
    ],
  },
];

function createManualItemsStore() {
  const { subscribe, set, update } = writable<ShoppingItem[]>([]);

  return {
    subscribe,
    add: (item: Omit<ShoppingItem, "id" | "isManual" | "sourceRecipeIds">) =>
      update((items) => [
        ...items,
        { ...item, id: crypto.randomUUID(), isManual: true, sourceRecipeIds: [] },
      ]),
    remove: (id: string) => update((items) => items.filter((i) => i.id !== id)),
    toggleOnHand: (id: string) =>
      update((items) =>
        items.map((i) => (i.id === id ? { ...i, isOnHand: !i.isOnHand } : i))
      ),
    updateQuantity: (id: string, quantity: number) =>
      update((items) =>
        items.map((i) => (i.id === id ? { ...i, quantity } : i))
      ),
  };
}

export const manualItemsStore = createManualItemsStore();
export const quickListsStore = writable<QuickList[]>(mockQuickLists);

// Derived store that aggregates ingredients from meal plan
export const aggregatedShoppingList = derived(
  [mealPlanStore, recipeById, manualItemsStore],
  ([$mealPlans, $recipeMap, $manualItems]) => {
    const aggregated = new Map<string, ShoppingItem>();

    // Aggregate from meal plans
    $mealPlans.forEach((plan) => {
      plan.meals.forEach((meal) => {
        const recipe = $recipeMap.get(meal.recipeId);
        if (!recipe) return;

        const multiplier = meal.servings / recipe.servings;

        recipe.ingredients.forEach((ing) => {
          const key = `${ing.name.toLowerCase()}-${ing.unit}`;
          const existing = aggregated.get(key);

          if (existing) {
            existing.quantity += ing.quantity * multiplier;
            if (!existing.sourceRecipeIds.includes(recipe.id)) {
              existing.sourceRecipeIds.push(recipe.id);
            }
          } else {
            aggregated.set(key, {
              id: key,
              name: ing.name,
              quantity: ing.quantity * multiplier,
              unit: ing.unit,
              category: "Grocery", // Default category
              isOnHand: false,
              isManual: false,
              sourceRecipeIds: [recipe.id],
            });
          }
        });
      });
    });

    // Combine with manual items
    return [...Array.from(aggregated.values()), ...$manualItems];
  }
);
```

**File**: `src/lib/stores/navigation.ts`

**Change**: Create navigation state store

```typescript
import { writable } from "svelte/store";

export type TabId = "dashboard" | "recipes" | "mealplan" | "shopping";

export const activeTab = writable<TabId>("dashboard");
```

**File**: `src/lib/stores/index.ts`

**Change**: Update barrel export

```typescript
export { appStore } from "./app";
export { recipeStore, recipeById } from "./recipes";
export { mealPlanStore, mealPlanByDate } from "./mealPlan";
export {
  manualItemsStore,
  quickListsStore,
  aggregatedShoppingList,
} from "./shoppingList";
export { activeTab, type TabId } from "./navigation";
```

#### Navigation Shell

**File**: `src/lib/components/TabNavigation.svelte`

**Change**: Create tab navigation component

```svelte
<script lang="ts">
  import { activeTab, type TabId } from "$lib/stores";

  const tabs: { id: TabId; label: string }[] = [
    { id: "dashboard", label: "Dashboard" },
    { id: "recipes", label: "Recipes" },
    { id: "mealplan", label: "Meal Plan" },
    { id: "shopping", label: "Shopping List" },
  ];
</script>

<nav class="bg-white border-b border-gray-200">
  <div class="flex space-x-1 px-4">
    {#each tabs as tab}
      <button
        type="button"
        onclick={() => activeTab.set(tab.id)}
        class="px-4 py-3 text-sm font-medium transition-colors relative
          {$activeTab === tab.id
            ? 'text-emerald-600'
            : 'text-gray-500 hover:text-gray-700'}"
      >
        {tab.label}
        {#if $activeTab === tab.id}
          <span class="absolute bottom-0 left-0 right-0 h-0.5 bg-emerald-600"></span>
        {/if}
      </button>
    {/each}
  </div>
</nav>
```

**File**: `src/App.svelte`

**Change**: Replace with navigation shell

```svelte
<script lang="ts">
  import TabNavigation from "$lib/components/TabNavigation.svelte";
  import { activeTab } from "$lib/stores";

  // Placeholder components - will be replaced in later phases
  import Dashboard from "$lib/components/Dashboard.svelte";
  import Recipes from "$lib/components/Recipes.svelte";
  import MealPlan from "$lib/components/MealPlan.svelte";
  import ShoppingList from "$lib/components/ShoppingList.svelte";
</script>

<div class="min-h-screen bg-gray-50 flex flex-col">
  <header class="bg-white shadow-sm">
    <div class="px-6 py-4">
      <h1 class="text-2xl font-bold text-emerald-600">feast</h1>
    </div>
    <TabNavigation />
  </header>

  <main class="flex-1 p-6">
    {#if $activeTab === "dashboard"}
      <Dashboard />
    {:else if $activeTab === "recipes"}
      <Recipes />
    {:else if $activeTab === "mealplan"}
      <MealPlan />
    {:else if $activeTab === "shopping"}
      <ShoppingList />
    {/if}
  </main>
</div>
```

#### Placeholder View Components

Create placeholder components for each view (will be fully implemented in later phases):

**File**: `src/lib/components/Dashboard.svelte`

```svelte
<div class="text-center py-12 text-gray-500">
  Dashboard coming soon...
</div>
```

**File**: `src/lib/components/Recipes.svelte`

```svelte
<div class="text-center py-12 text-gray-500">
  Recipes coming soon...
</div>
```

**File**: `src/lib/components/MealPlan.svelte`

```svelte
<div class="text-center py-12 text-gray-500">
  Meal Plan coming soon...
</div>
```

**File**: `src/lib/components/ShoppingList.svelte`

```svelte
<div class="text-center py-12 text-gray-500">
  Shopping List coming soon...
</div>
```

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`

#### Integration Verification
- [ ] All types importable from `$lib/types`
- [ ] All stores importable from `$lib/stores`
- [ ] App compiles without errors: `pnpm build`

#### Manual Verification
- [ ] App launches with `pnpm tauri dev`
- [ ] Header displays "feast" branding
- [ ] All 4 tabs visible and clickable
- [ ] Active tab indicator updates on click
- [ ] Content area switches between placeholder views

**Checkpoint**: Pause for manual verification before proceeding to Phase 2.

---

## Phase 2: Dashboard

### Goal
Build the dashboard view with weekly calendar widget and supplementary widgets.

### Integration Points

**Consumes from Phase 1**: `mealPlanByDate`, `recipeById`, `activeTab` from `$lib/stores`
**Produces for next phase**: Reusable calendar components, widget patterns

**Wiring required**:
- [ ] Replace placeholder in `src/lib/components/Dashboard.svelte`
- [ ] Create sub-components in `src/lib/components/dashboard/`

### Changes

#### Dashboard Components

**File**: `src/lib/components/dashboard/WeeklyCalendar.svelte`

**Change**: Create weekly calendar grid showing planned meals

```svelte
<script lang="ts">
  import { mealPlanByDate, recipeById } from "$lib/stores";
  import type { MealType } from "$lib/types";

  const mealTypeOrder: MealType[] = ["breakfast", "lunch", "dinner", "snack"];
  const mealTypeLabels: Record<MealType, string> = {
    breakfast: "Breakfast",
    lunch: "Lunch",
    dinner: "Dinner",
    snack: "Snack",
  };

  function getWeekDates(): { date: string; dayName: string; dayNum: number }[] {
    const today = new Date();
    const monday = new Date(today);
    monday.setDate(today.getDate() - today.getDay() + 1);

    return Array.from({ length: 7 }, (_, i) => {
      const date = new Date(monday);
      date.setDate(monday.getDate() + i);
      return {
        date: date.toISOString().split("T")[0],
        dayName: date.toLocaleDateString("en-US", { weekday: "short" }),
        dayNum: date.getDate(),
      };
    });
  }

  const weekDates = getWeekDates();
  const today = new Date().toISOString().split("T")[0];
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
  <div class="px-6 py-4 border-b border-gray-100">
    <h2 class="text-lg font-semibold text-gray-800">This Week</h2>
  </div>

  <div class="grid grid-cols-7 divide-x divide-gray-100">
    {#each weekDates as { date, dayName, dayNum }}
      {@const plan = $mealPlanByDate.get(date)}
      {@const isToday = date === today}

      <div class="min-h-[200px] {isToday ? 'bg-emerald-50/50' : ''}">
        <div class="px-3 py-2 border-b border-gray-100 text-center">
          <div class="text-xs text-gray-500 uppercase">{dayName}</div>
          <div class="text-lg font-semibold {isToday ? 'text-emerald-600' : 'text-gray-800'}">
            {dayNum}
          </div>
        </div>

        <div class="p-2 space-y-1">
          {#if plan}
            {#each mealTypeOrder as mealType}
              {#each plan.meals.filter(m => m.mealType === mealType) as meal}
                {@const recipe = $recipeById.get(meal.recipeId)}
                {#if recipe}
                  <div class="text-xs p-2 rounded-lg bg-emerald-100 text-emerald-800 truncate">
                    <span class="font-medium">{recipe.name}</span>
                  </div>
                {/if}
              {/each}
            {/each}
          {:else}
            <div class="text-xs text-gray-400 text-center py-4">No meals</div>
          {/if}
        </div>
      </div>
    {/each}
  </div>
</div>
```

**File**: `src/lib/components/dashboard/QuickStats.svelte`

**Change**: Create quick stats widget

```svelte
<script lang="ts">
  import { mealPlanStore, aggregatedShoppingList } from "$lib/stores";
  import { derived } from "svelte/store";

  const stats = derived(
    [mealPlanStore, aggregatedShoppingList],
    ([$plans, $items]) => ({
      mealsPlanned: $plans.reduce((acc, p) => acc + p.meals.length, 0),
      shoppingItems: $items.filter((i) => !i.isOnHand).length,
      itemsOnHand: $items.filter((i) => i.isOnHand).length,
    })
  );
</script>

<div class="grid grid-cols-3 gap-4">
  <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-4">
    <div class="text-3xl font-bold text-emerald-600">{$stats.mealsPlanned}</div>
    <div class="text-sm text-gray-500">Meals planned</div>
  </div>

  <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-4">
    <div class="text-3xl font-bold text-blue-600">{$stats.shoppingItems}</div>
    <div class="text-sm text-gray-500">Items to buy</div>
  </div>

  <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-4">
    <div class="text-3xl font-bold text-gray-600">{$stats.itemsOnHand}</div>
    <div class="text-sm text-gray-500">Items on hand</div>
  </div>
</div>
```

**File**: `src/lib/components/dashboard/RecentRecipes.svelte`

**Change**: Create recent recipes widget

```svelte
<script lang="ts">
  import { recipeStore, activeTab } from "$lib/stores";
  import { derived } from "svelte/store";

  const recentRecipes = derived(recipeStore, ($recipes) =>
    [...$recipes]
      .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
      .slice(0, 3)
  );
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
  <div class="px-6 py-4 border-b border-gray-100 flex justify-between items-center">
    <h2 class="text-lg font-semibold text-gray-800">Recent Recipes</h2>
    <button
      type="button"
      onclick={() => activeTab.set("recipes")}
      class="text-sm text-emerald-600 hover:text-emerald-700"
    >
      View all
    </button>
  </div>

  <div class="divide-y divide-gray-100">
    {#each $recentRecipes as recipe}
      <div class="px-6 py-3 hover:bg-gray-50 transition-colors">
        <div class="font-medium text-gray-800">{recipe.name}</div>
        <div class="text-sm text-gray-500">
          {recipe.prepTime + recipe.cookTime} min · {recipe.servings} servings
        </div>
      </div>
    {/each}
  </div>
</div>
```

**File**: `src/lib/components/Dashboard.svelte`

**Change**: Compose dashboard from widgets

```svelte
<script lang="ts">
  import WeeklyCalendar from "./dashboard/WeeklyCalendar.svelte";
  import QuickStats from "./dashboard/QuickStats.svelte";
  import RecentRecipes from "./dashboard/RecentRecipes.svelte";
</script>

<div class="space-y-6 max-w-6xl mx-auto">
  <QuickStats />
  <WeeklyCalendar />

  <div class="grid grid-cols-2 gap-6">
    <RecentRecipes />

    <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-6">
      <h2 class="text-lg font-semibold text-gray-800 mb-4">Quick Actions</h2>
      <div class="space-y-2">
        <button
          type="button"
          class="w-full text-left px-4 py-3 rounded-lg bg-emerald-50 text-emerald-700 hover:bg-emerald-100 transition-colors"
        >
          + Add new recipe
        </button>
        <button
          type="button"
          class="w-full text-left px-4 py-3 rounded-lg bg-blue-50 text-blue-700 hover:bg-blue-100 transition-colors"
        >
          Plan this week's meals
        </button>
        <button
          type="button"
          class="w-full text-left px-4 py-3 rounded-lg bg-gray-50 text-gray-700 hover:bg-gray-100 transition-colors"
        >
          View shopping list
        </button>
      </div>
    </div>
  </div>
</div>
```

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`

#### Integration Verification
- [ ] Dashboard component imports and renders
- [ ] Store subscriptions work correctly

#### Manual Verification
- [ ] Weekly calendar displays 7 days (Mon-Sun)
- [ ] Today's date is highlighted
- [ ] Mock meals appear on correct days
- [ ] Quick stats show correct counts
- [ ] Recent recipes list displays
- [ ] Quick action buttons have hover states

**Checkpoint**: Pause for manual verification before proceeding to Phase 3.

---

## Phase 3: Recipes

### Goal
Build recipe list view, detail view, create form, and import form.

### Integration Points

**Consumes from Phase 2**: Widget patterns, store subscriptions
**Produces for next phase**: Recipe components reusable in meal planning

**Wiring required**:
- [ ] Replace placeholder in `src/lib/components/Recipes.svelte`
- [ ] Create sub-components in `src/lib/components/recipes/`

### Changes

#### Recipe Components

**File**: `src/lib/components/recipes/RecipeCard.svelte`

**Change**: Create recipe card for list view

```svelte
<script lang="ts">
  import type { Recipe } from "$lib/types";

  interface Props {
    recipe: Recipe;
    onSelect: (recipe: Recipe) => void;
  }

  let { recipe, onSelect }: Props = $props();
</script>

<button
  type="button"
  onclick={() => onSelect(recipe)}
  class="w-full text-left bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden hover:shadow-md transition-shadow"
>
  {#if recipe.imageUrl}
    <div class="h-40 bg-gray-200">
      <img src={recipe.imageUrl} alt={recipe.name} class="w-full h-full object-cover" />
    </div>
  {:else}
    <div class="h-40 bg-gradient-to-br from-emerald-100 to-emerald-200 flex items-center justify-center">
      <span class="text-4xl">🍽️</span>
    </div>
  {/if}

  <div class="p-4">
    <h3 class="font-semibold text-gray-800 mb-1">{recipe.name}</h3>
    <p class="text-sm text-gray-500 line-clamp-2 mb-3">{recipe.description}</p>

    <div class="flex items-center gap-4 text-sm text-gray-500">
      <span>{recipe.prepTime + recipe.cookTime} min</span>
      <span>{recipe.servings} servings</span>
    </div>

    {#if recipe.tags.length > 0}
      <div class="flex flex-wrap gap-1 mt-3">
        {#each recipe.tags.slice(0, 3) as tag}
          <span class="px-2 py-0.5 text-xs rounded-full bg-gray-100 text-gray-600">
            {tag}
          </span>
        {/each}
      </div>
    {/if}
  </div>
</button>
```

**File**: `src/lib/components/recipes/RecipeDetail.svelte`

**Change**: Create recipe detail view

```svelte
<script lang="ts">
  import type { Recipe } from "$lib/types";

  interface Props {
    recipe: Recipe;
    onBack: () => void;
  }

  let { recipe, onBack }: Props = $props();
  let servingMultiplier = $state(1);
  let adjustedServings = $derived(recipe.servings * servingMultiplier);
</script>

<div class="max-w-4xl mx-auto">
  <button
    type="button"
    onclick={onBack}
    class="flex items-center gap-2 text-gray-600 hover:text-gray-800 mb-6"
  >
    <span>←</span> Back to recipes
  </button>

  <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
    {#if recipe.imageUrl}
      <div class="h-64 bg-gray-200">
        <img src={recipe.imageUrl} alt={recipe.name} class="w-full h-full object-cover" />
      </div>
    {:else}
      <div class="h-64 bg-gradient-to-br from-emerald-100 to-emerald-200 flex items-center justify-center">
        <span class="text-6xl">🍽️</span>
      </div>
    {/if}

    <div class="p-6">
      <h1 class="text-2xl font-bold text-gray-800 mb-2">{recipe.name}</h1>
      <p class="text-gray-600 mb-6">{recipe.description}</p>

      <div class="flex flex-wrap gap-4 mb-6 text-sm">
        <div class="flex items-center gap-2 px-3 py-2 bg-gray-100 rounded-lg">
          <span>⏱️</span>
          <span>Prep: {recipe.prepTime} min</span>
        </div>
        <div class="flex items-center gap-2 px-3 py-2 bg-gray-100 rounded-lg">
          <span>🍳</span>
          <span>Cook: {recipe.cookTime} min</span>
        </div>
        <div class="flex items-center gap-2 px-3 py-2 bg-gray-100 rounded-lg">
          <span>👥</span>
          <div class="flex items-center gap-2">
            <button
              type="button"
              onclick={() => servingMultiplier = Math.max(0.5, servingMultiplier - 0.5)}
              class="w-6 h-6 rounded bg-gray-200 hover:bg-gray-300"
            >
              -
            </button>
            <span>{adjustedServings} servings</span>
            <button
              type="button"
              onclick={() => servingMultiplier += 0.5}
              class="w-6 h-6 rounded bg-gray-200 hover:bg-gray-300"
            >
              +
            </button>
          </div>
        </div>
      </div>

      {#if recipe.tags.length > 0}
        <div class="flex flex-wrap gap-2 mb-6">
          {#each recipe.tags as tag}
            <span class="px-3 py-1 text-sm rounded-full bg-emerald-100 text-emerald-700">
              {tag}
            </span>
          {/each}
        </div>
      {/if}

      <div class="grid md:grid-cols-2 gap-8">
        <div>
          <h2 class="text-lg font-semibold text-gray-800 mb-4">Ingredients</h2>
          <ul class="space-y-2">
            {#each recipe.ingredients as ing}
              <li class="flex items-start gap-2">
                <span class="text-emerald-500 mt-0.5">•</span>
                <span>
                  {(ing.quantity * servingMultiplier).toFixed(ing.quantity * servingMultiplier % 1 === 0 ? 0 : 1)}
                  {ing.unit} {ing.name}
                  {#if ing.notes}
                    <span class="text-gray-500">({ing.notes})</span>
                  {/if}
                </span>
              </li>
            {/each}
          </ul>
        </div>

        <div>
          <h2 class="text-lg font-semibold text-gray-800 mb-4">Instructions</h2>
          <ol class="space-y-3">
            {#each recipe.instructions as step, i}
              <li class="flex gap-3">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-emerald-100 text-emerald-700 flex items-center justify-center text-sm font-medium">
                  {i + 1}
                </span>
                <span>{step}</span>
              </li>
            {/each}
          </ol>
        </div>
      </div>

      {#if recipe.nutrition}
        <div class="mt-8 p-4 bg-gray-50 rounded-lg">
          <h2 class="text-lg font-semibold text-gray-800 mb-3">Nutrition (per serving)</h2>
          <div class="grid grid-cols-4 gap-4 text-center">
            <div>
              <div class="text-xl font-bold text-gray-800">{recipe.nutrition.calories}</div>
              <div class="text-sm text-gray-500">Calories</div>
            </div>
            <div>
              <div class="text-xl font-bold text-gray-800">{recipe.nutrition.protein}g</div>
              <div class="text-sm text-gray-500">Protein</div>
            </div>
            <div>
              <div class="text-xl font-bold text-gray-800">{recipe.nutrition.carbs}g</div>
              <div class="text-sm text-gray-500">Carbs</div>
            </div>
            <div>
              <div class="text-xl font-bold text-gray-800">{recipe.nutrition.fat}g</div>
              <div class="text-sm text-gray-500">Fat</div>
            </div>
          </div>
        </div>
      {/if}

      {#if recipe.notes}
        <div class="mt-6 p-4 bg-yellow-50 rounded-lg border border-yellow-100">
          <h2 class="font-semibold text-yellow-800 mb-2">Notes</h2>
          <p class="text-yellow-700">{recipe.notes}</p>
        </div>
      {/if}

      {#if recipe.sourceUrl}
        <div class="mt-6">
          <a
            href={recipe.sourceUrl}
            target="_blank"
            rel="noopener noreferrer"
            class="text-emerald-600 hover:text-emerald-700 underline"
          >
            View original recipe →
          </a>
        </div>
      {/if}
    </div>
  </div>
</div>
```

**File**: `src/lib/components/recipes/RecipeForm.svelte`

**Change**: Create recipe create/edit form

```svelte
<script lang="ts">
  import type { Recipe, Ingredient } from "$lib/types";

  interface Props {
    onSave: (recipe: Omit<Recipe, "id" | "createdAt">) => void;
    onCancel: () => void;
  }

  let { onSave, onCancel }: Props = $props();

  let name = $state("");
  let description = $state("");
  let prepTime = $state(15);
  let cookTime = $state(30);
  let servings = $state(4);
  let ingredients = $state<Omit<Ingredient, "id">[]>([
    { name: "", quantity: 1, unit: "" },
  ]);
  let instructions = $state([""]);
  let tags = $state("");
  let notes = $state("");
  let sourceUrl = $state("");

  function addIngredient() {
    ingredients = [...ingredients, { name: "", quantity: 1, unit: "" }];
  }

  function removeIngredient(index: number) {
    ingredients = ingredients.filter((_, i) => i !== index);
  }

  function addInstruction() {
    instructions = [...instructions, ""];
  }

  function removeInstruction(index: number) {
    instructions = instructions.filter((_, i) => i !== index);
  }

  function handleSubmit() {
    onSave({
      name,
      description,
      prepTime,
      cookTime,
      servings,
      ingredients: ingredients
        .filter((i) => i.name.trim())
        .map((i, idx) => ({ ...i, id: String(idx) })),
      instructions: instructions.filter((i) => i.trim()),
      tags: tags.split(",").map((t) => t.trim()).filter(Boolean),
      notes: notes || undefined,
      sourceUrl: sourceUrl || undefined,
    });
  }
</script>

<div class="max-w-2xl mx-auto bg-white rounded-xl shadow-sm border border-gray-100 p-6">
  <h2 class="text-xl font-bold text-gray-800 mb-6">Add New Recipe</h2>

  <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-6">
    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Recipe Name</label>
      <input
        type="text"
        bind:value={name}
        required
        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
      />
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Description</label>
      <textarea
        bind:value={description}
        rows="2"
        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
      ></textarea>
    </div>

    <div class="grid grid-cols-3 gap-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Prep Time (min)</label>
        <input
          type="number"
          bind:value={prepTime}
          min="0"
          class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Cook Time (min)</label>
        <input
          type="number"
          bind:value={cookTime}
          min="0"
          class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Servings</label>
        <input
          type="number"
          bind:value={servings}
          min="1"
          class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
        />
      </div>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-2">Ingredients</label>
      <div class="space-y-2">
        {#each ingredients as ing, i}
          <div class="flex gap-2">
            <input
              type="number"
              bind:value={ing.quantity}
              min="0"
              step="0.25"
              class="w-20 px-3 py-2 border border-gray-300 rounded-lg"
              placeholder="Qty"
            />
            <input
              type="text"
              bind:value={ing.unit}
              class="w-24 px-3 py-2 border border-gray-300 rounded-lg"
              placeholder="Unit"
            />
            <input
              type="text"
              bind:value={ing.name}
              class="flex-1 px-3 py-2 border border-gray-300 rounded-lg"
              placeholder="Ingredient name"
            />
            <button
              type="button"
              onclick={() => removeIngredient(i)}
              class="px-3 py-2 text-red-500 hover:bg-red-50 rounded-lg"
            >
              ×
            </button>
          </div>
        {/each}
      </div>
      <button
        type="button"
        onclick={addIngredient}
        class="mt-2 text-sm text-emerald-600 hover:text-emerald-700"
      >
        + Add ingredient
      </button>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-2">Instructions</label>
      <div class="space-y-2">
        {#each instructions as step, i}
          <div class="flex gap-2">
            <span class="flex-shrink-0 w-8 h-10 flex items-center justify-center text-gray-500">
              {i + 1}.
            </span>
            <input
              type="text"
              bind:value={instructions[i]}
              class="flex-1 px-3 py-2 border border-gray-300 rounded-lg"
              placeholder="Step description"
            />
            <button
              type="button"
              onclick={() => removeInstruction(i)}
              class="px-3 py-2 text-red-500 hover:bg-red-50 rounded-lg"
            >
              ×
            </button>
          </div>
        {/each}
      </div>
      <button
        type="button"
        onclick={addInstruction}
        class="mt-2 text-sm text-emerald-600 hover:text-emerald-700"
      >
        + Add step
      </button>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Tags (comma-separated)</label>
      <input
        type="text"
        bind:value={tags}
        placeholder="Italian, Pasta, Quick"
        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
      />
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Source URL (optional)</label>
      <input
        type="url"
        bind:value={sourceUrl}
        placeholder="https://..."
        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
      />
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Notes (optional)</label>
      <textarea
        bind:value={notes}
        rows="2"
        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
      ></textarea>
    </div>

    <div class="flex justify-end gap-3 pt-4 border-t">
      <button
        type="button"
        onclick={onCancel}
        class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
      >
        Cancel
      </button>
      <button
        type="submit"
        class="px-6 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
      >
        Save Recipe
      </button>
    </div>
  </form>
</div>
```

**File**: `src/lib/components/recipes/ImportRecipe.svelte`

**Change**: Create import recipe interface

```svelte
<script lang="ts">
  interface Props {
    onImport: (url: string) => void;
    onCancel: () => void;
  }

  let { onImport, onCancel }: Props = $props();
  let url = $state("");
  let isLoading = $state(false);

  function handleSubmit() {
    if (!url.trim()) return;
    isLoading = true;
    // Simulate import delay (actual parsing will be backend)
    setTimeout(() => {
      onImport(url);
      isLoading = false;
    }, 1500);
  }
</script>

<div class="max-w-xl mx-auto bg-white rounded-xl shadow-sm border border-gray-100 p-6">
  <h2 class="text-xl font-bold text-gray-800 mb-2">Import Recipe from URL</h2>
  <p class="text-gray-500 mb-6">
    Paste a link to a recipe from any website. We'll extract the ingredients, instructions, and more.
  </p>

  <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-4">
    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Recipe URL</label>
      <input
        type="url"
        bind:value={url}
        required
        placeholder="https://www.example.com/recipe/..."
        class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
      />
    </div>

    {#if isLoading}
      <div class="flex items-center justify-center py-8">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-emerald-600"></div>
        <span class="ml-3 text-gray-600">Importing recipe...</span>
      </div>
    {/if}

    <div class="flex justify-end gap-3 pt-4">
      <button
        type="button"
        onclick={onCancel}
        class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
      >
        Cancel
      </button>
      <button
        type="submit"
        disabled={isLoading}
        class="px-6 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors disabled:opacity-50"
      >
        Import Recipe
      </button>
    </div>
  </form>
</div>
```

**File**: `src/lib/components/Recipes.svelte`

**Change**: Compose recipes view

```svelte
<script lang="ts">
  import { recipeStore } from "$lib/stores";
  import type { Recipe } from "$lib/types";
  import RecipeCard from "./recipes/RecipeCard.svelte";
  import RecipeDetail from "./recipes/RecipeDetail.svelte";
  import RecipeForm from "./recipes/RecipeForm.svelte";
  import ImportRecipe from "./recipes/ImportRecipe.svelte";

  type View = "list" | "detail" | "create" | "import";

  let currentView = $state<View>("list");
  let selectedRecipe = $state<Recipe | null>(null);
  let searchQuery = $state("");

  let filteredRecipes = $derived(
    $recipeStore.filter(
      (r) =>
        r.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        r.tags.some((t) => t.toLowerCase().includes(searchQuery.toLowerCase()))
    )
  );

  function selectRecipe(recipe: Recipe) {
    selectedRecipe = recipe;
    currentView = "detail";
  }

  function handleSaveRecipe(data: Omit<Recipe, "id" | "createdAt">) {
    const newRecipe: Recipe = {
      ...data,
      id: crypto.randomUUID(),
      createdAt: new Date().toISOString().split("T")[0],
    };
    recipeStore.add(newRecipe);
    currentView = "list";
  }

  function handleImport(url: string) {
    // Mock import - just shows success
    alert(`Recipe imported from: ${url}\n\n(In production, this would parse the URL)`);
    currentView = "list";
  }
</script>

{#if currentView === "list"}
  <div class="max-w-6xl mx-auto">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold text-gray-800">Recipes</h1>
      <div class="flex gap-2">
        <button
          type="button"
          onclick={() => currentView = "import"}
          class="px-4 py-2 border border-emerald-600 text-emerald-600 rounded-lg hover:bg-emerald-50 transition-colors"
        >
          Import from URL
        </button>
        <button
          type="button"
          onclick={() => currentView = "create"}
          class="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
        >
          + Add Recipe
        </button>
      </div>
    </div>

    <div class="mb-6">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search recipes by name or tag..."
        class="w-full max-w-md px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
      />
    </div>

    <div class="grid grid-cols-3 gap-6">
      {#each filteredRecipes as recipe}
        <RecipeCard {recipe} onSelect={selectRecipe} />
      {/each}
    </div>

    {#if filteredRecipes.length === 0}
      <div class="text-center py-12 text-gray-500">
        No recipes found. Add your first recipe!
      </div>
    {/if}
  </div>
{:else if currentView === "detail" && selectedRecipe}
  <RecipeDetail
    recipe={selectedRecipe}
    onBack={() => { currentView = "list"; selectedRecipe = null; }}
  />
{:else if currentView === "create"}
  <RecipeForm onSave={handleSaveRecipe} onCancel={() => currentView = "list"} />
{:else if currentView === "import"}
  <ImportRecipe onImport={handleImport} onCancel={() => currentView = "list"} />
{/if}
```

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`

#### Integration Verification
- [ ] Recipe components render correctly
- [ ] Store updates reflect in UI

#### Manual Verification
- [ ] Recipe list displays all mock recipes
- [ ] Search filters recipes by name and tag
- [ ] Clicking recipe shows detail view
- [ ] Serving size adjustment updates ingredient quantities
- [ ] Create recipe form has all fields
- [ ] Import recipe shows loading state
- [ ] All hover/focus states work

**Checkpoint**: Pause for manual verification before proceeding to Phase 4.

---

## Phase 4: Meal Planning

### Goal
Build the meal plan calendar view with click-to-add workflow and serving size adjustment.

### Integration Points

**Consumes from Phase 3**: Recipe components, recipe store
**Produces for next phase**: Meal plan data for shopping list aggregation

**Wiring required**:
- [ ] Replace placeholder in `src/lib/components/MealPlan.svelte`
- [ ] Create sub-components in `src/lib/components/mealplan/`
- [ ] Create shared modal component in `src/lib/components/shared/`

### Changes

#### Shared Components

**File**: `src/lib/components/shared/Modal.svelte`

**Change**: Create reusable modal component

```svelte
<script lang="ts">
  import { onMount } from "svelte";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    title: string;
    children: any;
  }

  let { isOpen, onClose, title, children }: Props = $props();

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") onClose();
  }

  onMount(() => {
    document.addEventListener("keydown", handleKeydown);
    return () => document.removeEventListener("keydown", handleKeydown);
  });
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="fixed inset-0 bg-black/50"
      onclick={onClose}
      role="button"
      tabindex="-1"
      aria-label="Close modal"
    ></div>

    <div class="relative bg-white rounded-xl shadow-xl max-w-lg w-full mx-4 max-h-[90vh] overflow-auto">
      <div class="flex items-center justify-between px-6 py-4 border-b">
        <h2 class="text-lg font-semibold text-gray-800">{title}</h2>
        <button
          type="button"
          onclick={onClose}
          class="text-gray-400 hover:text-gray-600"
        >
          ✕
        </button>
      </div>
      <div class="p-6">
        {@render children()}
      </div>
    </div>
  </div>
{/if}
```

#### Meal Plan Components

**File**: `src/lib/components/mealplan/RecipePickerModal.svelte`

**Change**: Create recipe picker modal for adding meals

```svelte
<script lang="ts">
  import Modal from "../shared/Modal.svelte";
  import { recipeStore } from "$lib/stores";
  import type { Recipe, MealType } from "$lib/types";

  interface Props {
    isOpen: boolean;
    date: string;
    onClose: () => void;
    onSelect: (recipeId: string, mealType: MealType, servings: number) => void;
  }

  let { isOpen, date, onClose, onSelect }: Props = $props();

  let searchQuery = $state("");
  let selectedRecipe = $state<Recipe | null>(null);
  let mealType = $state<MealType>("dinner");
  let servings = $state(4);

  let filteredRecipes = $derived(
    $recipeStore.filter((r) =>
      r.name.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  function handleConfirm() {
    if (!selectedRecipe) return;
    onSelect(selectedRecipe.id, mealType, servings);
    resetForm();
    onClose();
  }

  function resetForm() {
    selectedRecipe = null;
    searchQuery = "";
    mealType = "dinner";
    servings = 4;
  }

  const mealTypes: { value: MealType; label: string }[] = [
    { value: "breakfast", label: "Breakfast" },
    { value: "lunch", label: "Lunch" },
    { value: "dinner", label: "Dinner" },
    { value: "snack", label: "Snack" },
  ];

  const formattedDate = $derived(
    new Date(date + "T00:00:00").toLocaleDateString("en-US", {
      weekday: "long",
      month: "short",
      day: "numeric",
    })
  );
</script>

<Modal {isOpen} {onClose} title="Add Meal for {formattedDate}">
  {#snippet children()}
    {#if !selectedRecipe}
      <div class="space-y-4">
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search recipes..."
          class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500"
        />

        <div class="max-h-64 overflow-y-auto space-y-2">
          {#each filteredRecipes as recipe}
            <button
              type="button"
              onclick={() => { selectedRecipe = recipe; servings = recipe.servings; }}
              class="w-full text-left px-4 py-3 rounded-lg hover:bg-gray-50 border border-gray-200"
            >
              <div class="font-medium text-gray-800">{recipe.name}</div>
              <div class="text-sm text-gray-500">
                {recipe.prepTime + recipe.cookTime} min · {recipe.servings} servings
              </div>
            </button>
          {/each}
        </div>
      </div>
    {:else}
      <div class="space-y-4">
        <div class="p-4 bg-emerald-50 rounded-lg">
          <div class="font-medium text-emerald-800">{selectedRecipe.name}</div>
          <button
            type="button"
            onclick={() => selectedRecipe = null}
            class="text-sm text-emerald-600 hover:text-emerald-700"
          >
            Change recipe
          </button>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Meal Type</label>
          <div class="grid grid-cols-4 gap-2">
            {#each mealTypes as mt}
              <button
                type="button"
                onclick={() => mealType = mt.value}
                class="px-3 py-2 text-sm rounded-lg border transition-colors
                  {mealType === mt.value
                    ? 'bg-emerald-600 text-white border-emerald-600'
                    : 'border-gray-300 hover:border-emerald-500'}"
              >
                {mt.label}
              </button>
            {/each}
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Servings</label>
          <div class="flex items-center gap-3">
            <button
              type="button"
              onclick={() => servings = Math.max(1, servings - 1)}
              class="w-10 h-10 rounded-lg bg-gray-100 hover:bg-gray-200 text-xl"
            >
              -
            </button>
            <span class="text-xl font-medium w-12 text-center">{servings}</span>
            <button
              type="button"
              onclick={() => servings++}
              class="w-10 h-10 rounded-lg bg-gray-100 hover:bg-gray-200 text-xl"
            >
              +
            </button>
          </div>
        </div>

        <button
          type="button"
          onclick={handleConfirm}
          class="w-full py-3 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
        >
          Add to Meal Plan
        </button>
      </div>
    {/if}
  {/snippet}
</Modal>
```

**File**: `src/lib/components/mealplan/MealPlanCalendar.svelte`

**Change**: Create meal plan calendar with click-to-add

```svelte
<script lang="ts">
  import { mealPlanStore, mealPlanByDate, recipeById } from "$lib/stores";
  import type { MealType } from "$lib/types";
  import RecipePickerModal from "./RecipePickerModal.svelte";

  interface Props {
    weekOffset?: number;
  }

  let { weekOffset = 0 }: Props = $props();

  let isModalOpen = $state(false);
  let selectedDate = $state("");

  const mealTypeOrder: MealType[] = ["breakfast", "lunch", "dinner", "snack"];
  const mealTypeLabels: Record<MealType, string> = {
    breakfast: "Breakfast",
    lunch: "Lunch",
    dinner: "Dinner",
    snack: "Snack",
  };
  const mealTypeColors: Record<MealType, string> = {
    breakfast: "bg-amber-100 text-amber-800",
    lunch: "bg-blue-100 text-blue-800",
    dinner: "bg-emerald-100 text-emerald-800",
    snack: "bg-purple-100 text-purple-800",
  };

  function getWeekDates(): { date: string; dayName: string; dayNum: number; month: string }[] {
    const today = new Date();
    const monday = new Date(today);
    monday.setDate(today.getDate() - today.getDay() + 1 + weekOffset * 7);

    return Array.from({ length: 7 }, (_, i) => {
      const date = new Date(monday);
      date.setDate(monday.getDate() + i);
      return {
        date: date.toISOString().split("T")[0],
        dayName: date.toLocaleDateString("en-US", { weekday: "short" }),
        dayNum: date.getDate(),
        month: date.toLocaleDateString("en-US", { month: "short" }),
      };
    });
  }

  let weekDates = $derived(getWeekDates());
  const today = new Date().toISOString().split("T")[0];

  function openAddModal(date: string) {
    selectedDate = date;
    isModalOpen = true;
  }

  function handleAddMeal(recipeId: string, mealType: MealType, servings: number) {
    mealPlanStore.addMeal(selectedDate, recipeId, mealType, servings);
  }

  function handleRemoveMeal(date: string, mealId: string) {
    mealPlanStore.removeMeal(date, mealId);
  }
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
  <div class="grid grid-cols-7 divide-x divide-gray-100">
    {#each weekDates as { date, dayName, dayNum, month }}
      {@const plan = $mealPlanByDate.get(date)}
      {@const isToday = date === today}

      <div class="min-h-[300px] flex flex-col">
        <div class="px-3 py-3 border-b border-gray-100 text-center {isToday ? 'bg-emerald-50' : ''}">
          <div class="text-xs text-gray-500 uppercase">{dayName}</div>
          <div class="text-xl font-semibold {isToday ? 'text-emerald-600' : 'text-gray-800'}">
            {dayNum}
          </div>
          <div class="text-xs text-gray-400">{month}</div>
        </div>

        <div class="flex-1 p-2 space-y-2">
          {#if plan}
            {#each mealTypeOrder as mealType}
              {#each plan.meals.filter(m => m.mealType === mealType) as meal}
                {@const recipe = $recipeById.get(meal.recipeId)}
                {#if recipe}
                  <div class="group relative p-2 rounded-lg {mealTypeColors[mealType]}">
                    <div class="text-[10px] uppercase font-medium opacity-70">
                      {mealTypeLabels[mealType]}
                    </div>
                    <div class="text-sm font-medium truncate">{recipe.name}</div>
                    <div class="text-xs opacity-70">{meal.servings} servings</div>
                    <button
                      type="button"
                      onclick={() => handleRemoveMeal(date, meal.id)}
                      class="absolute top-1 right-1 w-5 h-5 rounded-full bg-white/50 opacity-0 group-hover:opacity-100 transition-opacity text-xs"
                    >
                      ✕
                    </button>
                  </div>
                {/if}
              {/each}
            {/each}
          {/if}
        </div>

        <div class="p-2 border-t border-gray-100">
          <button
            type="button"
            onclick={() => openAddModal(date)}
            class="w-full py-2 text-sm text-gray-400 hover:text-emerald-600 hover:bg-emerald-50 rounded-lg transition-colors"
          >
            + Add meal
          </button>
        </div>
      </div>
    {/each}
  </div>
</div>

<RecipePickerModal
  isOpen={isModalOpen}
  date={selectedDate}
  onClose={() => isModalOpen = false}
  onSelect={handleAddMeal}
/>
```

**File**: `src/lib/components/MealPlan.svelte`

**Change**: Compose meal plan view

```svelte
<script lang="ts">
  import MealPlanCalendar from "./mealplan/MealPlanCalendar.svelte";

  let weekOffset = $state(0);

  function getWeekLabel(offset: number): string {
    const today = new Date();
    const monday = new Date(today);
    monday.setDate(today.getDate() - today.getDay() + 1 + offset * 7);
    const sunday = new Date(monday);
    sunday.setDate(monday.getDate() + 6);

    const fmt = (d: Date) => d.toLocaleDateString("en-US", { month: "short", day: "numeric" });
    return `${fmt(monday)} - ${fmt(sunday)}`;
  }

  let weekLabel = $derived(getWeekLabel(weekOffset));
</script>

<div class="max-w-6xl mx-auto">
  <div class="flex justify-between items-center mb-6">
    <h1 class="text-2xl font-bold text-gray-800">Meal Plan</h1>

    <div class="flex items-center gap-4">
      <button
        type="button"
        onclick={() => weekOffset--}
        class="p-2 hover:bg-gray-100 rounded-lg transition-colors"
      >
        ←
      </button>
      <span class="text-gray-600 min-w-[160px] text-center">{weekLabel}</span>
      <button
        type="button"
        onclick={() => weekOffset++}
        class="p-2 hover:bg-gray-100 rounded-lg transition-colors"
      >
        →
      </button>
      {#if weekOffset !== 0}
        <button
          type="button"
          onclick={() => weekOffset = 0}
          class="text-sm text-emerald-600 hover:text-emerald-700"
        >
          Today
        </button>
      {/if}
    </div>
  </div>

  <MealPlanCalendar {weekOffset} />

  <div class="mt-4 text-sm text-gray-500 text-center">
    Click "+ Add meal" on any day to add a recipe to your meal plan
  </div>
</div>
```

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`

#### Integration Verification
- [ ] Modal component reusable
- [ ] Meal plan store updates trigger UI refresh

#### Manual Verification
- [ ] Calendar displays 7 days with correct dates
- [ ] Can navigate between weeks with arrows
- [ ] "Today" button returns to current week
- [ ] Click "+ Add meal" opens recipe picker modal
- [ ] Can search recipes in picker
- [ ] Can select meal type (breakfast/lunch/dinner/snack)
- [ ] Can adjust servings before adding
- [ ] Added meals appear on calendar with correct colors
- [ ] Hover reveals remove button on meals
- [ ] Can remove meals from calendar
- [ ] Modal closes on Escape key and backdrop click

**Checkpoint**: Pause for manual verification before proceeding to Phase 5.

---

## Phase 5: Shopping List

### Goal
Build the shopping list view with aggregated ingredients, manual additions, and quick lists.

### Integration Points

**Consumes from Phase 4**: Aggregated shopping list derived from meal plan
**Produces for next phase**: Complete shopping list functionality

**Wiring required**:
- [ ] Replace placeholder in `src/lib/components/ShoppingList.svelte`
- [ ] Create sub-components in `src/lib/components/shopping/`

### Changes

#### Shopping List Components

**File**: `src/lib/components/shopping/ShoppingItem.svelte`

**Change**: Create shopping item row component

```svelte
<script lang="ts">
  import type { ShoppingItem } from "$lib/types";
  import { recipeById } from "$lib/stores";

  interface Props {
    item: ShoppingItem;
    onToggle: () => void;
    onRemove?: () => void;
    onUpdateQuantity?: (quantity: number) => void;
  }

  let { item, onToggle, onRemove, onUpdateQuantity }: Props = $props();

  let isEditing = $state(false);
  let editQuantity = $state(item.quantity);

  function handleSaveQuantity() {
    onUpdateQuantity?.(editQuantity);
    isEditing = false;
  }

  const sourceNames = $derived(
    item.sourceRecipeIds
      .map((id) => $recipeById.get(id)?.name)
      .filter(Boolean)
      .join(", ")
  );
</script>

<div
  class="flex items-center gap-4 px-4 py-3 hover:bg-gray-50 transition-colors
    {item.isOnHand ? 'opacity-50' : ''}"
>
  <button
    type="button"
    onclick={onToggle}
    class="flex-shrink-0 w-6 h-6 rounded-full border-2 flex items-center justify-center transition-colors
      {item.isOnHand
        ? 'bg-emerald-500 border-emerald-500 text-white'
        : 'border-gray-300 hover:border-emerald-500'}"
  >
    {#if item.isOnHand}✓{/if}
  </button>

  <div class="flex-1 min-w-0">
    <div class="font-medium text-gray-800 {item.isOnHand ? 'line-through' : ''}">
      {item.name}
    </div>
    {#if sourceNames && !item.isManual}
      <div class="text-xs text-gray-400 truncate">
        From: {sourceNames}
      </div>
    {/if}
  </div>

  <div class="flex items-center gap-2">
    {#if isEditing && onUpdateQuantity}
      <input
        type="number"
        bind:value={editQuantity}
        min="0"
        step="0.25"
        class="w-16 px-2 py-1 text-sm border border-gray-300 rounded"
      />
      <button
        type="button"
        onclick={handleSaveQuantity}
        class="text-emerald-600 hover:text-emerald-700"
      >
        ✓
      </button>
    {:else}
      <button
        type="button"
        onclick={() => { isEditing = true; editQuantity = item.quantity; }}
        class="text-sm text-gray-600 hover:text-gray-800"
      >
        {item.quantity} {item.unit}
      </button>
    {/if}
  </div>

  {#if onRemove && item.isManual}
    <button
      type="button"
      onclick={onRemove}
      class="text-gray-400 hover:text-red-500"
    >
      ✕
    </button>
  {/if}
</div>
```

**File**: `src/lib/components/shopping/AddItemForm.svelte`

**Change**: Create add item form

```svelte
<script lang="ts">
  interface Props {
    onAdd: (name: string, quantity: number, unit: string, category: string) => void;
  }

  let { onAdd }: Props = $props();

  let name = $state("");
  let quantity = $state(1);
  let unit = $state("");
  let category = $state("Grocery");
  let isExpanded = $state(false);

  const categories = [
    "Produce",
    "Dairy",
    "Meat",
    "Bakery",
    "Frozen",
    "Pantry",
    "Spices",
    "Beverages",
    "Household",
    "Other",
  ];

  function handleSubmit() {
    if (!name.trim()) return;
    onAdd(name, quantity, unit, category);
    name = "";
    quantity = 1;
    unit = "";
    isExpanded = false;
  }
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-100 p-4">
  {#if !isExpanded}
    <button
      type="button"
      onclick={() => isExpanded = true}
      class="w-full text-left text-emerald-600 hover:text-emerald-700"
    >
      + Add item manually
    </button>
  {:else}
    <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-3">
      <div class="flex gap-2">
        <input
          type="number"
          bind:value={quantity}
          min="0"
          step="0.25"
          class="w-20 px-3 py-2 border border-gray-300 rounded-lg"
          placeholder="Qty"
        />
        <input
          type="text"
          bind:value={unit}
          class="w-24 px-3 py-2 border border-gray-300 rounded-lg"
          placeholder="Unit"
        />
        <input
          type="text"
          bind:value={name}
          class="flex-1 px-3 py-2 border border-gray-300 rounded-lg"
          placeholder="Item name"
        />
      </div>

      <div class="flex gap-2">
        <select
          bind:value={category}
          class="flex-1 px-3 py-2 border border-gray-300 rounded-lg"
        >
          {#each categories as cat}
            <option value={cat}>{cat}</option>
          {/each}
        </select>

        <button
          type="button"
          onclick={() => isExpanded = false}
          class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg"
        >
          Cancel
        </button>
        <button
          type="submit"
          class="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700"
        >
          Add
        </button>
      </div>
    </form>
  {/if}
</div>
```

**File**: `src/lib/components/shopping/QuickLists.svelte`

**Change**: Create quick lists accordion

```svelte
<script lang="ts">
  import { quickListsStore } from "$lib/stores";
  import type { QuickListItem } from "$lib/types";

  interface Props {
    onAddItems: (items: QuickListItem[]) => void;
  }

  let { onAddItems }: Props = $props();

  let expandedList = $state<string | null>(null);

  function toggleList(id: string) {
    expandedList = expandedList === id ? null : id;
  }

  function addSingleItem(item: QuickListItem) {
    onAddItems([item]);
  }

  function addAllItems(listId: string) {
    const list = $quickListsStore.find((l) => l.id === listId);
    if (list) {
      onAddItems(list.items);
    }
  }
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
  <div class="px-4 py-3 border-b border-gray-100">
    <h3 class="font-semibold text-gray-800">Quick Lists</h3>
  </div>

  <div class="divide-y divide-gray-100">
    {#each $quickListsStore as list}
      <div>
        <button
          type="button"
          onclick={() => toggleList(list.id)}
          class="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-50 transition-colors"
        >
          <span class="font-medium text-gray-700">{list.name}</span>
          <span class="text-gray-400 text-sm">
            {list.items.length} items
            <span class="ml-2">{expandedList === list.id ? '▲' : '▼'}</span>
          </span>
        </button>

        {#if expandedList === list.id}
          <div class="px-4 pb-3 bg-gray-50">
            <div class="space-y-1 mb-3">
              {#each list.items as item}
                <div class="flex items-center justify-between py-1 text-sm">
                  <span class="text-gray-600">
                    {item.quantity} {item.unit} {item.name}
                  </span>
                  <button
                    type="button"
                    onclick={() => addSingleItem(item)}
                    class="text-emerald-600 hover:text-emerald-700"
                  >
                    + Add
                  </button>
                </div>
              {/each}
            </div>
            <button
              type="button"
              onclick={() => addAllItems(list.id)}
              class="w-full py-2 text-sm bg-emerald-600 text-white rounded-lg hover:bg-emerald-700"
            >
              Add all items
            </button>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>
```

**File**: `src/lib/components/ShoppingList.svelte`

**Change**: Compose shopping list view

```svelte
<script lang="ts">
  import { aggregatedShoppingList, manualItemsStore } from "$lib/stores";
  import type { QuickListItem } from "$lib/types";
  import ShoppingItem from "./shopping/ShoppingItem.svelte";
  import AddItemForm from "./shopping/AddItemForm.svelte";
  import QuickLists from "./shopping/QuickLists.svelte";

  // Track on-hand state for aggregated items (not in store)
  let onHandIds = $state(new Set<string>());

  function toggleOnHand(itemId: string, isManual: boolean) {
    if (isManual) {
      manualItemsStore.toggleOnHand(itemId);
    } else {
      if (onHandIds.has(itemId)) {
        onHandIds.delete(itemId);
      } else {
        onHandIds.add(itemId);
      }
      onHandIds = new Set(onHandIds); // Trigger reactivity
    }
  }

  function handleAddItem(name: string, quantity: number, unit: string, category: string) {
    manualItemsStore.add({ name, quantity, unit, category, isOnHand: false });
  }

  function handleRemoveItem(id: string) {
    manualItemsStore.remove(id);
  }

  function handleAddFromQuickList(items: QuickListItem[]) {
    items.forEach((item) => {
      manualItemsStore.add({
        name: item.name,
        quantity: item.quantity,
        unit: item.unit,
        category: item.category,
        isOnHand: false,
      });
    });
  }

  // Merge on-hand state with aggregated list
  let displayItems = $derived(
    $aggregatedShoppingList.map((item) => ({
      ...item,
      isOnHand: item.isManual ? item.isOnHand : onHandIds.has(item.id),
    }))
  );

  let itemsToBuy = $derived(displayItems.filter((i) => !i.isOnHand));
  let itemsOnHand = $derived(displayItems.filter((i) => i.isOnHand));
</script>

<div class="max-w-4xl mx-auto">
  <div class="flex justify-between items-center mb-6">
    <h1 class="text-2xl font-bold text-gray-800">Shopping List</h1>
    <div class="text-sm text-gray-500">
      {itemsToBuy.length} items to buy · {itemsOnHand.length} on hand
    </div>
  </div>

  <div class="grid grid-cols-3 gap-6">
    <div class="col-span-2 space-y-4">
      <AddItemForm onAdd={handleAddItem} />

      <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
        <div class="px-4 py-3 border-b border-gray-100">
          <h3 class="font-semibold text-gray-800">To Buy ({itemsToBuy.length})</h3>
        </div>

        {#if itemsToBuy.length > 0}
          <div class="divide-y divide-gray-100">
            {#each itemsToBuy as item}
              <ShoppingItem
                {item}
                onToggle={() => toggleOnHand(item.id, item.isManual)}
                onRemove={item.isManual ? () => handleRemoveItem(item.id) : undefined}
              />
            {/each}
          </div>
        {:else}
          <div class="px-4 py-8 text-center text-gray-500">
            No items to buy. Add meals to your plan or add items manually.
          </div>
        {/if}
      </div>

      {#if itemsOnHand.length > 0}
        <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
          <div class="px-4 py-3 border-b border-gray-100">
            <h3 class="font-semibold text-gray-500">On Hand ({itemsOnHand.length})</h3>
          </div>
          <div class="divide-y divide-gray-100">
            {#each itemsOnHand as item}
              <ShoppingItem
                {item}
                onToggle={() => toggleOnHand(item.id, item.isManual)}
                onRemove={item.isManual ? () => handleRemoveItem(item.id) : undefined}
              />
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <div>
      <QuickLists onAddItems={handleAddFromQuickList} />
    </div>
  </div>
</div>
```

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`

#### Integration Verification
- [ ] Aggregated list reflects meal plan changes
- [ ] Manual items persist in store

#### Manual Verification
- [ ] Shopping list shows aggregated ingredients from meal plan
- [ ] Can add items manually via form
- [ ] Can mark items as "on hand" (checkmark animation)
- [ ] Checked items move to "On Hand" section
- [ ] Can remove manually added items
- [ ] Quick lists expand/collapse on click
- [ ] Can add single item from quick list
- [ ] Can add all items from quick list
- [ ] Item counts update correctly in header

**Checkpoint**: Pause for manual verification before proceeding to Phase 6.

---

## Phase 6: Polish

### Goal
Add final visual polish, hover states, transitions, and ensure consistent styling across all views.

### Integration Points

**Consumes from Phase 5**: All components
**Produces**: Production-ready UI

**Wiring required**:
- [ ] Review all components for consistent styling
- [ ] Add missing transitions and hover states
- [ ] Ensure accessibility basics (focus states, ARIA labels)

### Changes

#### Global Styles

**File**: `src/app.css`

**Change**: Add custom CSS for transitions and focus states

```css
@import "tailwindcss";

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* Focus visible styles for accessibility */
:focus-visible {
  outline: 2px solid #10b981;
  outline-offset: 2px;
}

/* Smooth transitions for interactive elements */
button,
a,
input,
select,
textarea {
  transition: all 0.15s ease-in-out;
}

/* Custom scrollbar */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a1a1a1;
}
```

#### Component Polish

Review and update each component to ensure:
- Consistent border-radius (rounded-lg for small, rounded-xl for cards)
- Consistent shadow usage (shadow-sm for cards)
- Hover states on all interactive elements
- Focus-visible rings on buttons and inputs
- Consistent color palette (emerald primary, gray neutrals)
- Transition classes on state changes

#### Mock Data Enhancement

**File**: `src/lib/stores/recipes.ts`

**Change**: Add more realistic mock recipes (5-6 total with variety)

Include recipes like:
- Spaghetti Carbonara (Italian, Pasta)
- Chicken Stir Fry (Asian, Quick)
- Caesar Salad (Salad, Healthy)
- Beef Tacos (Mexican, Family)
- Overnight Oats (Breakfast, Healthy)
- Grilled Salmon (Seafood, Healthy)

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`
- [ ] Build succeeds: `pnpm build`

#### Integration Verification
- [ ] All components render without errors
- [ ] Navigation works across all tabs

#### Manual Verification
- [ ] All buttons have visible hover states
- [ ] All inputs have focus rings
- [ ] Cards have consistent styling
- [ ] Transitions feel smooth (not jarring)
- [ ] Color palette is consistent
- [ ] Mock data is realistic and varied
- [ ] No placeholder text remaining
- [ ] App feels polished and production-ready

---

## Testing Strategy

### Unit Tests
- Recipe store CRUD operations
- Meal plan store operations
- Shopping list aggregation logic

### Integration Tests
- Navigation between tabs
- Adding meal to plan updates shopping list

### Manual Testing Checklist
1. [ ] Navigate through all tabs multiple times
2. [ ] Create a recipe, verify it appears in list
3. [ ] Add multiple meals to plan across different days
4. [ ] Verify shopping list aggregates ingredients correctly
5. [ ] Mark items as on-hand, verify they move sections
6. [ ] Add items from quick lists
7. [ ] Adjust serving sizes in recipe detail
8. [ ] Navigate weeks in meal plan calendar
9. [ ] Search/filter recipes
10. [ ] Close modals with Escape key and backdrop click

## Rollback Plan

Git revert to commit before Phase 1:
```
git revert --no-commit HEAD~N..HEAD
```

No data migration required — this is a UI-only change with mock data.

## Migration Notes

- **Data migration**: None required
- **Feature flags**: None
- **Backwards compatibility**: Not applicable (new feature)

## References

- Ticket: `ai_docs/prompts/2025-12-13-meal-planning-ui.md`
- Similar implementation: `src/App.svelte:1-39` (Svelte 5 patterns)
- Store pattern: `src/lib/stores/app.ts:1-20`
