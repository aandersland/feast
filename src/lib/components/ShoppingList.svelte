<script lang="ts">
  import { onMount } from "svelte";
  import { aggregatedShoppingList, manualItemsStore, getWeekStart } from "$lib/stores";
  import type { QuickListItem } from "$lib/types";
  import ShoppingItem from "./shopping/ShoppingItem.svelte";
  import AddItemForm from "./shopping/AddItemForm.svelte";
  import QuickLists from "./shopping/QuickLists.svelte";

  // Track on-hand state for aggregated items (not in store)
  let onHandIds = $state(new Set<string>());

  // Get current week start for manual items
  const weekStart = getWeekStart(0);

  // Load manual items on mount
  onMount(() => {
    manualItemsStore.load(weekStart);
  });

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

  async function handleAddItem(name: string, quantity: number, unit: string, category: string) {
    await manualItemsStore.add(weekStart, { name, quantity, unit, category, isOnHand: false });
  }

  async function handleRemoveItem(id: string) {
    await manualItemsStore.remove(id);
  }

  async function handleAddFromQuickList(items: QuickListItem[]) {
    for (const item of items) {
      await manualItemsStore.add(weekStart, {
        name: item.name,
        quantity: item.quantity,
        unit: item.unit,
        category: item.category,
        isOnHand: false,
      });
    }
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
