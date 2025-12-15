<script lang="ts">
  import { recipeStore, activeTab } from "$lib/stores";
  import { derived } from "svelte/store";

  const recentRecipes = derived(recipeStore, ($recipes) =>
    [...$recipes]
      .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
      .slice(0, 3)
  );
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
  <div class="px-6 py-4 border-b border-gray-100 flex justify-between items-center">
    <h2 class="text-lg font-semibold text-gray-800">Recent Recipes</h2>
    <button
      type="button"
      onclick={() => activeTab.set("recipes")}
      class="text-sm text-emerald-600 hover:text-emerald-700"
    >
      View all
    </button>
  </div>

  <div class="divide-y divide-gray-100">
    {#each $recentRecipes as recipe}
      <div class="px-6 py-3 hover:bg-gray-50 transition-colors">
        <div class="font-medium text-gray-800">{recipe.name}</div>
        <div class="text-sm text-gray-500">
          {recipe.prepTime + recipe.cookTime} min · {recipe.servings} servings
        </div>
      </div>
    {/each}
  </div>
</div>
