import { writable, derived, get } from "svelte/store";
import type { ShoppingItem, QuickList, QuickListItem, ShoppingList, WeeklyShoppingLists, ShoppingListType } from "$lib/types";
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
  getQuickLists,
  createQuickList as createQuickListBackend,
  updateQuickList,
  deleteQuickList as deleteQuickListBackend,
  addQuickListItem,
  updateQuickListItem,
  removeQuickListItem,
  addQuickListToShopping,
  type ShoppingListInput,
  type ShoppingItemInput,
  type ManualItemInput,
  type QuickListItemInput,
} from "$lib/tauri/commands";
import { toastStore } from "./toast";
import { mealPlanStore } from "./mealPlan";
import { recipeById } from "./recipes";

// Helper to get week start date (Monday)
function getWeekStart(weekOffset: number = 0): string {
  const today = new Date();
  const monday = new Date(today);
  monday.setDate(today.getDate() - today.getDay() + 1 + weekOffset * 7);
  return monday.toISOString().split("T")[0];
}

// Loading states
export const shoppingListsLoading = writable(false);
export const manualItemsLoading = writable(false);
export const quickListsLoading = writable(false);

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
      const input: ManualItemInput = {
        weekStart,
        name: item.name,
        quantity: item.quantity,
        unit: item.unit,
        category: item.category,
      };
      const created = await createManualItem(input);
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

// ============ Quick Lists Store ============

const quickListsInternal = writable<QuickList[]>([]);

async function loadQuickLists() {
  quickListsLoading.set(true);
  try {
    const lists = await getQuickLists();
    // Transform backend QuickListWithItems to frontend QuickList
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
      const created = await createQuickListBackend(name);
      quickListsInternal.update((lists) => [
        ...lists,
        { id: created.id, name: created.name, items: [] },
      ]);
    } catch (e) {
      toastStore.error("Failed to create list");
      throw e;
    }
  },

  removeList: async (id: string) => {
    try {
      await deleteQuickListBackend(id);
      quickListsInternal.update((lists) => lists.filter((l) => l.id !== id));
    } catch (e) {
      toastStore.error("Failed to delete list");
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
      toastStore.error("Failed to rename list");
      throw e;
    }
  },

  addItem: async (listId: string, item: Omit<QuickListItem, "id">) => {
    try {
      const input: QuickListItemInput = {
        name: item.name,
        quantity: item.quantity,
        unit: item.unit,
        category: item.category,
      };
      const created = await addQuickListItem(listId, input);
      quickListsInternal.update((lists) =>
        lists.map((l) =>
          l.id === listId
            ? { ...l, items: [...l.items, {
                id: created.id,
                name: created.name,
                quantity: created.quantity,
                unit: created.unit,
                category: created.category,
              }] }
            : l
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
          l.id === listId
            ? { ...l, items: l.items.filter((i) => i.id !== itemId) }
            : l
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
      const input: QuickListItemInput = {
        name: updates.name ?? item.name,
        quantity: updates.quantity ?? item.quantity,
        unit: updates.unit ?? item.unit,
        category: updates.category ?? item.category,
      };
      await updateQuickListItem(itemId, input);
      quickListsInternal.update((lists) =>
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
      // Caller should reload shopping lists to reflect changes
    } catch (e) {
      toastStore.error("Failed to add to shopping list");
      throw e;
    }
  },
};

// ============ Weekly Shopping Lists Store ============

const weeklyListsInternal = writable<WeeklyShoppingLists[]>([]);

async function loadWeeklyShoppingLists(weekStart: string) {
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
          type: list.listType as ShoppingListType,
          items: list.items.map((item) => ({
            id: item.id,
            name: item.name,
            quantity: item.quantity,
            unit: item.unit,
            category: item.category,
            isOnHand: item.isChecked, // Backend uses isChecked
            isManual: false,
            sourceRecipeIds: item.sourceRecipeIds?.split(",").filter(Boolean) ?? [],
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
}

export const weeklyShoppingListsStore = {
  subscribe: weeklyListsInternal.subscribe,
  load: loadWeeklyShoppingLists,

  // Get or create lists for a specific week
  getOrCreateWeek: async (weekStart: string) => {
    const weeks = get(weeklyListsInternal);
    const existing = weeks.find((w) => w.weekStart === weekStart);
    if (existing) return;

    // Load from backend - it will create default lists if needed
    await loadWeeklyShoppingLists(weekStart);
  },

  // Add a new custom list to a week
  addList: async (weekStart: string, name: string) => {
    try {
      const input: ShoppingListInput = {
        weekStart,
        name,
        listType: "custom",
      };
      const created = await createShoppingList(input);
      weeklyListsInternal.update((weeks) =>
        weeks.map((w) =>
          w.weekStart === weekStart
            ? {
                ...w,
                lists: [
                  ...w.lists,
                  { id: created.id, name: created.name, type: "custom" as ShoppingListType, items: [] },
                ],
              }
            : w
        )
      );
    } catch (e) {
      toastStore.error("Failed to create list");
      throw e;
    }
  },

  // Remove a custom list
  removeList: async (weekStart: string, listId: string) => {
    try {
      await deleteShoppingList(listId);
      weeklyListsInternal.update((weeks) =>
        weeks.map((w) =>
          w.weekStart === weekStart
            ? { ...w, lists: w.lists.filter((l) => l.id !== listId) }
            : w
        )
      );
    } catch (e) {
      toastStore.error("Failed to delete list");
      throw e;
    }
  },

  // Add item to a specific list
  addItem: async (weekStart: string, listId: string, item: Omit<ShoppingItem, "id" | "isManual" | "sourceRecipeIds">) => {
    try {
      const input: ShoppingItemInput = {
        listId,
        name: item.name,
        quantity: item.quantity,
        unit: item.unit,
        category: item.category,
      };
      const created = await addShoppingItem(input);
      weeklyListsInternal.update((weeks) =>
        weeks.map((w) => {
          if (w.weekStart !== weekStart) return w;

          return {
            ...w,
            lists: w.lists.map((l) => {
              if (l.id !== listId) return l;

              return {
                ...l,
                items: [
                  ...l.items,
                  {
                    id: created.id,
                    name: created.name,
                    quantity: created.quantity,
                    unit: created.unit,
                    category: created.category,
                    isOnHand: created.isChecked,
                    isManual: true,
                    sourceRecipeIds: [],
                  },
                ],
              };
            }),
          };
        })
      );
    } catch (e) {
      toastStore.error("Failed to add item");
      throw e;
    }
  },

  // Move item between lists
  moveItem: async (weekStart: string, fromListId: string, toListId: string, itemId: string) => {
    try {
      await moveShoppingItem(itemId, toListId);
      weeklyListsInternal.update((weeks) =>
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
      );
    } catch (e) {
      toastStore.error("Failed to move item");
      throw e;
    }
  },

  // Toggle item on-hand status
  toggleItemOnHand: async (weekStart: string, listId: string, itemId: string) => {
    const weeks = get(weeklyListsInternal);
    const week = weeks.find((w) => w.weekStart === weekStart);
    const list = week?.lists.find((l) => l.id === listId);
    const item = list?.items.find((i) => i.id === itemId);
    if (!item) return;

    try {
      await updateShoppingItem(itemId, undefined, !item.isOnHand);
      weeklyListsInternal.update((weeks) =>
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
      );
    } catch (e) {
      toastStore.error("Failed to update item");
      throw e;
    }
  },

  // Hard delete - completely remove item from list
  removeItem: async (weekStart: string, listId: string, itemId: string) => {
    try {
      // Use soft delete backend - for hard delete we would need a different backend call
      await softDeleteShoppingItem(itemId);
      weeklyListsInternal.update((weeks) =>
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
      );
    } catch (e) {
      toastStore.error("Failed to remove item");
      throw e;
    }
  },

  // Soft delete - hide item but keep it (can be restored)
  softDeleteItem: async (weekStart: string, listId: string, itemId: string) => {
    try {
      await softDeleteShoppingItem(itemId);
      weeklyListsInternal.update((weeks) =>
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
      );
    } catch (e) {
      toastStore.error("Failed to delete item");
      throw e;
    }
  },

  // Restore soft-deleted item
  restoreItem: async (weekStart: string, listId: string, itemId: string) => {
    try {
      await restoreShoppingItem(itemId);
      weeklyListsInternal.update((weeks) =>
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
      );
    } catch (e) {
      toastStore.error("Failed to restore item");
      throw e;
    }
  },

  // Track that an aggregated item was moved to another list (for visual indication)
  markItemMoved: async (weekStart: string, listId: string, itemId: string, toListId: string, toListName: string) => {
    // This is currently client-side only for visual indication
    // The backend tracks movedToListId separately
    weeklyListsInternal.update((weeks) =>
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
    );
  },
};

// ============ Aggregated Shopping List ============
// Keep client-side derivation for reactivity, backend aggregation available for cross-session consistency

export const aggregatedShoppingList = derived(
  [mealPlanStore, recipeById, manualItemsInternal],
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
