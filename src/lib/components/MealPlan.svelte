<script lang="ts">
  import MealPlanCalendar from "./mealplan/MealPlanCalendar.svelte";
  import ShoppingSection from "./shopping/ShoppingSection.svelte";

  let weekOffset = $state(0);

  function getWeekLabel(offset: number): string {
    const today = new Date();
    const monday = new Date(today);
    monday.setDate(today.getDate() - today.getDay() + 1 + offset * 7);
    const sunday = new Date(monday);
    sunday.setDate(monday.getDate() + 6);

    const fmt = (d: Date) => d.toLocaleDateString("en-US", { month: "short", day: "numeric" });
    return `${fmt(monday)} - ${fmt(sunday)}`;
  }

  let weekLabel = $derived(getWeekLabel(weekOffset));
</script>

<div class="max-w-[1800px] 3xl:max-w-[2400px] mx-auto px-2 sm:px-4 2xl:px-6">
  <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3 mb-6">
    <h1 class="text-xl sm:text-2xl font-bold text-gray-800">Meal Plan</h1>

    <div class="flex items-center gap-2 sm:gap-4">
      <button
        type="button"
        onclick={() => weekOffset--}
        class="p-1.5 sm:p-2 hover:bg-gray-100 rounded-lg transition-colors"
      >
        {'\u2190'}
      </button>
      <span class="text-gray-600 min-w-[140px] sm:min-w-[160px] text-center text-sm sm:text-base">{weekLabel}</span>
      <button
        type="button"
        onclick={() => weekOffset++}
        class="p-1.5 sm:p-2 hover:bg-gray-100 rounded-lg transition-colors"
      >
        {'\u2192'}
      </button>
      {#if weekOffset !== 0}
        <button
          type="button"
          onclick={() => weekOffset = 0}
          class="text-xs sm:text-sm text-emerald-600 hover:text-emerald-700"
        >
          Today
        </button>
      {/if}
    </div>
  </div>

  <MealPlanCalendar {weekOffset} />

  <div class="mt-4 text-sm text-gray-500 text-center">
    Click "+ Add meal" on any day to add a recipe to your meal plan
  </div>

  <!-- Shopping Section -->
  <div class="mt-8 pt-8 border-t border-gray-200">
    <ShoppingSection {weekOffset} />
  </div>
</div>
