<script lang="ts">
  import { activeTab, type TabId } from "$lib/stores";

  const tabs: { id: TabId; label: string }[] = [
    { id: "dashboard", label: "Dashboard" },
    { id: "recipes", label: "Recipes" },
    { id: "mealplan", label: "Meal Plan" },
    { id: "quicklists", label: "Quick Lists" },
  ];
</script>

<nav class="bg-white border-b border-gray-200" aria-label="Main navigation">
  <div class="flex space-x-1 px-4">
    {#each tabs as tab}
      <button
        type="button"
        onclick={() => activeTab.set(tab.id)}
        class="px-4 py-3 text-sm font-medium transition-colors relative
          {$activeTab === tab.id
            ? 'text-emerald-600'
            : 'text-gray-500 hover:text-gray-700 hover:bg-gray-50'}"
        aria-current={$activeTab === tab.id ? "page" : undefined}
      >
        {tab.label}
        {#if $activeTab === tab.id}
          <span class="absolute bottom-0 left-0 right-0 h-0.5 bg-emerald-600 transition-all"></span>
        {/if}
      </button>
    {/each}
  </div>
</nav>
