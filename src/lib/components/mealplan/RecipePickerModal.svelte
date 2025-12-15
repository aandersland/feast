<script lang="ts">
  import Modal from "../shared/Modal.svelte";
  import { recipeStore } from "$lib/stores";
  import type { Recipe, MealType } from "$lib/types";

  interface Props {
    isOpen: boolean;
    date: string;
    onClose: () => void;
    onSelect: (recipeId: string, mealType: MealType, servings: number) => void;
  }

  let { isOpen, date, onClose, onSelect }: Props = $props();

  let searchQuery = $state("");
  let selectedRecipe = $state<Recipe | null>(null);
  let mealType = $state<MealType>("dinner");
  let servings = $state(4);

  let filteredRecipes = $derived(
    $recipeStore.filter((r) =>
      r.name.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  function handleConfirm() {
    if (!selectedRecipe) return;
    onSelect(selectedRecipe.id, mealType, servings);
    resetForm();
    onClose();
  }

  function resetForm() {
    selectedRecipe = null;
    searchQuery = "";
    mealType = "dinner";
    servings = 4;
  }

  const mealTypes: { value: MealType; label: string }[] = [
    { value: "breakfast", label: "Breakfast" },
    { value: "lunch", label: "Lunch" },
    { value: "dinner", label: "Dinner" },
    { value: "snack", label: "Snack" },
  ];

  const formattedDate = $derived(
    new Date(date + "T00:00:00").toLocaleDateString("en-US", {
      weekday: "long",
      month: "short",
      day: "numeric",
    })
  );
</script>

<Modal {isOpen} {onClose} title="Add Meal for {formattedDate}">
  {#snippet children()}
    {#if !selectedRecipe}
      <div class="space-y-4">
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search recipes..."
          class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500"
        />

        <div class="max-h-64 overflow-y-auto space-y-2">
          {#each filteredRecipes as recipe}
            <button
              type="button"
              onclick={() => { selectedRecipe = recipe; servings = recipe.servings; }}
              class="w-full text-left px-4 py-3 rounded-lg hover:bg-gray-50 border border-gray-200"
            >
              <div class="font-medium text-gray-800">{recipe.name}</div>
              <div class="text-sm text-gray-500">
                {recipe.prepTime + recipe.cookTime} min · {recipe.servings} servings
              </div>
            </button>
          {/each}
        </div>
      </div>
    {:else}
      <div class="space-y-4">
        <div class="p-4 bg-emerald-50 rounded-lg">
          <div class="font-medium text-emerald-800">{selectedRecipe.name}</div>
          <button
            type="button"
            onclick={() => selectedRecipe = null}
            class="text-sm text-emerald-600 hover:text-emerald-700"
          >
            Change recipe
          </button>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Meal Type</label>
          <div class="grid grid-cols-4 gap-2">
            {#each mealTypes as mt}
              <button
                type="button"
                onclick={() => mealType = mt.value}
                class="px-3 py-2 text-sm rounded-lg border transition-colors
                  {mealType === mt.value
                    ? 'bg-emerald-600 text-white border-emerald-600'
                    : 'border-gray-300 hover:border-emerald-500'}"
              >
                {mt.label}
              </button>
            {/each}
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Servings</label>
          <div class="flex items-center gap-3">
            <button
              type="button"
              onclick={() => servings = Math.max(1, servings - 1)}
              class="w-10 h-10 rounded-lg bg-gray-100 hover:bg-gray-200 text-xl transition-colors"
              aria-label="Decrease servings"
            >
              -
            </button>
            <span class="text-xl font-medium w-12 text-center">{servings}</span>
            <button
              type="button"
              onclick={() => servings++}
              class="w-10 h-10 rounded-lg bg-gray-100 hover:bg-gray-200 text-xl transition-colors"
              aria-label="Increase servings"
            >
              +
            </button>
          </div>
        </div>

        <button
          type="button"
          onclick={handleConfirm}
          class="w-full py-3 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
        >
          Add to Meal Plan
        </button>
      </div>
    {/if}
  {/snippet}
</Modal>
