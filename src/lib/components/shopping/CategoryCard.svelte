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
    onSoftDeleteItem: (itemId: string) => void;
    onHardDeleteItem: (itemId: string) => void;
    onRestoreItem: (itemId: string) => void;
    showDeleted?: boolean;
  }

  let {
    category,
    items,
    availableLists,
    currentListId,
    onToggleOnHand,
    onMoveItem,
    onSoftDeleteItem,
    onHardDeleteItem,
    onRestoreItem,
    showDeleted = false,
  }: Props = $props();

  // Filter items based on showDeleted flag
  let visibleItems = $derived(
    showDeleted ? items : items.filter((i) => !i.isDeleted)
  );
  let deletedCount = $derived(items.filter((i) => i.isDeleted).length);

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
  let toBuyCount = $derived(visibleItems.filter((i) => !i.isOnHand && !i.isDeleted).length);
</script>

{#if visibleItems.length > 0 || (showDeleted && deletedCount > 0)}
<div class="rounded-xl border-2 {colorClass} overflow-hidden">
  <button
    type="button"
    onclick={() => isCollapsed = !isCollapsed}
    class="w-full px-3 sm:px-4 py-2 sm:py-3 flex items-center justify-between hover:bg-white/50 transition-colors"
  >
    <div class="flex items-center gap-2 flex-wrap">
      <h3 class="font-semibold text-gray-800 text-sm sm:text-base">{category}</h3>
      <span class="text-xs sm:text-sm text-gray-500">({toBuyCount} to buy)</span>
      {#if deletedCount > 0 && !showDeleted}
        <span class="text-xs text-gray-400">+{deletedCount} hidden</span>
      {/if}
    </div>
    <span class="text-gray-400">{isCollapsed ? '\u25B6' : '\u25BC'}</span>
  </button>

  {#if !isCollapsed}
    <div class="divide-y divide-gray-100 bg-white/50">
      {#each visibleItems as item (item.id)}
        <ShoppingItemRow
          {item}
          {availableLists}
          {currentListId}
          onToggle={() => onToggleOnHand(item.id)}
          onMove={(toListId) => onMoveItem(item.id, toListId)}
          onSoftDelete={() => onSoftDeleteItem(item.id)}
          onHardDelete={item.isManual ? () => onHardDeleteItem(item.id) : undefined}
          onRestore={item.isDeleted ? () => onRestoreItem(item.id) : undefined}
        />
      {/each}
    </div>
  {/if}
</div>
{/if}
