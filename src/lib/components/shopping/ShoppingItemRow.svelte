<script lang="ts">
  import type { ShoppingItem, ShoppingList } from "$lib/types";
  import { recipeById } from "$lib/stores";

  interface Props {
    item: ShoppingItem;
    availableLists: ShoppingList[];
    currentListId: string;
    onToggle: () => void;
    onMove: (toListId: string) => void;
    onSoftDelete?: () => void;
    onHardDelete?: () => void;
    onRestore?: () => void;
  }

  let { item, availableLists, currentListId, onToggle, onMove, onSoftDelete, onHardDelete, onRestore }: Props = $props();

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

  // Color coding for lists
  const listColors: Record<string, string> = {
    'weekly': 'bg-emerald-100 text-emerald-700',
    'midweek': 'bg-blue-100 text-blue-700',
    'custom': 'bg-purple-100 text-purple-700',
  };

  // Get list color for moved indicator
  const movedToList = $derived(
    item.movedToListId ? availableLists.find((l) => l.id === item.movedToListId) : null
  );
</script>

<div
  class="flex flex-wrap sm:flex-nowrap items-center gap-2 sm:gap-3 px-3 sm:px-4 py-2 sm:py-3 hover:bg-white transition-colors
    {item.isDeleted ? 'opacity-40 bg-gray-100' : ''}
    {item.isOnHand ? 'opacity-50' : ''}
    {item.movedToListId ? 'bg-gray-50' : ''}"
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
    {#if item.isOnHand}<span class="text-xs">{'\u2713'}</span>{/if}
  </button>

  <!-- Item info -->
  <div class="flex-1 min-w-0 min-w-[120px]">
    <div class="font-medium text-gray-800 {item.isOnHand || item.isDeleted ? 'line-through' : ''}">
      {item.name}
    </div>
    <div class="flex flex-wrap items-center gap-1">
      {#if sourceNames && !item.isManual}
        <span class="text-xs text-gray-400 truncate">From: {sourceNames}</span>
      {/if}
      {#if item.movedToListName}
        <span class="text-xs px-1.5 py-0.5 rounded {listColors[movedToList?.type || 'custom']}">
          {'\u2192'} {item.movedToListName}
        </span>
      {/if}
    </div>
  </div>

  <!-- Quantity with imperial/metric -->
  <div class="text-sm text-gray-600 whitespace-nowrap">
    {formatQuantity(item.quantity, item.unit)}
  </div>

  <!-- Action buttons -->
  <div class="flex items-center gap-1 w-full sm:w-auto justify-end">
    {#if item.isDeleted}
      <!-- Restore button for soft-deleted items -->
      {#if onRestore}
        <button
          type="button"
          onclick={onRestore}
          class="p-1.5 text-xs rounded bg-emerald-100 text-emerald-700 hover:bg-emerald-200 transition-colors"
          title="Restore item"
        >
          {'\u21B6'} Restore
        </button>
      {/if}
      <!-- Hard delete for soft-deleted items -->
      {#if onHardDelete}
        <button
          type="button"
          onclick={onHardDelete}
          class="p-1.5 text-xs rounded bg-red-100 text-red-700 hover:bg-red-200 transition-colors"
          title="Delete permanently"
        >
          {'\u2715'}
        </button>
      {/if}
    {:else}
      <!-- Move to list buttons -->
      {#each otherLists as list (list.id)}
        <button
          type="button"
          onclick={() => onMove(list.id)}
          class="p-1.5 text-xs rounded hover:bg-gray-100 transition-colors hidden sm:block"
          title="Move to {list.name}"
        >
          {'\u2192'} {list.name.substring(0, 1)}
        </button>
      {/each}

      <!-- Soft delete button (eye icon - hide) -->
      {#if onSoftDelete}
        <button
          type="button"
          onclick={onSoftDelete}
          class="p-1 text-gray-400 hover:text-amber-500 transition-colors"
          title="Hide item (can restore later)"
        >
          {'\u2298'}
        </button>
      {/if}

      <!-- Hard delete button (trash) -->
      {#if onHardDelete}
        <button
          type="button"
          onclick={onHardDelete}
          class="p-1 text-gray-400 hover:text-red-500 transition-colors"
          title="Delete permanently"
        >
          {'\u2715'}
        </button>
      {/if}
    {/if}
  </div>
</div>
