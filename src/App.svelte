<script lang="ts">
  import TabNavigation from "$lib/components/TabNavigation.svelte";
  import { activeTab } from "$lib/stores";

  // Placeholder components - will be replaced in later phases
  import Dashboard from "$lib/components/Dashboard.svelte";
  import Recipes from "$lib/components/Recipes.svelte";
  import MealPlan from "$lib/components/MealPlan.svelte";
  import QuickListsManager from "$lib/components/QuickListsManager.svelte";
</script>

<div class="min-h-screen bg-gray-50 flex flex-col">
  <header class="bg-white shadow-sm sticky top-0 z-40">
    <div class="px-6 py-4">
      <h1 class="text-2xl font-bold text-emerald-600">feast</h1>
    </div>
    <TabNavigation />
  </header>

  <main class="flex-1 p-6">
    {#if $activeTab === "dashboard"}
      <Dashboard />
    {:else if $activeTab === "recipes"}
      <Recipes />
    {:else if $activeTab === "mealplan"}
      <MealPlan />
    {:else if $activeTab === "quicklists"}
      <QuickListsManager />
    {/if}
  </main>
</div>
