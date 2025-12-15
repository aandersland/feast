<script lang="ts">
  import { mealPlanByDate, recipeById } from "$lib/stores";
  import type { MealType } from "$lib/types";

  const mealTypeOrder: MealType[] = ["breakfast", "lunch", "dinner", "snack"];
  const mealTypeLabels: Record<MealType, string> = {
    breakfast: "Breakfast",
    lunch: "Lunch",
    dinner: "Dinner",
    snack: "Snack",
  };

  function getWeekDates(): { date: string; dayName: string; dayNum: number }[] {
    const today = new Date();
    const monday = new Date(today);
    monday.setDate(today.getDate() - today.getDay() + 1);

    return Array.from({ length: 7 }, (_, i) => {
      const date = new Date(monday);
      date.setDate(monday.getDate() + i);
      return {
        date: date.toISOString().split("T")[0],
        dayName: date.toLocaleDateString("en-US", { weekday: "short" }),
        dayNum: date.getDate(),
      };
    });
  }

  const weekDates = getWeekDates();
  const today = new Date().toISOString().split("T")[0];
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
  <div class="px-6 py-4 border-b border-gray-100">
    <h2 class="text-lg sm:text-xl font-semibold text-gray-800">This Week</h2>
  </div>

  <div class="grid grid-cols-7 divide-x divide-gray-100">
    {#each weekDates as { date, dayName, dayNum }}
      {@const plan = $mealPlanByDate.get(date)}
      {@const isToday = date === today}

      <div class="min-h-[200px] {isToday ? 'bg-emerald-50/50' : ''}">
        <div class="px-3 py-2 border-b border-gray-100 text-center">
          <div class="text-xs text-gray-500 uppercase">{dayName}</div>
          <div class="text-lg font-semibold {isToday ? 'text-emerald-600' : 'text-gray-800'}">
            {dayNum}
          </div>
        </div>

        <div class="p-2 space-y-1">
          {#if plan}
            {#each mealTypeOrder as mealType}
              {#each plan.meals.filter(m => m.mealType === mealType) as meal}
                {@const recipe = $recipeById.get(meal.recipeId)}
                {#if recipe}
                  <div class="text-xs p-2 rounded-lg bg-emerald-100 text-emerald-800 truncate">
                    <span class="font-medium">{recipe.name}</span>
                  </div>
                {/if}
              {/each}
            {/each}
          {:else}
            <div class="text-xs text-gray-400 text-center py-4">No meals</div>
          {/if}
        </div>
      </div>
    {/each}
  </div>
</div>
