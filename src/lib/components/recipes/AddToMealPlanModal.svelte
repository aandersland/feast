<script lang="ts">
  import type { Recipe, MealType } from "$lib/types";
  import { mealPlanStore } from "$lib/stores";
  import Modal from "$lib/components/shared/Modal.svelte";

  interface Props {
    isOpen: boolean;
    recipe: Recipe;
    onClose: () => void;
  }

  let { isOpen, recipe, onClose }: Props = $props();

  let selectedDate = $state(new Date().toISOString().split("T")[0]);
  let selectedMealType = $state<MealType>("dinner");
  let servings = $state(4);

  const mealTypes: MealType[] = ["breakfast", "lunch", "dinner", "snack"];

  function handleAdd() {
    mealPlanStore.addMeal(selectedDate, recipe.id, selectedMealType, servings);
    onClose();
  }

  // Reset form when opened
  $effect(() => {
    if (isOpen) {
      selectedDate = new Date().toISOString().split("T")[0];
      selectedMealType = "dinner";
      servings = recipe.servings;
    }
  });
</script>

<Modal {isOpen} {onClose} title="Add to Meal Plan">
  {#snippet children()}
    <div class="space-y-4">
      <p class="text-gray-600">Add <strong>{recipe.name}</strong> to your meal plan.</p>

      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Date</label>
        <input
          type="date"
          bind:value={selectedDate}
          class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
        />
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Meal</label>
        <select
          bind:value={selectedMealType}
          class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
        >
          {#each mealTypes as type}
            <option value={type} class="capitalize">{type}</option>
          {/each}
        </select>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Servings</label>
        <input
          type="number"
          bind:value={servings}
          min="1"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
        />
      </div>

      <div class="flex justify-end gap-2 pt-4">
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
        >
          Cancel
        </button>
        <button
          type="button"
          onclick={handleAdd}
          class="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
        >
          Add
        </button>
      </div>
    </div>
  {/snippet}
</Modal>
