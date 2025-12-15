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
  let editQuantity = $state(0);

  // Reset editQuantity when entering edit mode (handled in onclick below)

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
      class="text-gray-400 hover:text-red-500 transition-colors"
      aria-label="Remove item"
    >
      x
    </button>
  {/if}
</div>
