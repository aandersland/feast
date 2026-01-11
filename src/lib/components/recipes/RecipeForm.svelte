<script lang="ts">
  import type { Recipe, Ingredient } from "$lib/types";

  interface Props {
    recipe?: Recipe | null;
    onSave: (recipe: Omit<Recipe, "id" | "createdAt">) => void;
    onCancel: () => void;
  }

  let { recipe = null, onSave, onCancel }: Props = $props();

  // Initialize with defaults - $effect will sync from recipe prop
  let name = $state("");
  let description = $state("");
  let prepTime = $state(15);
  let cookTime = $state(30);
  let servings = $state(4);
  let ingredients = $state<Omit<Ingredient, "id">[]>([{ name: "", quantity: 1, unit: "" }]);
  let instructions = $state<string[]>([""]);
  let tags = $state("");
  let notes = $state("");
  let sourceUrl = $state("");

  // Track recipe ID to detect when we're editing a different recipe
  let currentRecipeId = $state<string | null>(null);

  $effect(() => {
    const recipeId = recipe?.id ?? null;
    // Reset form when recipe changes (including initial mount with recipe)
    if (recipeId !== currentRecipeId) {
      currentRecipeId = recipeId;
      if (recipe) {
        name = recipe.name;
        description = recipe.description;
        prepTime = recipe.prepTime;
        cookTime = recipe.cookTime;
        servings = recipe.servings;
        ingredients = recipe.ingredients.map(({ id, ...rest }) => rest);
        instructions = [...recipe.instructions];
        tags = recipe.tags.join(", ");
        notes = recipe.notes ?? "";
        sourceUrl = recipe.sourceUrl ?? "";
      } else {
        // Reset to defaults for new recipe
        name = "";
        description = "";
        prepTime = 15;
        cookTime = 30;
        servings = 4;
        ingredients = [{ name: "", quantity: 1, unit: "" }];
        instructions = [""];
        tags = "";
        notes = "";
        sourceUrl = "";
      }
    }
  });

  function addIngredient() {
    ingredients = [...ingredients, { name: "", quantity: 1, unit: "" }];
  }

  function removeIngredient(index: number) {
    ingredients = ingredients.filter((_, i) => i !== index);
  }

  function addInstruction() {
    instructions = [...instructions, ""];
  }

  function removeInstruction(index: number) {
    instructions = instructions.filter((_, i) => i !== index);
  }

  function handleSubmit() {
    onSave({
      name,
      description,
      prepTime,
      cookTime,
      servings,
      ingredients: ingredients
        .filter((i) => i.name.trim())
        .map((i, idx) => ({ ...i, id: String(idx) })),
      instructions: instructions.filter((i) => i.trim()),
      tags: tags.split(",").map((t) => t.trim()).filter(Boolean),
      notes: notes || undefined,
      sourceUrl: sourceUrl || undefined,
    });
  }
</script>

<div>
  <h2 class="text-xl font-bold text-gray-800 mb-6">{recipe ? "Edit Recipe" : "Add New Recipe"}</h2>

  <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-6">
    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">
        Recipe Name
        <input
          type="text"
          bind:value={name}
          required
          class="mt-1 w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
        />
      </label>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">
        Description
        <textarea
          bind:value={description}
          rows="2"
          class="mt-1 w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
        ></textarea>
      </label>
    </div>

    <div class="grid grid-cols-3 gap-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">
          Prep Time (min)
          <input
            type="number"
            bind:value={prepTime}
            min="0"
            class="mt-1 w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
          />
        </label>
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">
          Cook Time (min)
          <input
            type="number"
            bind:value={cookTime}
            min="0"
            class="mt-1 w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
          />
        </label>
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">
          Servings
          <input
            type="number"
            bind:value={servings}
            min="1"
            class="mt-1 w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
          />
        </label>
      </div>
    </div>

    <div>
      <span class="block text-sm font-medium text-gray-700 mb-2">Ingredients</span>
      <div class="space-y-2">
        {#each ingredients as ing, i}
          <div class="flex gap-2">
            <input
              type="number"
              bind:value={ing.quantity}
              min="0"
              step="0.25"
              class="w-20 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
              placeholder="Qty"
            />
            <input
              type="text"
              bind:value={ing.unit}
              class="w-24 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
              placeholder="Unit"
            />
            <input
              type="text"
              bind:value={ing.name}
              class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
              placeholder="Ingredient name"
            />
            <button
              type="button"
              onclick={() => removeIngredient(i)}
              class="px-3 py-2 text-red-500 hover:bg-red-50 rounded-lg transition-colors"
              aria-label="Remove ingredient"
            >
              x
            </button>
          </div>
        {/each}
      </div>
      <button
        type="button"
        onclick={addIngredient}
        class="mt-2 text-sm text-emerald-600 hover:text-emerald-700"
      >
        + Add ingredient
      </button>
    </div>

    <div>
      <span class="block text-sm font-medium text-gray-700 mb-2">Instructions</span>
      <div class="space-y-2">
        {#each instructions as step, i}
          <div class="flex gap-2">
            <span class="flex-shrink-0 w-8 h-10 flex items-center justify-center text-gray-500">
              {i + 1}.
            </span>
            <input
              type="text"
              bind:value={instructions[i]}
              class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
              placeholder="Step description"
            />
            <button
              type="button"
              onclick={() => removeInstruction(i)}
              class="px-3 py-2 text-red-500 hover:bg-red-50 rounded-lg transition-colors"
              aria-label="Remove instruction"
            >
              x
            </button>
          </div>
        {/each}
      </div>
      <button
        type="button"
        onclick={addInstruction}
        class="mt-2 text-sm text-emerald-600 hover:text-emerald-700"
      >
        + Add step
      </button>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">
        Tags (comma-separated)
        <input
          type="text"
          bind:value={tags}
          placeholder="Italian, Pasta, Quick"
          class="mt-1 w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
        />
      </label>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">
        Source URL (optional)
        <input
          type="url"
          bind:value={sourceUrl}
          placeholder="https://..."
          class="mt-1 w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
        />
      </label>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">
        Notes (optional)
        <textarea
          bind:value={notes}
          rows="2"
          class="mt-1 w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
        ></textarea>
      </label>
    </div>

    <div class="flex justify-end gap-3 pt-4 border-t">
      <button
        type="button"
        onclick={onCancel}
        class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
      >
        Cancel
      </button>
      <button
        type="submit"
        class="px-6 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
      >
        Save Recipe
      </button>
    </div>
  </form>
</div>
