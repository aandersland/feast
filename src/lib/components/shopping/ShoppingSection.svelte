<script lang="ts">
  import type { ShoppingItem, ShoppingList, QuickListItem } from "$lib/types";
  import { GROCERY_CATEGORIES } from "$lib/types";
  import {
    weeklyShoppingListsStore,
    createWeekAggregatedList,
    getWeekStart,
    quickListsStore,
    recipeById,
    softDeletedAggregatedStore,
  } from "$lib/stores";
  import ShoppingListTabs from "./ShoppingListTabs.svelte";
  import CategoryCard from "./CategoryCard.svelte";
  import ViewToggle from "./ViewToggle.svelte";
  import AddItemForm from "./AddItemForm.svelte";
  import QuickListsDropdown from "./QuickListsDropdown.svelte";

  interface Props {
    weekOffset: number;
  }

  let { weekOffset }: Props = $props();

  let activeListId = $state<string | null>(null);
  let viewMode = $state<"category" | "recipe">("category");
  let showAddListModal = $state(false);
  let newListName = $state("");

  // Get week start for current offset
  let weekStart = $derived(getWeekStart(weekOffset));

  // Ensure week exists in store
  $effect(() => {
    weeklyShoppingListsStore.getOrCreateWeek(weekStart);
  });

  // Get this week's lists - use $state to store the data and $effect to subscribe
  let weekData = $state<{ lists: ShoppingList[] }>({ lists: [] });

  $effect(() => {
    const unsub = weeklyShoppingListsStore.subscribe((weeks) => {
      const week = weeks.find((w) => w.weekStart === weekStart);
      weekData = week || { lists: [] };
    });
    return unsub;
  });

  // Set active list to first list if not set
  $effect(() => {
    if (!activeListId && weekData.lists.length > 0) {
      activeListId = weekData.lists[0].id;
    }
  });

  // Get aggregated items from meal plan for this week
  let aggregatedStore = $derived(createWeekAggregatedList(weekOffset));
  let rawAggregatedItems: ShoppingItem[] = $state([]);
  let softDeletedSet: Set<string> = $state(new Set());

  $effect(() => {
    const unsub = aggregatedStore.subscribe((items) => {
      rawAggregatedItems = items;
    });
    return unsub;
  });

  $effect(() => {
    const unsub = softDeletedAggregatedStore.subscribe((set) => {
      softDeletedSet = set;
    });
    return unsub;
  });

  // Merge soft delete state into aggregated items
  let aggregatedItems = $derived.by(() => {
    if (!activeListId) return rawAggregatedItems;
    return rawAggregatedItems.map((item) => ({
      ...item,
      isDeleted: softDeletedAggregatedStore.isDeleted(softDeletedSet, weekStart, activeListId!, item.id),
    }));
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

  // Group items by recipe - returns array of [recipeName, items] pairs
  let itemsByRecipe = $derived.by(() => {
    const groups = new Map<string, ShoppingItem[]>();
    const noRecipe: ShoppingItem[] = [];
    const recipes = $recipeById;

    displayItems.forEach((item) => {
      if (item.sourceRecipeIds.length === 0) {
        noRecipe.push(item);
      } else {
        item.sourceRecipeIds.forEach((recipeId) => {
          // Use recipe name instead of ID
          const recipe = recipes.get(recipeId);
          const recipeName = recipe?.name || `Recipe ${recipeId}`;
          const existing = groups.get(recipeName) || [];
          groups.set(recipeName, [...existing, item]);
        });
      }
    });

    if (noRecipe.length > 0) {
      groups.set("Manual Items", noRecipe);
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

    // Find the item in displayItems (could be aggregated or from list)
    const item = displayItems.find(i => i.id === itemId);
    if (!item) return;

    // Check if item is in the current list's items (manual/stored item)
    const isStoredItem = activeList?.items.some(i => i.id === itemId);

    if (isStoredItem) {
      // Item exists in store, use moveItem
      weeklyShoppingListsStore.moveItem(weekStart, activeListId, toListId, itemId);
    } else {
      // Aggregated item - add a copy to the target list
      weeklyShoppingListsStore.addItem(weekStart, toListId, {
        name: item.name,
        quantity: item.quantity,
        unit: item.unit,
        category: item.category,
        isOnHand: item.isOnHand,
      });
    }
  }

  function handleSoftDeleteItem(itemId: string) {
    if (!activeListId) return;

    // Check if item is in the stored list or aggregated
    const isStoredItem = activeList?.items.some(i => i.id === itemId);

    if (isStoredItem) {
      weeklyShoppingListsStore.softDeleteItem(weekStart, activeListId, itemId);
    } else {
      // Aggregated item - use the separate store
      softDeletedAggregatedStore.softDelete(weekStart, activeListId, itemId);
    }
  }

  function handleHardDeleteItem(itemId: string) {
    if (!activeListId) return;
    weeklyShoppingListsStore.removeItem(weekStart, activeListId, itemId);
  }

  function handleRestoreItem(itemId: string) {
    if (!activeListId) return;

    // Check if item is in the stored list or aggregated
    const isStoredItem = activeList?.items.some(i => i.id === itemId);

    if (isStoredItem) {
      weeklyShoppingListsStore.restoreItem(weekStart, activeListId, itemId);
    } else {
      // Aggregated item - use the separate store
      softDeletedAggregatedStore.restore(weekStart, activeListId, itemId);
    }
  }

  let showDeletedItems = $state(false);

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

  function handleAddFromQuickList(items: QuickListItem[]) {
    if (!activeListId) return;
    items.forEach((item) => {
      weeklyShoppingListsStore.addItem(weekStart, activeListId!, {
        name: item.name,
        quantity: item.quantity,
        unit: item.unit,
        category: item.category,
        isOnHand: false,
      });
    });
  }
</script>

<div class="flex flex-col">
  <!-- Header -->
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2 mb-4">
    <h2 class="text-lg sm:text-xl font-bold text-gray-800">Shopping Lists</h2>
    <div class="flex flex-wrap items-center gap-2 sm:gap-4">
      <span class="text-xs sm:text-sm text-gray-500">
        {toBuyCount} to buy · {onHandCount} on hand
      </span>
      <label class="flex items-center gap-1 text-xs text-gray-500 cursor-pointer">
        <input type="checkbox" bind:checked={showDeletedItems} class="rounded" />
        Show hidden
      </label>
      <ViewToggle activeView={viewMode} onViewChange={(v) => viewMode = v} />
    </div>
  </div>

  <!-- Tabs -->
  <ShoppingListTabs
    lists={weekData.lists}
    {activeListId}
    onSelectList={(id) => activeListId = id}
    onAddList={() => showAddListModal = true}
  />

  <!-- Add Item Form and Quick Lists Dropdown -->
  <div class="my-4 flex flex-col sm:flex-row gap-2">
    <div class="flex-1">
      <AddItemForm onAdd={handleAddItem} />
    </div>
    <QuickListsDropdown onAddItems={handleAddFromQuickList} />
  </div>

  <!-- Items Grid -->
  {#if viewMode === "category"}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3 sm:gap-4">
      {#each itemsByCategory as [category, items] (category)}
        <CategoryCard
          {category}
          {items}
          availableLists={weekData.lists}
          currentListId={activeListId || ""}
          onToggleOnHand={handleToggleOnHand}
          onMoveItem={handleMoveItem}
          onSoftDeleteItem={handleSoftDeleteItem}
          onHardDeleteItem={handleHardDeleteItem}
          onRestoreItem={handleRestoreItem}
          showDeleted={showDeletedItems}
        />
      {/each}
    </div>
  {:else}
    <!-- Recipe view - similar structure but grouped by recipe -->
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3 sm:gap-4">
      {#each itemsByRecipe as [recipeName, items] (recipeName)}
        <CategoryCard
          category={recipeName}
          {items}
          availableLists={weekData.lists}
          currentListId={activeListId || ""}
          onToggleOnHand={handleToggleOnHand}
          onMoveItem={handleMoveItem}
          onSoftDeleteItem={handleSoftDeleteItem}
          onHardDeleteItem={handleHardDeleteItem}
          onRestoreItem={handleRestoreItem}
          showDeleted={showDeletedItems}
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
