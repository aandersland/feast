<script lang="ts">
  import type { QuickListItem } from "$lib/types";

  interface Props {
    onAdd: (item: Omit<QuickListItem, "id">) => void;
  }

  let { onAdd }: Props = $props();

  let isExpanded = $state(false);
  let quantity = $state(1);
  let unit = $state("");
  let name = $state("");
  let category = $state("Other");

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

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!name.trim()) return;

    onAdd({
      name: name.trim(),
      quantity,
      unit: unit.trim(),
      category,
    });

    // Reset form
    quantity = 1;
    unit = "";
    name = "";
    category = "Other";
    isExpanded = false;
  }
</script>

{#if isExpanded}
  <form onsubmit={handleSubmit} class="p-3 bg-gray-50 rounded-lg border border-gray-200">
    <div class="flex flex-wrap gap-2 mb-3">
      <input
        type="number"
        bind:value={quantity}
        min="1"
        placeholder="Qty"
        class="w-16 px-2 py-1.5 text-sm border border-gray-300 rounded-lg focus:outline-none focus:ring-1 focus:ring-emerald-500"
      />
      <input
        type="text"
        bind:value={unit}
        placeholder="unit"
        class="w-20 px-2 py-1.5 text-sm border border-gray-300 rounded-lg focus:outline-none focus:ring-1 focus:ring-emerald-500"
      />
      <input
        type="text"
        bind:value={name}
        placeholder="Item name"
        class="flex-1 min-w-[140px] px-2 py-1.5 text-sm border border-gray-300 rounded-lg focus:outline-none focus:ring-1 focus:ring-emerald-500"
      />
      <select
        bind:value={category}
        class="px-2 py-1.5 text-sm border border-gray-300 rounded-lg focus:outline-none focus:ring-1 focus:ring-emerald-500"
      >
        {#each categories as cat}
          <option value={cat}>{cat}</option>
        {/each}
      </select>
    </div>
    <div class="flex justify-end gap-2">
      <button
        type="button"
        onclick={() => (isExpanded = false)}
        class="px-3 py-1.5 text-sm text-gray-600 hover:text-gray-800"
      >
        Cancel
      </button>
      <button
        type="submit"
        class="px-3 py-1.5 text-sm bg-emerald-600 text-white rounded-lg hover:bg-emerald-700"
      >
        Add Item
      </button>
    </div>
  </form>
{:else}
  <button
    type="button"
    onclick={() => (isExpanded = true)}
    class="w-full py-2 text-sm text-emerald-600 hover:text-emerald-700 hover:bg-emerald-50 rounded-lg transition-colors"
  >
    + Add item
  </button>
{/if}
