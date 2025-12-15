<script lang="ts">
  import type { Recipe } from "$lib/types";
  import AddToMealPlanModal from "./AddToMealPlanModal.svelte";

  interface Props {
    recipe: Recipe;
    onClose: () => void;
    onEdit: () => void;
  }

  let { recipe, onClose, onEdit }: Props = $props();
  let servingMultiplier = $state(1);
  let adjustedServings = $derived(recipe.servings * servingMultiplier);
  let showMealPlanModal = $state(false);

  // Reset multiplier when recipe changes
  $effect(() => {
    recipe; // dependency
    servingMultiplier = 1;
  });
</script>

<div class="h-full flex flex-col">
  <!-- Header -->
  <div class="flex items-center justify-between px-4 py-3 border-b border-gray-100">
    <h2 class="font-semibold text-gray-800 truncate">{recipe.name}</h2>
    <button
      type="button"
      onclick={onClose}
      class="w-8 h-8 flex items-center justify-center rounded-lg text-gray-400 hover:text-gray-600 hover:bg-gray-100"
      aria-label="Close panel"
    >
      x
    </button>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-auto p-4">
    <!-- Image -->
    {#if recipe.imageUrl}
      <div class="h-48 bg-gray-200 rounded-lg mb-4 overflow-hidden">
        <img src={recipe.imageUrl} alt={recipe.name} class="w-full h-full object-cover" />
      </div>
    {:else}
      <div class="h-48 bg-gradient-to-br from-emerald-100 to-emerald-200 rounded-lg mb-4 flex items-center justify-center">
        <span class="text-5xl">🍽️</span>
      </div>
    {/if}

    <p class="text-gray-600 text-sm mb-4">{recipe.description}</p>

    <!-- Meta -->
    <div class="flex flex-wrap gap-2 mb-4 text-sm">
      <span class="px-2 py-1 bg-gray-100 rounded">⏱️ {recipe.prepTime + recipe.cookTime} min</span>
      <div class="flex items-center gap-1 px-2 py-1 bg-gray-100 rounded">
        <span>👥</span>
        <button
          type="button"
          onclick={() => servingMultiplier = Math.max(0.5, servingMultiplier - 0.5)}
          class="w-5 h-5 rounded bg-gray-200 hover:bg-gray-300 text-xs"
        >-</button>
        <span>{adjustedServings}</span>
        <button
          type="button"
          onclick={() => servingMultiplier += 0.5}
          class="w-5 h-5 rounded bg-gray-200 hover:bg-gray-300 text-xs"
        >+</button>
      </div>
    </div>

    <!-- Tags -->
    {#if recipe.tags.length > 0}
      <div class="flex flex-wrap gap-1 mb-4">
        {#each recipe.tags as tag}
          <span class="px-2 py-0.5 text-xs rounded-full bg-emerald-100 text-emerald-700">{tag}</span>
        {/each}
      </div>
    {/if}

    <!-- Ingredients -->
    <div class="mb-4">
      <h3 class="font-semibold text-gray-800 mb-2">Ingredients</h3>
      <ul class="space-y-1 text-sm">
        {#each recipe.ingredients as ing}
          <li class="flex gap-2">
            <span class="text-emerald-500">•</span>
            <span>
              {(ing.quantity * servingMultiplier).toFixed(ing.quantity * servingMultiplier % 1 === 0 ? 0 : 1)}
              {ing.unit} {ing.name}
              {#if ing.notes}<span class="text-gray-400">({ing.notes})</span>{/if}
            </span>
          </li>
        {/each}
      </ul>
    </div>

    <!-- Instructions -->
    <div class="mb-4">
      <h3 class="font-semibold text-gray-800 mb-2">Instructions</h3>
      <ol class="space-y-2 text-sm">
        {#each recipe.instructions as step, i}
          <li class="flex gap-2">
            <span class="flex-shrink-0 w-5 h-5 rounded-full bg-emerald-100 text-emerald-700 flex items-center justify-center text-xs">{i + 1}</span>
            <span>{step}</span>
          </li>
        {/each}
      </ol>
    </div>

    <!-- Nutrition -->
    {#if recipe.nutrition}
      <div class="p-3 bg-gray-50 rounded-lg text-sm">
        <h3 class="font-semibold text-gray-800 mb-2">Nutrition (per serving)</h3>
        <div class="grid grid-cols-4 gap-2 text-center">
          <div>
            <div class="font-bold">{recipe.nutrition.calories}</div>
            <div class="text-xs text-gray-500">Cal</div>
          </div>
          <div>
            <div class="font-bold">{recipe.nutrition.protein}g</div>
            <div class="text-xs text-gray-500">Protein</div>
          </div>
          <div>
            <div class="font-bold">{recipe.nutrition.carbs}g</div>
            <div class="text-xs text-gray-500">Carbs</div>
          </div>
          <div>
            <div class="font-bold">{recipe.nutrition.fat}g</div>
            <div class="text-xs text-gray-500">Fat</div>
          </div>
        </div>
      </div>
    {/if}
  </div>

  <!-- Actions Footer -->
  <div class="flex gap-2 p-4 border-t border-gray-100">
    <button
      type="button"
      onclick={onEdit}
      class="flex-1 px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
    >
      Edit
    </button>
    <button
      type="button"
      onclick={() => showMealPlanModal = true}
      class="flex-1 px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
    >
      Add to Meal Plan
    </button>
  </div>
</div>

<AddToMealPlanModal
  isOpen={showMealPlanModal}
  {recipe}
  onClose={() => showMealPlanModal = false}
/>
