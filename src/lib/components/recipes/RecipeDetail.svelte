<script lang="ts">
  import type { Recipe } from "$lib/types";

  interface Props {
    recipe: Recipe;
    onBack: () => void;
  }

  let { recipe, onBack }: Props = $props();
  let servingMultiplier = $state(1);
  let adjustedServings = $derived(recipe.servings * servingMultiplier);
</script>

<div class="max-w-4xl mx-auto">
  <button
    type="button"
    onclick={onBack}
    class="flex items-center gap-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 px-3 py-2 -ml-3 rounded-lg mb-6 transition-colors"
  >
    <span aria-hidden="true">&larr;</span> Back to recipes
  </button>

  <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
    {#if recipe.imageUrl}
      <div class="h-64 bg-gray-200">
        <img src={recipe.imageUrl} alt={recipe.name} class="w-full h-full object-cover" />
      </div>
    {:else}
      <div class="h-64 bg-gradient-to-br from-emerald-100 to-emerald-200 flex items-center justify-center">
        <span class="text-6xl">🍽️</span>
      </div>
    {/if}

    <div class="p-6">
      <h1 class="text-2xl font-bold text-gray-800 mb-2">{recipe.name}</h1>
      <p class="text-gray-600 mb-6">{recipe.description}</p>

      <div class="flex flex-wrap gap-4 mb-6 text-sm">
        <div class="flex items-center gap-2 px-3 py-2 bg-gray-100 rounded-lg">
          <span>⏱️</span>
          <span>Prep: {recipe.prepTime} min</span>
        </div>
        <div class="flex items-center gap-2 px-3 py-2 bg-gray-100 rounded-lg">
          <span>🍳</span>
          <span>Cook: {recipe.cookTime} min</span>
        </div>
        <div class="flex items-center gap-2 px-3 py-2 bg-gray-100 rounded-lg">
          <span>👥</span>
          <div class="flex items-center gap-2">
            <button
              type="button"
              onclick={() => servingMultiplier = Math.max(0.5, servingMultiplier - 0.5)}
              class="w-6 h-6 rounded-lg bg-gray-200 hover:bg-gray-300 transition-colors flex items-center justify-center font-medium"
              aria-label="Decrease servings"
            >
              -
            </button>
            <span>{adjustedServings} servings</span>
            <button
              type="button"
              onclick={() => servingMultiplier += 0.5}
              class="w-6 h-6 rounded-lg bg-gray-200 hover:bg-gray-300 transition-colors flex items-center justify-center font-medium"
              aria-label="Increase servings"
            >
              +
            </button>
          </div>
        </div>
      </div>

      {#if recipe.tags.length > 0}
        <div class="flex flex-wrap gap-2 mb-6">
          {#each recipe.tags as tag}
            <span class="px-3 py-1 text-sm rounded-full bg-emerald-100 text-emerald-700">
              {tag}
            </span>
          {/each}
        </div>
      {/if}

      <div class="grid md:grid-cols-2 gap-8">
        <div>
          <h2 class="text-lg font-semibold text-gray-800 mb-4">Ingredients</h2>
          <ul class="space-y-2">
            {#each recipe.ingredients as ing}
              <li class="flex items-start gap-2">
                <span class="text-emerald-500 mt-0.5">•</span>
                <span>
                  {(ing.quantity * servingMultiplier).toFixed(ing.quantity * servingMultiplier % 1 === 0 ? 0 : 1)}
                  {ing.unit} {ing.name}
                  {#if ing.notes}
                    <span class="text-gray-500">({ing.notes})</span>
                  {/if}
                </span>
              </li>
            {/each}
          </ul>
        </div>

        <div>
          <h2 class="text-lg font-semibold text-gray-800 mb-4">Instructions</h2>
          <ol class="space-y-3">
            {#each recipe.instructions as step, i}
              <li class="flex gap-3">
                <span class="flex-shrink-0 w-6 h-6 rounded-full bg-emerald-100 text-emerald-700 flex items-center justify-center text-sm font-medium">
                  {i + 1}
                </span>
                <span>{step}</span>
              </li>
            {/each}
          </ol>
        </div>
      </div>

      {#if recipe.nutrition}
        <div class="mt-8 p-4 bg-gray-50 rounded-lg">
          <h2 class="text-lg font-semibold text-gray-800 mb-3">Nutrition (per serving)</h2>
          <div class="grid grid-cols-4 gap-4 text-center">
            <div>
              <div class="text-xl font-bold text-gray-800">{recipe.nutrition.calories}</div>
              <div class="text-sm text-gray-500">Calories</div>
            </div>
            <div>
              <div class="text-xl font-bold text-gray-800">{recipe.nutrition.protein}g</div>
              <div class="text-sm text-gray-500">Protein</div>
            </div>
            <div>
              <div class="text-xl font-bold text-gray-800">{recipe.nutrition.carbs}g</div>
              <div class="text-sm text-gray-500">Carbs</div>
            </div>
            <div>
              <div class="text-xl font-bold text-gray-800">{recipe.nutrition.fat}g</div>
              <div class="text-sm text-gray-500">Fat</div>
            </div>
          </div>
        </div>
      {/if}

      {#if recipe.notes}
        <div class="mt-6 p-4 bg-yellow-50 rounded-lg border border-yellow-100">
          <h2 class="font-semibold text-yellow-800 mb-2">Notes</h2>
          <p class="text-yellow-700">{recipe.notes}</p>
        </div>
      {/if}

      {#if recipe.sourceUrl}
        <div class="mt-6">
          <a
            href={recipe.sourceUrl}
            target="_blank"
            rel="noopener noreferrer"
            class="text-emerald-600 hover:text-emerald-700 underline"
          >
            View original recipe &rarr;
          </a>
        </div>
      {/if}
    </div>
  </div>
</div>
