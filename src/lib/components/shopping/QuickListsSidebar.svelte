<script lang="ts">
  import { quickListsStore, activeTab } from "$lib/stores";
  import type { QuickListItem } from "$lib/types";

  interface Props {
    collapsed: boolean;
    onToggle: () => void;
    onAddItems: (items: QuickListItem[]) => void;
  }

  let { collapsed, onToggle, onAddItems }: Props = $props();

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

<div
  class="flex-shrink-0 transition-all duration-300 ease-in-out
    {collapsed ? 'w-10' : 'w-64 2xl:w-80'}"
>
  {#if collapsed}
    <!-- Collapsed state - just a toggle button -->
    <button
      type="button"
      onclick={onToggle}
      class="w-10 h-10 flex items-center justify-center bg-white rounded-lg shadow-sm border border-gray-200 hover:bg-gray-50"
      title="Expand Quick Lists"
    >
      {'\u25B6'}
    </button>
  {:else}
    <!-- Expanded state -->
    <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
      <div class="px-4 py-3 border-b border-gray-100 flex items-center justify-between">
        <h3 class="font-semibold text-gray-800">Quick Lists</h3>
        <div class="flex items-center gap-2">
          <button
            type="button"
            onclick={() => activeTab.set("quicklists")}
            class="text-xs text-emerald-600 hover:text-emerald-700"
          >
            Manage
          </button>
          <button
            type="button"
            onclick={onToggle}
            class="text-gray-400 hover:text-gray-600"
            title="Collapse"
          >
            {'\u25C0'}
          </button>
        </div>
      </div>

      <div class="divide-y divide-gray-100 max-h-[400px] overflow-y-auto">
        {#each $quickListsStore as list (list.id)}
          <div>
            <button
              type="button"
              onclick={() => toggleList(list.id)}
              class="w-full flex items-center justify-between px-4 py-2 hover:bg-gray-50 transition-colors text-sm"
            >
              <span class="font-medium text-gray-700">{list.name}</span>
              <span class="text-gray-400 text-xs">
                {list.items.length}
                <span class="ml-1">{expandedList === list.id ? '\u25B2' : '\u25BC'}</span>
              </span>
            </button>

            {#if expandedList === list.id}
              <div class="px-3 pb-2 bg-gray-50">
                <div class="space-y-1 mb-2">
                  {#each list.items as item (item.id)}
                    <div class="flex items-center justify-between py-1 text-xs">
                      <span class="text-gray-600 truncate">
                        {item.quantity} {item.unit} {item.name}
                      </span>
                      <button
                        type="button"
                        onclick={() => addSingleItem(item)}
                        class="text-emerald-600 hover:text-emerald-700 flex-shrink-0"
                      >
                        +
                      </button>
                    </div>
                  {/each}
                </div>
                <button
                  type="button"
                  onclick={() => addAllItems(list.id)}
                  class="w-full py-1.5 text-xs bg-emerald-600 text-white rounded hover:bg-emerald-700"
                >
                  Add all
                </button>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>
