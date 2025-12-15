<script lang="ts">
  import { quickListsStore } from "$lib/stores";
  import QuickListCard from "./quicklists/QuickListCard.svelte";
  import AddQuickListModal from "./quicklists/AddQuickListModal.svelte";

  let isModalOpen = $state(false);

  function handleAddList(name: string) {
    quickListsStore.addList(name);
  }
</script>

<div class="max-w-[1800px] 3xl:max-w-[2400px] mx-auto px-2 sm:px-4 2xl:px-6">
  <div class="flex items-center justify-between mb-6">
    <h2 class="text-xl sm:text-2xl font-bold text-gray-800">Quick Lists</h2>
    <button
      type="button"
      onclick={() => (isModalOpen = true)}
      class="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors flex items-center gap-2"
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
      </svg>
      New List
    </button>
  </div>

  {#if $quickListsStore.length === 0}
    <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-8 text-center">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-12 h-12 mx-auto text-gray-300 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
      </svg>
      <h3 class="text-lg font-medium text-gray-700 mb-2">No Quick Lists Yet</h3>
      <p class="text-gray-500 mb-4">Create a quick list to easily add common items to your shopping list.</p>
      <button
        type="button"
        onclick={() => (isModalOpen = true)}
        class="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
      >
        Create Your First List
      </button>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
      {#each $quickListsStore as list (list.id)}
        <QuickListCard {list} />
      {/each}
    </div>
  {/if}
</div>

<AddQuickListModal
  isOpen={isModalOpen}
  onClose={() => (isModalOpen = false)}
  onAdd={handleAddList}
/>
