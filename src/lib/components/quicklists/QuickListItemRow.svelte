<script lang="ts">
  import type { QuickListItem } from "$lib/types";

  interface Props {
    item: QuickListItem;
    onUpdate: (updates: Partial<QuickListItem>) => void;
    onRemove: () => void;
  }

  let { item, onUpdate, onRemove }: Props = $props();

  let isEditing = $state(false);
  let editQuantity = $state(item.quantity);
  let editUnit = $state(item.unit);
  let editName = $state(item.name);
  let editCategory = $state(item.category);

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

  function startEdit() {
    editQuantity = item.quantity;
    editUnit = item.unit;
    editName = item.name;
    editCategory = item.category;
    isEditing = true;
  }

  function saveEdit() {
    onUpdate({
      quantity: editQuantity,
      unit: editUnit,
      name: editName,
      category: editCategory,
    });
    isEditing = false;
  }

  function cancelEdit() {
    isEditing = false;
  }
</script>

{#if isEditing}
  <div class="flex flex-wrap items-center gap-2 py-2 px-3 bg-gray-50 rounded-lg">
    <input
      type="number"
      bind:value={editQuantity}
      min="1"
      class="w-16 px-2 py-1 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-emerald-500"
    />
    <input
      type="text"
      bind:value={editUnit}
      placeholder="unit"
      class="w-20 px-2 py-1 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-emerald-500"
    />
    <input
      type="text"
      bind:value={editName}
      placeholder="name"
      class="flex-1 min-w-[120px] px-2 py-1 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-emerald-500"
    />
    <select
      bind:value={editCategory}
      class="px-2 py-1 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-emerald-500"
    >
      {#each categories as cat}
        <option value={cat}>{cat}</option>
      {/each}
    </select>
    <div class="flex gap-1">
      <button
        type="button"
        onclick={saveEdit}
        class="px-2 py-1 text-sm bg-emerald-600 text-white rounded hover:bg-emerald-700"
      >
        Save
      </button>
      <button
        type="button"
        onclick={cancelEdit}
        class="px-2 py-1 text-sm bg-gray-200 text-gray-700 rounded hover:bg-gray-300"
      >
        Cancel
      </button>
    </div>
  </div>
{:else}
  <div class="flex items-center justify-between py-2 px-3 hover:bg-gray-50 rounded-lg group">
    <div class="flex items-center gap-2 text-sm">
      <span class="text-gray-700">{item.quantity} {item.unit}</span>
      <span class="font-medium text-gray-800">{item.name}</span>
      <span class="text-gray-400 text-xs">({item.category})</span>
    </div>
    <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
      <button
        type="button"
        onclick={startEdit}
        class="p-1 text-gray-400 hover:text-emerald-600 rounded"
        aria-label="Edit item"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
        </svg>
      </button>
      <button
        type="button"
        onclick={onRemove}
        class="p-1 text-gray-400 hover:text-red-600 rounded"
        aria-label="Remove item"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  </div>
{/if}
