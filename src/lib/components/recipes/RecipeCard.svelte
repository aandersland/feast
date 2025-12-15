<script lang="ts">
  import type { Recipe } from "$lib/types";

  interface Props {
    recipe: Recipe;
    onSelect: (recipe: Recipe) => void;
    isSelected?: boolean;
  }

  let { recipe, onSelect, isSelected = false }: Props = $props();
</script>

<button
  type="button"
  onclick={() => onSelect(recipe)}
  class="w-full text-left bg-white rounded-xl shadow-sm border overflow-hidden hover:shadow-md transition-shadow
    {isSelected ? 'border-emerald-500 ring-2 ring-emerald-200' : 'border-gray-100'}"
>
  {#if recipe.imageUrl}
    <div class="h-40 bg-gray-200">
      <img src={recipe.imageUrl} alt={recipe.name} class="w-full h-full object-cover" />
    </div>
  {:else}
    <div class="h-40 bg-gradient-to-br from-emerald-100 to-emerald-200 flex items-center justify-center">
      <span class="text-4xl">🍽️</span>
    </div>
  {/if}

  <div class="p-4">
    <h3 class="font-semibold text-gray-800 mb-1">{recipe.name}</h3>
    <p class="text-sm text-gray-500 line-clamp-2 mb-3">{recipe.description}</p>

    <div class="flex items-center gap-4 text-sm text-gray-500">
      <span>{recipe.prepTime + recipe.cookTime} min</span>
      <span>{recipe.servings} servings</span>
    </div>

    {#if recipe.tags.length > 0}
      <div class="flex flex-wrap gap-1 mt-3">
        {#each recipe.tags.slice(0, 3) as tag}
          <span class="px-2 py-0.5 text-xs rounded-full bg-gray-100 text-gray-600">
            {tag}
          </span>
        {/each}
      </div>
    {/if}
  </div>
</button>
