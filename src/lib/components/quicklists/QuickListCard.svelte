<script lang="ts">
  import type { QuickList } from "$lib/types";
  import { quickListsStore } from "$lib/stores";
  import QuickListItemRow from "./QuickListItemRow.svelte";
  import AddItemForm from "./AddItemForm.svelte";

  interface Props {
    list: QuickList;
  }

  let { list }: Props = $props();

  let isExpanded = $state(false);
  let isEditingName = $state(false);
  let editedName = $state(list.name);

  function toggleExpand() {
    isExpanded = !isExpanded;
  }

  function startEditName() {
    editedName = list.name;
    isEditingName = true;
  }

  function saveName() {
    if (editedName.trim()) {
      quickListsStore.renameList(list.id, editedName.trim());
    }
    isEditingName = false;
  }

  function handleNameKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      saveName();
    } else if (e.key === "Escape") {
      isEditingName = false;
    }
  }

  function deleteList() {
    quickListsStore.removeList(list.id);
  }
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
  <div class="flex items-center justify-between px-4 py-3 border-b border-gray-100">
    <div class="flex items-center gap-2 flex-1">
      {#if isEditingName}
        <input
          type="text"
          bind:value={editedName}
          onblur={saveName}
          onkeydown={handleNameKeydown}
          class="flex-1 px-2 py-1 text-sm font-semibold border border-emerald-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-emerald-500"
          autofocus
        />
      {:else}
        <button
          type="button"
          onclick={startEditName}
          class="font-semibold text-gray-800 hover:text-emerald-600 text-left"
        >
          {list.name}
        </button>
        <span class="text-gray-400 text-sm">({list.items.length} items)</span>
      {/if}
    </div>
    <div class="flex items-center gap-2">
      <button
        type="button"
        onclick={deleteList}
        class="p-1 text-gray-400 hover:text-red-600 rounded transition-colors"
        aria-label="Delete list"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
        </svg>
      </button>
      <button
        type="button"
        onclick={toggleExpand}
        class="p-1 text-gray-400 hover:text-gray-600 rounded transition-colors"
        aria-label={isExpanded ? "Collapse" : "Expand"}
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-5 h-5 transition-transform {isExpanded ? 'rotate-180' : ''}"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
      </button>
    </div>
  </div>

  {#if isExpanded}
    <div class="p-4">
      {#if list.items.length === 0}
        <p class="text-sm text-gray-500 text-center py-4">No items yet. Add your first item below.</p>
      {:else}
        <div class="space-y-1 mb-4">
          {#each list.items as item (item.id)}
            <QuickListItemRow
              {item}
              onUpdate={(updates) => quickListsStore.updateItem(list.id, item.id, updates)}
              onRemove={() => quickListsStore.removeItem(list.id, item.id)}
            />
          {/each}
        </div>
      {/if}
      <AddItemForm onAdd={(item) => quickListsStore.addItem(list.id, item)} />
    </div>
  {/if}
</div>
