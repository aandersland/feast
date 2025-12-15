import { writable, derived } from "svelte/store";
import type { ShoppingItem, QuickList, QuickListItem, ShoppingList, WeeklyShoppingLists, ShoppingListType } from "$lib/types";
import { mealPlanStore } from "./mealPlan";
import { recipeById } from "./recipes";

// Helper to get week start date (Monday)
function getWeekStart(weekOffset: number = 0): string {
  const today = new Date();
  const monday = new Date(today);
  monday.setDate(today.getDate() - today.getDay() + 1 + weekOffset * 7);
  return monday.toISOString().split("T")[0];
}

const mockQuickLists: QuickList[] = [
  {
    id: "1",
    name: "Pantry Staples",
    items: [
      { id: "ql1-1", name: "Olive oil", quantity: 1, unit: "bottle", category: "Oils" },
      { id: "ql1-2", name: "Salt", quantity: 1, unit: "container", category: "Spices" },
      { id: "ql1-3", name: "Black pepper", quantity: 1, unit: "container", category: "Spices" },
    ],
  },
  {
    id: "2",
    name: "Weekly Basics",
    items: [
      { id: "ql2-1", name: "Milk", quantity: 1, unit: "gallon", category: "Dairy" },
      { id: "ql2-2", name: "Eggs", quantity: 1, unit: "dozen", category: "Dairy" },
      { id: "ql2-3", name: "Bread", quantity: 1, unit: "loaf", category: "Bakery" },
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

function createQuickListsStore() {
  const { subscribe, set, update } = writable<QuickList[]>(mockQuickLists);

  return {
    subscribe,
    addList: (name: string) =>
      update((lists) => [
        ...lists,
        { id: crypto.randomUUID(), name, items: [] },
      ]),
    removeList: (id: string) =>
      update((lists) => lists.filter((l) => l.id !== id)),
    renameList: (id: string, name: string) =>
      update((lists) =>
        lists.map((l) => (l.id === id ? { ...l, name } : l))
      ),
    addItem: (listId: string, item: Omit<QuickListItem, "id">) =>
      update((lists) =>
        lists.map((l) =>
          l.id === listId
            ? { ...l, items: [...l.items, { ...item, id: crypto.randomUUID() }] }
            : l
        )
      ),
    removeItem: (listId: string, itemId: string) =>
      update((lists) =>
        lists.map((l) =>
          l.id === listId
            ? { ...l, items: l.items.filter((i) => i.id !== itemId) }
            : l
        )
      ),
    updateItem: (listId: string, itemId: string, updates: Partial<QuickListItem>) =>
      update((lists) =>
        lists.map((l) =>
          l.id === listId
            ? {
                ...l,
                items: l.items.map((i) =>
                  i.id === itemId ? { ...i, ...updates } : i
                ),
              }
            : l
        )
      ),
  };
}

export const quickListsStore = createQuickListsStore();

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

// Weekly Shopping Lists Store
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

    // Add item to a specific list (combines quantities if item already exists)
    addItem: (weekStart: string, listId: string, item: Omit<ShoppingItem, "id" | "isManual" | "sourceRecipeIds">) =>
      update((weeks) =>
        weeks.map((w) => {
          if (w.weekStart !== weekStart) return w;

          return {
            ...w,
            lists: w.lists.map((l) => {
              if (l.id !== listId) return l;

              // Check if item with same name and unit already exists
              const existingItem = l.items.find(
                (i) => i.name.toLowerCase() === item.name.toLowerCase() && i.unit === item.unit
              );

              if (existingItem) {
                // Combine quantities with existing item
                return {
                  ...l,
                  items: l.items.map((i) =>
                    i.id === existingItem.id
                      ? { ...i, quantity: i.quantity + item.quantity }
                      : i
                  ),
                };
              }

              // Add as new item
              return {
                ...l,
                items: [
                  ...l.items,
                  { ...item, id: crypto.randomUUID(), isManual: true, sourceRecipeIds: [] },
                ],
              };
            }),
          };
        })
      ),

    // Move item between lists (combines quantities if item already exists in target)
    moveItem: (weekStart: string, fromListId: string, toListId: string, itemId: string) =>
      update((weeks) =>
        weeks.map((w) => {
          if (w.weekStart !== weekStart) return w;

          const fromList = w.lists.find((l) => l.id === fromListId);
          const toList = w.lists.find((l) => l.id === toListId);
          const item = fromList?.items.find((i) => i.id === itemId);
          if (!item || !toList) return w;

          // Check if item with same name and unit already exists in target list
          const existingItem = toList.items.find(
            (i) => i.name.toLowerCase() === item.name.toLowerCase() && i.unit === item.unit
          );

          return {
            ...w,
            lists: w.lists.map((l) => {
              if (l.id === fromListId) {
                return { ...l, items: l.items.filter((i) => i.id !== itemId) };
              }
              if (l.id === toListId) {
                if (existingItem) {
                  // Combine quantities with existing item
                  return {
                    ...l,
                    items: l.items.map((i) =>
                      i.id === existingItem.id
                        ? {
                            ...i,
                            quantity: i.quantity + item.quantity,
                            sourceRecipeIds: [...new Set([...i.sourceRecipeIds, ...item.sourceRecipeIds])],
                          }
                        : i
                    ),
                  };
                }
                // Add as new item
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

    // Hard delete - completely remove item from list
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

    // Soft delete - hide item but keep it (can be restored)
    softDeleteItem: (weekStart: string, listId: string, itemId: string) =>
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
                          i.id === itemId
                            ? { ...i, isDeleted: true, deletedAt: new Date().toISOString() }
                            : i
                        ),
                      }
                    : l
                ),
              }
            : w
        )
      ),

    // Restore soft-deleted item
    restoreItem: (weekStart: string, listId: string, itemId: string) =>
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
                          i.id === itemId
                            ? { ...i, isDeleted: false, deletedAt: undefined }
                            : i
                        ),
                      }
                    : l
                ),
              }
            : w
        )
      ),

    // Track that an aggregated item was moved to another list (for visual indication)
    markItemMoved: (weekStart: string, listId: string, itemId: string, toListId: string, toListName: string) =>
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
                          i.id === itemId
                            ? { ...i, movedToListId: toListId, movedToListName: toListName }
                            : i
                        ),
                      }
                    : l
                ),
              }
            : w
        )
      ),
  };
}

export const weeklyShoppingListsStore = createWeeklyShoppingListsStore();

// Store for tracking soft-deleted aggregated item IDs (keyed by weekStart-listId-itemId)
function createSoftDeletedAggregatedStore() {
  const { subscribe, update } = writable<Set<string>>(new Set());

  return {
    subscribe,
    softDelete: (weekStart: string, listId: string, itemId: string) =>
      update((set) => {
        const newSet = new Set(set);
        newSet.add(`${weekStart}-${listId}-${itemId}`);
        return newSet;
      }),
    restore: (weekStart: string, listId: string, itemId: string) =>
      update((set) => {
        const newSet = new Set(set);
        newSet.delete(`${weekStart}-${listId}-${itemId}`);
        return newSet;
      }),
    isDeleted: (set: Set<string>, weekStart: string, listId: string, itemId: string) =>
      set.has(`${weekStart}-${listId}-${itemId}`),
  };
}

export const softDeletedAggregatedStore = createSoftDeletedAggregatedStore();

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
                  category: "Other", // Ingredient type doesn't have category
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

// Export helper to get week start date
export { getWeekStart };
