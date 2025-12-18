<script lang="ts">
  import { mealPlanStore, mealPlanByDate, recipeById } from "$lib/stores";
  import type { MealType } from "$lib/types";
  import RecipePickerModal from "./RecipePickerModal.svelte";

  interface Props {
    weekOffset?: number;
  }

  let { weekOffset = 0 }: Props = $props();

  let isModalOpen = $state(false);
  let selectedDate = $state("");

  const mealTypeOrder: MealType[] = ["breakfast", "lunch", "dinner", "snack"];
  const mealTypeLabels: Record<MealType, string> = {
    breakfast: "Breakfast",
    lunch: "Lunch",
    dinner: "Dinner",
    snack: "Snack",
  };
  const mealTypeColors: Record<MealType, string> = {
    breakfast: "bg-amber-100 text-amber-800",
    lunch: "bg-blue-100 text-blue-800",
    dinner: "bg-emerald-100 text-emerald-800",
    snack: "bg-purple-100 text-purple-800",
  };

  function getWeekDates(): { date: string; dayName: string; dayNum: number; month: string }[] {
    const today = new Date();
    const monday = new Date(today);
    monday.setDate(today.getDate() - today.getDay() + 1 + weekOffset * 7);

    return Array.from({ length: 7 }, (_, i) => {
      const date = new Date(monday);
      date.setDate(monday.getDate() + i);
      return {
        date: date.toISOString().split("T")[0],
        dayName: date.toLocaleDateString("en-US", { weekday: "short" }),
        dayNum: date.getDate(),
        month: date.toLocaleDateString("en-US", { month: "short" }),
      };
    });
  }

  let weekDates = $derived(getWeekDates());
  const today = new Date().toISOString().split("T")[0];

  // Load meal plans when week changes
  $effect(() => {
    const startDate = weekDates[0].date;
    const endDate = weekDates[6].date;
    mealPlanStore.load(startDate, endDate);
  });

  function openAddModal(date: string) {
    selectedDate = date;
    isModalOpen = true;
  }

  function handleAddMeal(recipeId: string, mealType: MealType, servings: number) {
    mealPlanStore.addMeal(selectedDate, recipeId, mealType, servings);
  }

  function handleRemoveMeal(date: string, mealId: string) {
    mealPlanStore.removeMeal(date, mealId);
  }
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
  <div class="grid grid-cols-7 divide-x divide-gray-100">
    {#each weekDates as { date, dayName, dayNum, month }}
      {@const plan = $mealPlanByDate.get(date)}
      {@const isToday = date === today}

      <div class="min-h-[300px] flex flex-col">
        <div class="px-3 py-3 border-b border-gray-100 text-center {isToday ? 'bg-emerald-50' : ''}">
          <div class="text-xs text-gray-500 uppercase">{dayName}</div>
          <div class="text-xl font-semibold {isToday ? 'text-emerald-600' : 'text-gray-800'}">
            {dayNum}
          </div>
          <div class="text-xs text-gray-400">{month}</div>
        </div>

        <div class="flex-1 p-2 space-y-2">
          {#if plan}
            {#each mealTypeOrder as mealType}
              {#each plan.meals.filter(m => m.mealType === mealType) as meal}
                {@const recipe = $recipeById.get(meal.recipeId)}
                {#if recipe}
                  <div class="group relative p-2 rounded-lg {mealTypeColors[mealType]}">
                    <div class="text-[10px] uppercase font-medium opacity-70">
                      {mealTypeLabels[mealType]}
                    </div>
                    <div class="text-sm font-medium truncate">{recipe.name}</div>
                    <div class="text-xs opacity-70">{meal.servings} servings</div>
                    <button
                      type="button"
                      onclick={() => handleRemoveMeal(date, meal.id)}
                      class="absolute top-1 right-1 w-5 h-5 rounded-full bg-white/50 opacity-0 group-hover:opacity-100 transition-opacity text-xs"
                    >
                      ✕
                    </button>
                  </div>
                {/if}
              {/each}
            {/each}
          {/if}
        </div>

        <div class="p-2 border-t border-gray-100">
          <button
            type="button"
            onclick={() => openAddModal(date)}
            class="w-full py-2 text-sm text-gray-400 hover:text-emerald-600 hover:bg-emerald-50 rounded-lg transition-colors"
          >
            + Add meal
          </button>
        </div>
      </div>
    {/each}
  </div>
</div>

<RecipePickerModal
  isOpen={isModalOpen}
  date={selectedDate}
  onClose={() => isModalOpen = false}
  onSelect={handleAddMeal}
/>
