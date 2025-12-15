<script lang="ts">
  import { quickListsStore, activeTab } from "$lib/stores";
  import type { QuickListItem } from "$lib/types";

  interface Props {
    onAddItems: (items: QuickListItem[]) => void;
  }

  let { onAddItems }: Props = $props();

  let isOpen = $state(false);
  let expandedList = $state<string | null>(null);
  let dropdownRef = $state<HTMLDivElement | null>(null);

  function toggleDropdown() {
    isOpen = !isOpen;
    if (!isOpen) {
      expandedList = null;
    }
  }

  function closeDropdown() {
    isOpen = false;
    expandedList = null;
  }

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

  function handleClickOutside(event: MouseEvent) {
    if (dropdownRef && !dropdownRef.contains(event.target as Node)) {
      closeDropdown();
    }
  }

  $effect(() => {
    if (isOpen) {
      document.addEventListener("click", handleClickOutside, true);
      return () => {
        document.removeEventListener("click", handleClickOutside, true);
      };
    }
  });
</script>

<div class="relative" bind:this={dropdownRef}>
  <button
    type="button"
    onclick={toggleDropdown}
    class="py-2 px-4 text-sm text-gray-500 hover:text-emerald-600 hover:bg-gray-50 rounded-lg border border-dashed border-gray-300 transition-colors flex items-center gap-2"
  >
    <span>+ From Quick List</span>
    <span class="text-xs">{isOpen ? '\u25B2' : '\u25BC'}</span>
  </button>

  {#if isOpen}
    <div class="absolute top-full right-0 mt-1 w-72 max-w-sm bg-white rounded-xl shadow-lg border border-gray-200 z-50 overflow-hidden">
      <div class="px-4 py-3 border-b border-gray-100 flex items-center justify-between bg-gray-50">
        <h3 class="font-semibold text-gray-800 text-sm">Quick Lists</h3>
        <button
          type="button"
          onclick={() => { closeDropdown(); activeTab.set("quicklists"); }}
          class="text-xs text-emerald-600 hover:text-emerald-700"
        >
          Manage
        </button>
      </div>

      <div class="divide-y divide-gray-100 max-h-96 overflow-y-auto">
        {#if $quickListsStore.length === 0}
          <div class="px-4 py-6 text-center text-sm text-gray-500">
            No quick lists yet.
            <button
              type="button"
              onclick={() => { closeDropdown(); activeTab.set("quicklists"); }}
              class="text-emerald-600 hover:text-emerald-700 block mx-auto mt-1"
            >
              Create one
            </button>
          </div>
        {:else}
          {#each $quickListsStore as list (list.id)}
            <div>
              <button
                type="button"
                onclick={() => toggleList(list.id)}
                class="w-full flex items-center justify-between px-4 py-2.5 hover:bg-gray-50 transition-colors text-sm"
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
                        <span class="text-gray-600 truncate flex-1 mr-2">
                          {item.quantity} {item.unit} {item.name}
                        </span>
                        <button
                          type="button"
                          onclick={() => addSingleItem(item)}
                          class="text-emerald-600 hover:text-emerald-700 flex-shrink-0 font-medium"
                        >
                          + Add
                        </button>
                      </div>
                    {/each}
                  </div>
                  <button
                    type="button"
                    onclick={() => addAllItems(list.id)}
                    class="w-full py-1.5 text-xs bg-emerald-600 text-white rounded hover:bg-emerald-700 font-medium"
                  >
                    Add all ({list.items.length})
                  </button>
                </div>
              {/if}
            </div>
          {/each}
        {/if}
      </div>
    </div>
  {/if}
</div>
