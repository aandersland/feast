# Shopping List Redesign Implementation Plan

## Overview

Redesign the shopping list experience by merging it into the Meal Plan page, enabling multi-list support per week (weekly, mid-week, custom), and improving information organization through a category-based card layout with a collapsible Quick Lists sidebar.

## Current State

The Shopping List is a standalone page (`src/lib/components/ShoppingList.svelte`) that:
- Aggregates ingredients from ALL meal plans globally (not week-specific)
- Displays items in a single scrolling vertical list
- Has Quick Lists in a right sidebar taking horizontal space
- Has no concept of multiple lists per week

**Key Discoveries**:
- Navigation defined in `src/lib/stores/navigation.ts:3` with TabId union type
- Tab array in `src/lib/components/TabNavigation.svelte:4-10`
- Week offset state in `src/lib/components/MealPlan.svelte:4` controls week navigation
- Shopping aggregation in `src/lib/stores/shoppingList.ts:102-143` is not week-aware
- Existing `ShoppingItem` component at `src/lib/components/shopping/ShoppingItem.svelte`

## Desired End State

The Meal Plan page becomes a weekly hub with:
- Meal calendar at top (unchanged)
- Shopping section below spanning full width
- Collapsible left sidebar for Quick Lists (collapsed by default)
- Tabbed interface for multiple lists: Weekly | Mid-week | + New
- Category cards in multi-column grid (Produce, Dairy, Meat, etc.)
- Items show name, quantity (imperial + metric), source recipe
- Click icon to move items between lists
- Shopping List tab removed from navigation

## What We're NOT Doing

- Data migration (mock data only)
- Backend/database changes
- User-defined custom categories
- Drag-and-drop between lists
- Consistency updates to other pages

## Integration Map

| Type | Location | Notes |
|------|----------|-------|
| Entry point | `src/App.svelte:27-29` | Remove shopping tab conditional |
| Registration | `src/lib/components/TabNavigation.svelte:4-10` | Remove "shopping" from tabs array |
| Type updates | `src/lib/stores/navigation.ts:3` | Remove "shopping" from TabId |
| Type additions | `src/lib/types/shoppingList.ts` | Add WeeklyShoppingLists types |
| Store changes | `src/lib/stores/shoppingList.ts` | Add weeklyShoppingListsStore |
| Store exports | `src/lib/stores/index.ts` | Export new store |
| Type exports | `src/lib/types/index.ts` | Export new types |
| Consumer | `src/lib/components/MealPlan.svelte` | Add ShoppingSection component |

## Implementation Approach

UI-first approach since this is a redesign with mock data. We build types and stores first to establish the data model, then create new components, integrate into Meal Plan, and finally clean up navigation.

---

## Phase 1: Types & Store Foundation

### Goal
Establish the data model for week-scoped multi-list shopping with category support.

### Integration Points

**Depends on**: Existing types and stores
**Produces for next phase**: `weeklyShoppingListsStore`, new types for components to consume

**Wiring required**:
- [ ] Export new types from `src/lib/types/index.ts`
- [ ] Export new store from `src/lib/stores/index.ts`

### Changes

#### Types

**File**: `src/lib/types/shoppingList.ts`

**Change**: Add types for multi-list support, shopping list types, and categories

```typescript
// Add after existing types

export type ShoppingListType = 'weekly' | 'midweek' | 'custom';

export interface ShoppingList {
  id: string;
  name: string;
  type: ShoppingListType;
  items: ShoppingItem[];
}

export interface WeeklyShoppingLists {
  weekStart: string; // ISO date of Monday
  lists: ShoppingList[];
}

export const GROCERY_CATEGORIES = [
  'Produce',
  'Meat & Seafood',
  'Dairy & Eggs',
  'Bakery',
  'Pantry',
  'Frozen',
  'Beverages',
  'Snacks',
  'Other',
] as const;

export type GroceryCategory = typeof GROCERY_CATEGORIES[number];

// Unit conversion helpers for imperial/metric display
export interface UnitDisplay {
  imperial: string;
  metric: string;
}
```

**File**: `src/lib/types/index.ts`

**Change**: Export new types

```typescript
export type {
  ShoppingItem,
  QuickList,
  QuickListItem,
  ShoppingListType,
  ShoppingList,
  WeeklyShoppingLists,
  GroceryCategory,
  UnitDisplay,
} from "./shoppingList";
export { GROCERY_CATEGORIES } from "./shoppingList";
```

#### Store

**File**: `src/lib/stores/shoppingList.ts`

**Change**: Add week-scoped multi-list store with mock data

```typescript
// Add imports
import type { ShoppingList, WeeklyShoppingLists, ShoppingListType } from "$lib/types";

// Add helper function after existing helpers
function getWeekStart(weekOffset: number = 0): string {
  const today = new Date();
  const monday = new Date(today);
  monday.setDate(today.getDate() - today.getDay() + 1 + weekOffset * 7);
  return monday.toISOString().split("T")[0];
}

// Add mock data for weekly shopping lists
const currentWeekStart = getWeekStart(0);

const mockWeeklyShoppingLists: WeeklyShoppingLists[] = [
  {
    weekStart: currentWeekStart,
    lists: [
      {
        id: "weekly-1",
        name: "Weekly",
        type: "weekly",
        items: [], // Will be populated from meal plan aggregation
      },
      {
        id: "midweek-1",
        name: "Mid-week",
        type: "midweek",
        items: [],
      },
    ],
  },
];

// Add new store
function createWeeklyShoppingListsStore() {
  const { subscribe, update } = writable<WeeklyShoppingLists[]>(mockWeeklyShoppingLists);

  return {
    subscribe,

    // Get or create lists for a specific week
    getOrCreateWeek: (weekStart: string) =>
      update((weeks) => {
        const existing = weeks.find((w) => w.weekStart === weekStart);
        if (existing) return weeks;

        return [
          ...weeks,
          {
            weekStart,
            lists: [
              { id: crypto.randomUUID(), name: "Weekly", type: "weekly" as ShoppingListType, items: [] },
              { id: crypto.randomUUID(), name: "Mid-week", type: "midweek" as ShoppingListType, items: [] },
            ],
          },
        ];
      }),

    // Add a new custom list to a week
    addList: (weekStart: string, name: string) =>
      update((weeks) =>
        weeks.map((w) =>
          w.weekStart === weekStart
            ? {
                ...w,
                lists: [
                  ...w.lists,
                  { id: crypto.randomUUID(), name, type: "custom" as ShoppingListType, items: [] },
                ],
              }
            : w
        )
      ),

    // Remove a custom list
    removeList: (weekStart: string, listId: string) =>
      update((weeks) =>
        weeks.map((w) =>
          w.weekStart === weekStart
            ? { ...w, lists: w.lists.filter((l) => l.id !== listId || l.type !== "custom") }
            : w
        )
      ),

    // Add item to a specific list
    addItem: (weekStart: string, listId: string, item: Omit<ShoppingItem, "id" | "isManual" | "sourceRecipeIds">) =>
      update((weeks) =>
        weeks.map((w) =>
          w.weekStart === weekStart
            ? {
                ...w,
                lists: w.lists.map((l) =>
                  l.id === listId
                    ? {
                        ...l,
                        items: [
                          ...l.items,
                          { ...item, id: crypto.randomUUID(), isManual: true, sourceRecipeIds: [] },
                        ],
                      }
                    : l
                ),
              }
            : w
        )
      ),

    // Move item between lists
    moveItem: (weekStart: string, fromListId: string, toListId: string, itemId: string) =>
      update((weeks) =>
        weeks.map((w) => {
          if (w.weekStart !== weekStart) return w;

          const fromList = w.lists.find((l) => l.id === fromListId);
          const item = fromList?.items.find((i) => i.id === itemId);
          if (!item) return w;

          return {
            ...w,
            lists: w.lists.map((l) => {
              if (l.id === fromListId) {
                return { ...l, items: l.items.filter((i) => i.id !== itemId) };
              }
              if (l.id === toListId) {
                return { ...l, items: [...l.items, item] };
              }
              return l;
            }),
          };
        })
      ),

    // Toggle item on-hand status
    toggleItemOnHand: (weekStart: string, listId: string, itemId: string) =>
      update((weeks) =>
        weeks.map((w) =>
          w.weekStart === weekStart
            ? {
                ...w,
                lists: w.lists.map((l) =>
                  l.id === listId
                    ? {
                        ...l,
                        items: l.items.map((i) =>
                          i.id === itemId ? { ...i, isOnHand: !i.isOnHand } : i
                        ),
                      }
                    : l
                ),
              }
            : w
        )
      ),

    // Remove item from list
    removeItem: (weekStart: string, listId: string, itemId: string) =>
      update((weeks) =>
        weeks.map((w) =>
          w.weekStart === weekStart
            ? {
                ...w,
                lists: w.lists.map((l) =>
                  l.id === listId
                    ? { ...l, items: l.items.filter((i) => i.id !== itemId) }
                    : l
                ),
              }
            : w
        )
      ),
  };
}

export const weeklyShoppingListsStore = createWeeklyShoppingListsStore();

// Derived store for a specific week's aggregated items (from meal plan)
export function createWeekAggregatedList(weekOffset: number) {
  return derived(
    [mealPlanStore, recipeById],
    ([$mealPlans, $recipeMap]) => {
      const weekStart = getWeekStart(weekOffset);
      const weekEnd = new Date(weekStart);
      weekEnd.setDate(weekEnd.getDate() + 6);
      const weekEndStr = weekEnd.toISOString().split("T")[0];

      const aggregated = new Map<string, ShoppingItem>();

      // Filter meal plans for this week only
      $mealPlans
        .filter((plan) => plan.date >= weekStart && plan.date <= weekEndStr)
        .forEach((plan) => {
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
                  category: ing.category || "Other",
                  isOnHand: false,
                  isManual: false,
                  sourceRecipeIds: [recipe.id],
                });
              }
            });
          });
        });

      return Array.from(aggregated.values());
    }
  );
}

// Helper to get week start date
export { getWeekStart };
```

**File**: `src/lib/stores/index.ts`

**Change**: Export new store and helper

```typescript
export {
  manualItemsStore,
  quickListsStore,
  aggregatedShoppingList,
  weeklyShoppingListsStore,
  createWeekAggregatedList,
  getWeekStart,
} from "./shoppingList";
```

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`

#### Integration Verification
- [ ] New types importable from `$lib/types`
- [ ] New store importable from `$lib/stores`
- [ ] `weeklyShoppingListsStore` methods work (subscribe, addItem, moveItem, etc.)

#### Manual Verification
- [ ] App still loads without errors
- [ ] Existing shopping list functionality unchanged

**Checkpoint**: Pause for manual verification before proceeding to Phase 2.

---

## Phase 2: Shopping Section Components

### Goal
Build the new shopping UI components: category cards, list tabs, and enhanced item display with move functionality.

### Integration Points

**Consumes from Phase 1**: `weeklyShoppingListsStore`, `ShoppingList`, `GroceryCategory`, `GROCERY_CATEGORIES`
**Produces for next phase**: `ShoppingSection.svelte`, `CategoryCard.svelte`, `ShoppingListTabs.svelte`

**Wiring required**:
- [ ] Create component directory `src/lib/components/shopping/`
- [ ] Components import from `$lib/stores` and `$lib/types`

### Changes

#### Category Card Component

**File**: `src/lib/components/shopping/CategoryCard.svelte` (new file)

**Change**: Create category card displaying items grouped by category

```svelte
<script lang="ts">
  import type { ShoppingItem, ShoppingList } from "$lib/types";
  import { recipeById } from "$lib/stores";
  import ShoppingItemRow from "./ShoppingItemRow.svelte";

  interface Props {
    category: string;
    items: ShoppingItem[];
    availableLists: ShoppingList[];
    currentListId: string;
    onToggleOnHand: (itemId: string) => void;
    onMoveItem: (itemId: string, toListId: string) => void;
    onRemoveItem: (itemId: string) => void;
  }

  let {
    category,
    items,
    availableLists,
    currentListId,
    onToggleOnHand,
    onMoveItem,
    onRemoveItem,
  }: Props = $props();

  let isCollapsed = $state(false);

  const categoryColors: Record<string, string> = {
    "Produce": "bg-green-50 border-green-200",
    "Meat & Seafood": "bg-red-50 border-red-200",
    "Dairy & Eggs": "bg-yellow-50 border-yellow-200",
    "Bakery": "bg-amber-50 border-amber-200",
    "Pantry": "bg-orange-50 border-orange-200",
    "Frozen": "bg-blue-50 border-blue-200",
    "Beverages": "bg-cyan-50 border-cyan-200",
    "Snacks": "bg-purple-50 border-purple-200",
    "Other": "bg-gray-50 border-gray-200",
  };

  let colorClass = $derived(categoryColors[category] || categoryColors["Other"]);
  let toBuyCount = $derived(items.filter((i) => !i.isOnHand).length);
</script>

<div class="rounded-xl border-2 {colorClass} overflow-hidden">
  <button
    type="button"
    onclick={() => isCollapsed = !isCollapsed}
    class="w-full px-4 py-3 flex items-center justify-between hover:bg-white/50 transition-colors"
  >
    <div class="flex items-center gap-2">
      <h3 class="font-semibold text-gray-800">{category}</h3>
      <span class="text-sm text-gray-500">({toBuyCount} to buy)</span>
    </div>
    <span class="text-gray-400">{isCollapsed ? '▶' : '▼'}</span>
  </button>

  {#if !isCollapsed}
    <div class="divide-y divide-gray-100 bg-white/50">
      {#each items as item (item.id)}
        <ShoppingItemRow
          {item}
          {availableLists}
          {currentListId}
          onToggle={() => onToggleOnHand(item.id)}
          onMove={(toListId) => onMoveItem(item.id, toListId)}
          onRemove={item.isManual ? () => onRemoveItem(item.id) : undefined}
        />
      {/each}
    </div>
  {/if}
</div>
```

#### Shopping Item Row Component

**File**: `src/lib/components/shopping/ShoppingItemRow.svelte` (new file)

**Change**: Create item row with imperial/metric display and move functionality

```svelte
<script lang="ts">
  import type { ShoppingItem, ShoppingList } from "$lib/types";
  import { recipeById } from "$lib/stores";

  interface Props {
    item: ShoppingItem;
    availableLists: ShoppingList[];
    currentListId: string;
    onToggle: () => void;
    onMove: (toListId: string) => void;
    onRemove?: () => void;
  }

  let { item, availableLists, currentListId, onToggle, onMove, onRemove }: Props = $props();

  let showMoveMenu = $state(false);

  // Simple unit conversion for display (imperial + metric)
  function formatQuantity(quantity: number, unit: string): string {
    const q = Math.round(quantity * 100) / 100;

    // Common conversions
    const conversions: Record<string, (n: number) => string> = {
      "g": (n) => `${n}g / ${(n * 0.035).toFixed(1)}oz`,
      "kg": (n) => `${n}kg / ${(n * 2.2).toFixed(1)}lb`,
      "ml": (n) => `${n}ml / ${(n * 0.034).toFixed(1)}fl oz`,
      "l": (n) => `${n}L / ${(n * 0.26).toFixed(1)}gal`,
      "oz": (n) => `${n}oz / ${(n * 28.35).toFixed(0)}g`,
      "lb": (n) => `${n}lb / ${(n * 0.45).toFixed(1)}kg`,
      "cup": (n) => `${n} cup / ${(n * 237).toFixed(0)}ml`,
      "tbsp": (n) => `${n} tbsp / ${(n * 15).toFixed(0)}ml`,
      "tsp": (n) => `${n} tsp / ${(n * 5).toFixed(0)}ml`,
    };

    const converter = conversions[unit.toLowerCase()];
    if (converter) {
      return converter(q);
    }
    return `${q} ${unit}`;
  }

  const sourceNames = $derived(
    item.sourceRecipeIds
      .map((id) => $recipeById.get(id)?.name)
      .filter(Boolean)
      .join(", ")
  );

  const otherLists = $derived(availableLists.filter((l) => l.id !== currentListId));
</script>

<div
  class="flex items-center gap-3 px-4 py-3 hover:bg-white transition-colors
    {item.isOnHand ? 'opacity-50' : ''}"
>
  <!-- Checkbox -->
  <button
    type="button"
    onclick={onToggle}
    class="flex-shrink-0 w-5 h-5 rounded-full border-2 flex items-center justify-center transition-colors
      {item.isOnHand
        ? 'bg-emerald-500 border-emerald-500 text-white'
        : 'border-gray-300 hover:border-emerald-500'}"
  >
    {#if item.isOnHand}<span class="text-xs">✓</span>{/if}
  </button>

  <!-- Item info -->
  <div class="flex-1 min-w-0">
    <div class="font-medium text-gray-800 {item.isOnHand ? 'line-through' : ''}">
      {item.name}
    </div>
    {#if sourceNames && !item.isManual}
      <div class="text-xs text-gray-400 truncate">From: {sourceNames}</div>
    {/if}
  </div>

  <!-- Quantity with imperial/metric -->
  <div class="text-sm text-gray-600 whitespace-nowrap">
    {formatQuantity(item.quantity, item.unit)}
  </div>

  <!-- Move to list buttons -->
  <div class="flex items-center gap-1">
    {#each otherLists as list (list.id)}
      <button
        type="button"
        onclick={() => onMove(list.id)}
        class="p-1.5 text-xs rounded hover:bg-gray-100 transition-colors"
        title="Move to {list.name}"
      >
        → {list.name.substring(0, 1)}
      </button>
    {/each}
  </div>

  <!-- Remove button -->
  {#if onRemove}
    <button
      type="button"
      onclick={onRemove}
      class="p-1 text-gray-400 hover:text-red-500 transition-colors"
      title="Remove item"
    >
      ✕
    </button>
  {/if}
</div>
```

#### Shopping List Tabs Component

**File**: `src/lib/components/shopping/ShoppingListTabs.svelte` (new file)

**Change**: Create tabbed interface for switching between lists

```svelte
<script lang="ts">
  import type { ShoppingList } from "$lib/types";

  interface Props {
    lists: ShoppingList[];
    activeListId: string;
    onSelectList: (listId: string) => void;
    onAddList: () => void;
  }

  let { lists, activeListId, onSelectList, onAddList }: Props = $props();
</script>

<div class="flex items-center gap-1 border-b border-gray-200">
  {#each lists as list (list.id)}
    <button
      type="button"
      onclick={() => onSelectList(list.id)}
      class="px-4 py-2 text-sm font-medium transition-colors relative
        {activeListId === list.id
          ? 'text-emerald-600'
          : 'text-gray-500 hover:text-gray-700 hover:bg-gray-50'}"
    >
      {list.name}
      {#if list.items.length > 0}
        <span class="ml-1 text-xs text-gray-400">({list.items.filter(i => !i.isOnHand).length})</span>
      {/if}
      {#if activeListId === list.id}
        <span class="absolute bottom-0 left-0 right-0 h-0.5 bg-emerald-600"></span>
      {/if}
    </button>
  {/each}

  <button
    type="button"
    onclick={onAddList}
    class="px-3 py-2 text-sm text-gray-400 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
    title="Add new list"
  >
    + New List
  </button>
</div>
```

#### View Toggle Component

**File**: `src/lib/components/shopping/ViewToggle.svelte` (new file)

**Change**: Toggle between category and recipe views

```svelte
<script lang="ts">
  type ViewMode = "category" | "recipe";

  interface Props {
    activeView: ViewMode;
    onViewChange: (view: ViewMode) => void;
  }

  let { activeView, onViewChange }: Props = $props();
</script>

<div class="flex items-center gap-2 text-sm">
  <span class="text-gray-500">View by:</span>
  <div class="flex rounded-lg border border-gray-200 overflow-hidden">
    <button
      type="button"
      onclick={() => onViewChange("category")}
      class="px-3 py-1.5 transition-colors
        {activeView === 'category' ? 'bg-emerald-100 text-emerald-700' : 'hover:bg-gray-50'}"
    >
      Category
    </button>
    <button
      type="button"
      onclick={() => onViewChange("recipe")}
      class="px-3 py-1.5 transition-colors
        {activeView === 'recipe' ? 'bg-emerald-100 text-emerald-700' : 'hover:bg-gray-50'}"
    >
      Recipe
    </button>
  </div>
</div>
```

#### Main Shopping Section Component

**File**: `src/lib/components/shopping/ShoppingSection.svelte` (new file)

**Change**: Main container orchestrating all shopping components

```svelte
<script lang="ts">
  import type { ShoppingItem, ShoppingList } from "$lib/types";
  import { GROCERY_CATEGORIES } from "$lib/types";
  import {
    weeklyShoppingListsStore,
    createWeekAggregatedList,
    getWeekStart,
    quickListsStore,
  } from "$lib/stores";
  import { derived } from "svelte/store";
  import ShoppingListTabs from "./ShoppingListTabs.svelte";
  import CategoryCard from "./CategoryCard.svelte";
  import ViewToggle from "./ViewToggle.svelte";
  import AddItemForm from "./AddItemForm.svelte";
  import QuickListsSidebar from "./QuickListsSidebar.svelte";

  interface Props {
    weekOffset: number;
  }

  let { weekOffset }: Props = $props();

  let activeListId = $state<string | null>(null);
  let viewMode = $state<"category" | "recipe">("category");
  let sidebarCollapsed = $state(true);
  let showAddListModal = $state(false);
  let newListName = $state("");

  // Get week start for current offset
  let weekStart = $derived(getWeekStart(weekOffset));

  // Ensure week exists in store
  $effect(() => {
    weeklyShoppingListsStore.getOrCreateWeek(weekStart);
  });

  // Get this week's lists
  let weekData = $derived.by(() => {
    let result: { lists: ShoppingList[] } = { lists: [] };
    weeklyShoppingListsStore.subscribe((weeks) => {
      const week = weeks.find((w) => w.weekStart === weekStart);
      if (week) result = week;
    })();
    return result;
  });

  // Set active list to first list if not set
  $effect(() => {
    if (!activeListId && weekData.lists.length > 0) {
      activeListId = weekData.lists[0].id;
    }
  });

  // Get aggregated items from meal plan for this week
  let aggregatedStore = $derived(createWeekAggregatedList(weekOffset));
  let aggregatedItems: ShoppingItem[] = $state([]);

  $effect(() => {
    const unsub = aggregatedStore.subscribe((items) => {
      aggregatedItems = items;
    });
    return unsub;
  });

  // Current active list
  let activeList = $derived(weekData.lists.find((l) => l.id === activeListId));

  // Combine aggregated items with list items for weekly list
  let displayItems = $derived.by(() => {
    if (!activeList) return [];

    if (activeList.type === "weekly") {
      // Merge aggregated (from recipes) with manual items
      const manualItems = activeList.items;
      return [...aggregatedItems, ...manualItems];
    }

    return activeList.items;
  });

  // Group items by category
  let itemsByCategory = $derived.by(() => {
    const groups = new Map<string, ShoppingItem[]>();

    GROCERY_CATEGORIES.forEach((cat) => groups.set(cat, []));

    displayItems.forEach((item) => {
      const category = GROCERY_CATEGORIES.includes(item.category as typeof GROCERY_CATEGORIES[number])
        ? item.category
        : "Other";
      const existing = groups.get(category) || [];
      groups.set(category, [...existing, item]);
    });

    // Filter out empty categories
    return Array.from(groups.entries()).filter(([_, items]) => items.length > 0);
  });

  // Group items by recipe
  let itemsByRecipe = $derived.by(() => {
    const groups = new Map<string, ShoppingItem[]>();
    const noRecipe: ShoppingItem[] = [];

    displayItems.forEach((item) => {
      if (item.sourceRecipeIds.length === 0) {
        noRecipe.push(item);
      } else {
        item.sourceRecipeIds.forEach((recipeId) => {
          const existing = groups.get(recipeId) || [];
          groups.set(recipeId, [...existing, item]);
        });
      }
    });

    if (noRecipe.length > 0) {
      groups.set("manual", noRecipe);
    }

    return Array.from(groups.entries());
  });

  // Stats
  let totalItems = $derived(displayItems.length);
  let toBuyCount = $derived(displayItems.filter((i) => !i.isOnHand).length);
  let onHandCount = $derived(displayItems.filter((i) => i.isOnHand).length);

  // Handlers
  function handleToggleOnHand(itemId: string) {
    if (!activeListId) return;
    weeklyShoppingListsStore.toggleItemOnHand(weekStart, activeListId, itemId);
  }

  function handleMoveItem(itemId: string, toListId: string) {
    if (!activeListId) return;
    weeklyShoppingListsStore.moveItem(weekStart, activeListId, toListId, itemId);
  }

  function handleRemoveItem(itemId: string) {
    if (!activeListId) return;
    weeklyShoppingListsStore.removeItem(weekStart, activeListId, itemId);
  }

  function handleAddItem(name: string, quantity: number, unit: string, category: string) {
    if (!activeListId) return;
    weeklyShoppingListsStore.addItem(weekStart, activeListId, {
      name,
      quantity,
      unit,
      category,
      isOnHand: false,
    });
  }

  function handleAddList() {
    if (!newListName.trim()) return;
    weeklyShoppingListsStore.addList(weekStart, newListName.trim());
    newListName = "";
    showAddListModal = false;
  }

  function handleAddFromQuickList(items: { name: string; quantity: number; unit: string; category: string }[]) {
    if (!activeListId) return;
    items.forEach((item) => {
      weeklyShoppingListsStore.addItem(weekStart, activeListId!, {
        ...item,
        isOnHand: false,
      });
    });
  }
</script>

<div class="flex gap-4">
  <!-- Quick Lists Sidebar -->
  <QuickListsSidebar
    collapsed={sidebarCollapsed}
    onToggle={() => sidebarCollapsed = !sidebarCollapsed}
    onAddItems={handleAddFromQuickList}
  />

  <!-- Main Content -->
  <div class="flex-1 min-w-0">
    <!-- Header -->
    <div class="flex items-center justify-between mb-4">
      <h2 class="text-xl font-bold text-gray-800">Shopping Lists</h2>
      <div class="flex items-center gap-4">
        <span class="text-sm text-gray-500">
          {toBuyCount} to buy · {onHandCount} on hand
        </span>
        <ViewToggle {viewMode} onViewChange={(v) => viewMode = v} />
      </div>
    </div>

    <!-- Tabs -->
    <ShoppingListTabs
      lists={weekData.lists}
      {activeListId}
      onSelectList={(id) => activeListId = id}
      onAddList={() => showAddListModal = true}
    />

    <!-- Add Item Form -->
    <div class="my-4">
      <AddItemForm onAdd={handleAddItem} />
    </div>

    <!-- Items Grid -->
    {#if viewMode === "category"}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each itemsByCategory as [category, items] (category)}
          <CategoryCard
            {category}
            {items}
            availableLists={weekData.lists}
            currentListId={activeListId || ""}
            onToggleOnHand={handleToggleOnHand}
            onMoveItem={handleMoveItem}
            onRemoveItem={handleRemoveItem}
          />
        {/each}
      </div>
    {:else}
      <!-- Recipe view - similar structure but grouped by recipe -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each itemsByRecipe as [recipeId, items] (recipeId)}
          <CategoryCard
            category={recipeId === "manual" ? "Manual Items" : recipeId}
            {items}
            availableLists={weekData.lists}
            currentListId={activeListId || ""}
            onToggleOnHand={handleToggleOnHand}
            onMoveItem={handleMoveItem}
            onRemoveItem={handleRemoveItem}
          />
        {/each}
      </div>
    {/if}

    {#if displayItems.length === 0}
      <div class="text-center py-12 text-gray-500">
        No items in this list. Add meals to your plan or add items manually.
      </div>
    {/if}
  </div>
</div>

<!-- Add List Modal -->
{#if showAddListModal}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-white rounded-xl p-6 w-96 shadow-xl">
      <h3 class="text-lg font-semibold mb-4">Add New List</h3>
      <input
        type="text"
        bind:value={newListName}
        placeholder="List name (e.g., Costco run)"
        class="w-full px-3 py-2 border border-gray-300 rounded-lg mb-4"
      />
      <div class="flex justify-end gap-2">
        <button
          type="button"
          onclick={() => showAddListModal = false}
          class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg"
        >
          Cancel
        </button>
        <button
          type="button"
          onclick={handleAddList}
          class="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700"
        >
          Add List
        </button>
      </div>
    </div>
  </div>
{/if}
```

#### Quick Lists Sidebar Component

**File**: `src/lib/components/shopping/QuickListsSidebar.svelte` (new file)

**Change**: Collapsible sidebar for Quick Lists

```svelte
<script lang="ts">
  import { quickListsStore, activeTab } from "$lib/stores";
  import type { QuickListItem } from "$lib/types";

  interface Props {
    collapsed: boolean;
    onToggle: () => void;
    onAddItems: (items: QuickListItem[]) => void;
  }

  let { collapsed, onToggle, onAddItems }: Props = $props();

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

<div
  class="flex-shrink-0 transition-all duration-300 ease-in-out
    {collapsed ? 'w-10' : 'w-64'}"
>
  {#if collapsed}
    <!-- Collapsed state - just a toggle button -->
    <button
      type="button"
      onclick={onToggle}
      class="w-10 h-10 flex items-center justify-center bg-white rounded-lg shadow-sm border border-gray-200 hover:bg-gray-50"
      title="Expand Quick Lists"
    >
      ▶
    </button>
  {:else}
    <!-- Expanded state -->
    <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
      <div class="px-4 py-3 border-b border-gray-100 flex items-center justify-between">
        <h3 class="font-semibold text-gray-800">Quick Lists</h3>
        <div class="flex items-center gap-2">
          <button
            type="button"
            onclick={() => activeTab.set("quicklists")}
            class="text-xs text-emerald-600 hover:text-emerald-700"
          >
            Manage
          </button>
          <button
            type="button"
            onclick={onToggle}
            class="text-gray-400 hover:text-gray-600"
            title="Collapse"
          >
            ◀
          </button>
        </div>
      </div>

      <div class="divide-y divide-gray-100 max-h-[400px] overflow-y-auto">
        {#each $quickListsStore as list (list.id)}
          <div>
            <button
              type="button"
              onclick={() => toggleList(list.id)}
              class="w-full flex items-center justify-between px-4 py-2 hover:bg-gray-50 transition-colors text-sm"
            >
              <span class="font-medium text-gray-700">{list.name}</span>
              <span class="text-gray-400 text-xs">
                {list.items.length}
                <span class="ml-1">{expandedList === list.id ? '▲' : '▼'}</span>
              </span>
            </button>

            {#if expandedList === list.id}
              <div class="px-3 pb-2 bg-gray-50">
                <div class="space-y-1 mb-2">
                  {#each list.items as item (item.id)}
                    <div class="flex items-center justify-between py-1 text-xs">
                      <span class="text-gray-600 truncate">
                        {item.quantity} {item.unit} {item.name}
                      </span>
                      <button
                        type="button"
                        onclick={() => addSingleItem(item)}
                        class="text-emerald-600 hover:text-emerald-700 flex-shrink-0"
                      >
                        +
                      </button>
                    </div>
                  {/each}
                </div>
                <button
                  type="button"
                  onclick={() => addAllItems(list.id)}
                  class="w-full py-1.5 text-xs bg-emerald-600 text-white rounded hover:bg-emerald-700"
                >
                  Add all
                </button>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>
```

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`

#### Integration Verification
- [ ] All new components importable
- [ ] Components compile without errors

#### Manual Verification
- [ ] Components can be rendered in isolation (optional)

**Checkpoint**: Pause for manual verification before proceeding to Phase 3.

---

## Phase 3: Meal Plan Integration

### Goal
Integrate the ShoppingSection component into MealPlan.svelte below the calendar.

### Integration Points

**Consumes from Phase 2**: `ShoppingSection.svelte`
**Produces**: Complete Meal Plan page with shopping functionality

**Wiring required**:
- [ ] Import ShoppingSection in `src/lib/components/MealPlan.svelte`
- [ ] Pass `weekOffset` prop to ShoppingSection

### Changes

#### Meal Plan Component

**File**: `src/lib/components/MealPlan.svelte`

**Change**: Add ShoppingSection below calendar

```svelte
<script lang="ts">
  import MealPlanCalendar from "./mealplan/MealPlanCalendar.svelte";
  import ShoppingSection from "./shopping/ShoppingSection.svelte";

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

<div class="max-w-7xl mx-auto">
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

  <!-- Shopping Section -->
  <div class="mt-8 pt-8 border-t border-gray-200">
    <ShoppingSection {weekOffset} />
  </div>
</div>
```

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`
- [ ] Tests pass: `pnpm test`

#### Integration Verification
- [ ] ShoppingSection renders below calendar
- [ ] `weekOffset` prop flows correctly

#### Manual Verification
- [ ] Navigate to Meal Plan page
- [ ] See meal calendar at top
- [ ] See shopping section below with tabs
- [ ] Week navigation updates both calendar and shopping
- [ ] Quick Lists sidebar expands/collapses
- [ ] Can add items from Quick Lists
- [ ] Category cards display correctly
- [ ] Can toggle items as on-hand
- [ ] Can move items between lists
- [ ] Can switch between category and recipe views

**Checkpoint**: Pause for manual verification before proceeding to Phase 4.

---

## Phase 4: Navigation Cleanup

### Goal
Remove the Shopping List tab from navigation and clean up unused code.

### Integration Points

**Consumes**: All prior phase outputs
**Produces**: Complete feature with clean navigation

**Wiring required**:
- [ ] Remove "shopping" from TabId type in `src/lib/stores/navigation.ts`
- [ ] Remove "Shopping List" from tabs array in `src/lib/components/TabNavigation.svelte`
- [ ] Remove ShoppingList case from `src/App.svelte`
- [ ] Remove ShoppingList import from `src/App.svelte`

### Changes

#### Navigation Store

**File**: `src/lib/stores/navigation.ts`

**Change**: Remove "shopping" from TabId

```typescript
import { writable } from "svelte/store";

export type TabId = "dashboard" | "recipes" | "mealplan" | "quicklists";

export const activeTab = writable<TabId>("dashboard");
```

#### Tab Navigation Component

**File**: `src/lib/components/TabNavigation.svelte`

**Change**: Remove Shopping List tab

```svelte
<script lang="ts">
  import { activeTab, type TabId } from "$lib/stores";

  const tabs: { id: TabId; label: string }[] = [
    { id: "dashboard", label: "Dashboard" },
    { id: "recipes", label: "Recipes" },
    { id: "mealplan", label: "Meal Plan" },
    { id: "quicklists", label: "Quick Lists" },
  ];
</script>

<nav class="bg-white border-b border-gray-200" aria-label="Main navigation">
  <div class="flex space-x-1 px-4">
    {#each tabs as tab}
      <button
        type="button"
        onclick={() => activeTab.set(tab.id)}
        class="px-4 py-3 text-sm font-medium transition-colors relative
          {$activeTab === tab.id
            ? 'text-emerald-600'
            : 'text-gray-500 hover:text-gray-700 hover:bg-gray-50'}"
        aria-current={$activeTab === tab.id ? "page" : undefined}
      >
        {tab.label}
        {#if $activeTab === tab.id}
          <span class="absolute bottom-0 left-0 right-0 h-0.5 bg-emerald-600 transition-all"></span>
        {/if}
      </button>
    {/each}
  </div>
</nav>
```

#### App Component

**File**: `src/App.svelte`

**Change**: Remove ShoppingList import and conditional rendering

```svelte
<script lang="ts">
  import TabNavigation from "$lib/components/TabNavigation.svelte";
  import { activeTab } from "$lib/stores";

  // Placeholder components - will be replaced in later phases
  import Dashboard from "$lib/components/Dashboard.svelte";
  import Recipes from "$lib/components/Recipes.svelte";
  import MealPlan from "$lib/components/MealPlan.svelte";
  import QuickListsManager from "$lib/components/QuickListsManager.svelte";
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
    {:else if $activeTab === "quicklists"}
      <QuickListsManager />
    {/if}
  </main>
</div>
```

#### Optional: Delete Unused File

**File**: `src/lib/components/ShoppingList.svelte`

**Change**: This file can be deleted or kept for reference. Recommend keeping temporarily until feature is verified working.

### Success Criteria

#### Automated Verification
- [ ] Types check: `pnpm check`
- [ ] Lint passes: `pnpm lint`
- [ ] Tests pass: `pnpm test`

#### Integration Verification
- [ ] No TypeScript errors about missing "shopping" tab
- [ ] All imports resolve correctly

#### Manual Verification
- [ ] App loads without errors
- [ ] Navigation shows: Dashboard, Recipes, Meal Plan, Quick Lists (4 tabs)
- [ ] No "Shopping List" tab visible
- [ ] Clicking "Meal Plan" shows combined meal + shopping view
- [ ] Quick Lists tab still works for template management

---

## Testing Strategy

### Unit Tests
- Store methods (addItem, moveItem, toggleOnHand)
- Unit conversion functions (if extracted)
- Week calculation helpers

### Integration Tests
- Week offset changes update both calendar and shopping
- Items from meal plan appear in weekly list

### Manual Testing Checklist
1. [ ] Navigate to Meal Plan page
2. [ ] See meal calendar with week navigation
3. [ ] See shopping section below calendar
4. [ ] Expand Quick Lists sidebar
5. [ ] Add items from Quick List to shopping
6. [ ] Collapse Quick Lists sidebar
7. [ ] Switch between Weekly and Mid-week tabs
8. [ ] Create a new custom list
9. [ ] Add manual item to list
10. [ ] Mark item as on-hand (checkbox)
11. [ ] Move item from Weekly to Mid-week list
12. [ ] Switch to Recipe view
13. [ ] Navigate to different week
14. [ ] Verify shopping lists update for new week
15. [ ] Verify Shopping List tab is removed
16. [ ] Verify Quick Lists tab still works

## Rollback Plan

Git revert to commit before Phase 1:
```bash
git revert --no-commit HEAD~N..HEAD
```

Since this is mock data only with no migrations, rollback is straightforward file restoration.

## Migration Notes

- **Data migration**: None required (mock data only)
- **Feature flags**: None
- **Backwards compatibility**: Not applicable (replacing existing UI)

## References

- Ticket: `ai_docs/prompts/2024-12-14-shopping-list-redesign.md`
- Current ShoppingList: `src/lib/components/ShoppingList.svelte`
- Current MealPlan: `src/lib/components/MealPlan.svelte`
