<script lang="ts">
  import type { ShoppingList } from "$lib/types";

  interface Props {
    lists: ShoppingList[];
    activeListId: string | null;
    onSelectList: (listId: string) => void;
    onAddList: () => void;
  }

  let { lists, activeListId, onSelectList, onAddList }: Props = $props();
</script>

<div class="flex items-center gap-1 border-b border-gray-200">
  {#each lists as list (list.id)}
    <button
      type="button"
      onclick={() => onSelectList(list.id)}
      class="px-4 py-2 text-sm font-medium transition-colors relative
        {activeListId === list.id
          ? 'text-emerald-600'
          : 'text-gray-500 hover:text-gray-700 hover:bg-gray-50'}"
    >
      {list.name}
      {#if list.items.length > 0}
        <span class="ml-1 text-xs text-gray-400">({list.items.filter(i => !i.isOnHand).length})</span>
      {/if}
      {#if activeListId === list.id}
        <span class="absolute bottom-0 left-0 right-0 h-0.5 bg-emerald-600"></span>
      {/if}
    </button>
  {/each}

  <button
    type="button"
    onclick={onAddList}
    class="px-3 py-2 text-sm text-gray-400 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
    title="Add new list"
  >
    + New List
  </button>
</div>
