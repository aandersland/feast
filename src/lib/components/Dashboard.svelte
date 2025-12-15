<script lang="ts">
  import WeeklyCalendar from "./dashboard/WeeklyCalendar.svelte";
  import QuickStats from "./dashboard/QuickStats.svelte";
  import RecentRecipes from "./dashboard/RecentRecipes.svelte";
</script>

<div class="space-y-6 max-w-[1800px] 3xl:max-w-[2400px] mx-auto px-2 sm:px-4 2xl:px-6">
  <QuickStats />
  <WeeklyCalendar />

  <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 sm:gap-6">
    <RecentRecipes />

    <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-6">
      <h2 class="text-lg sm:text-xl font-semibold text-gray-800 mb-4">Quick Actions</h2>
      <div class="space-y-2">
        <button
          type="button"
          class="w-full text-left px-4 py-3 rounded-lg bg-emerald-50 text-emerald-700 hover:bg-emerald-100 transition-colors"
        >
          + Add new recipe
        </button>
        <button
          type="button"
          class="w-full text-left px-4 py-3 rounded-lg bg-blue-50 text-blue-700 hover:bg-blue-100 transition-colors"
        >
          Plan this week's meals
        </button>
        <button
          type="button"
          class="w-full text-left px-4 py-3 rounded-lg bg-gray-50 text-gray-700 hover:bg-gray-100 transition-colors"
        >
          View shopping list
        </button>
      </div>
    </div>
  </div>
</div>
