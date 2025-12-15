<script lang="ts">
  import { GROCERY_CATEGORIES } from "$lib/types";

  interface Props {
    onAdd: (name: string, quantity: number, unit: string, category: string) => void;
  }

  let { onAdd }: Props = $props();

  let name = $state("");
  let quantity = $state(1);
  let unit = $state("item");
  let category = $state("Other");
  let isExpanded = $state(false);

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!name.trim()) return;

    onAdd(name.trim(), quantity, unit, category);
    name = "";
    quantity = 1;
    unit = "item";
    category = "Other";
    isExpanded = false;
  }
</script>

{#if !isExpanded}
  <button
    type="button"
    onclick={() => isExpanded = true}
    class="w-full py-2 text-sm text-gray-500 hover:text-emerald-600 hover:bg-gray-50 rounded-lg border border-dashed border-gray-300 transition-colors"
  >
    + Add item manually
  </button>
{:else}
  <form onsubmit={handleSubmit} class="p-4 bg-gray-50 rounded-lg">
    <div class="flex gap-2 mb-2">
      <input
        type="text"
        bind:value={name}
        placeholder="Item name"
        class="flex-1 px-3 py-2 border border-gray-300 rounded-lg text-sm"
      />
      <input
        type="number"
        bind:value={quantity}
        min="0.1"
        step="0.1"
        class="w-20 px-3 py-2 border border-gray-300 rounded-lg text-sm"
      />
      <input
        type="text"
        bind:value={unit}
        placeholder="unit"
        class="w-24 px-3 py-2 border border-gray-300 rounded-lg text-sm"
      />
    </div>
    <div class="flex gap-2">
      <select
        bind:value={category}
        class="flex-1 px-3 py-2 border border-gray-300 rounded-lg text-sm"
      >
        {#each GROCERY_CATEGORIES as cat}
          <option value={cat}>{cat}</option>
        {/each}
      </select>
      <button
        type="submit"
        class="px-4 py-2 bg-emerald-600 text-white rounded-lg text-sm hover:bg-emerald-700"
      >
        Add
      </button>
      <button
        type="button"
        onclick={() => isExpanded = false}
        class="px-4 py-2 text-gray-500 hover:text-gray-700 text-sm"
      >
        Cancel
      </button>
    </div>
  </form>
{/if}
