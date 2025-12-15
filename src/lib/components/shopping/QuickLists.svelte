<script lang="ts">
  import { quickListsStore, activeTab } from "$lib/stores";
  import type { QuickListItem } from "$lib/types";

  interface Props {
    onAddItems: (items: QuickListItem[]) => void;
  }

  let { onAddItems }: Props = $props();

  let expandedList = $state<string | null>(null);

  function toggleList(id: string) {
    expandedList = expandedList === id ? null : id;
  }

  function addSingleItem(item: QuickListItem) {
    onAddItems([item]);
  }

  function addAllItems(listId: string) {
    const list = $quickListsStore.find((l) => l.id === listId);
    if (list) {
      onAddItems(list.items);
    }
  }
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
  <div class="px-4 py-3 border-b border-gray-100 flex items-center justify-between">
    <h3 class="font-semibold text-gray-800">Quick Lists</h3>
    <button
      type="button"
      onclick={() => activeTab.set("quicklists")}
      class="text-sm text-emerald-600 hover:text-emerald-700"
    >
      Manage lists
    </button>
  </div>

  <div class="divide-y divide-gray-100">
    {#each $quickListsStore as list}
      <div>
        <button
          type="button"
          onclick={() => toggleList(list.id)}
          class="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-50 transition-colors"
        >
          <span class="font-medium text-gray-700">{list.name}</span>
          <span class="text-gray-400 text-sm">
            {list.items.length} items
            <span class="ml-2">{expandedList === list.id ? '▲' : '▼'}</span>
          </span>
        </button>

        {#if expandedList === list.id}
          <div class="px-4 pb-3 bg-gray-50">
            <div class="space-y-1 mb-3">
              {#each list.items as item}
                <div class="flex items-center justify-between py-1 text-sm">
                  <span class="text-gray-600">
                    {item.quantity} {item.unit} {item.name}
                  </span>
                  <button
                    type="button"
                    onclick={() => addSingleItem(item)}
                    class="text-emerald-600 hover:text-emerald-700"
                  >
                    + Add
                  </button>
                </div>
              {/each}
            </div>
            <button
              type="button"
              onclick={() => addAllItems(list.id)}
              class="w-full py-2 text-sm bg-emerald-600 text-white rounded-lg hover:bg-emerald-700"
            >
              Add all items
            </button>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>
