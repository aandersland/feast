<script lang="ts">
  import { mealPlanStore, aggregatedShoppingList } from "$lib/stores";
  import { derived } from "svelte/store";

  const stats = derived(
    [mealPlanStore, aggregatedShoppingList],
    ([$plans, $items]) => ({
      mealsPlanned: $plans.reduce((acc, p) => acc + p.meals.length, 0),
      shoppingItems: $items.filter((i) => !i.isOnHand).length,
      itemsOnHand: $items.filter((i) => i.isOnHand).length,
    })
  );
</script>

<div class="grid grid-cols-1 sm:grid-cols-3 gap-3 sm:gap-4">
  <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-4 hover:shadow-md transition-shadow">
    <div class="text-3xl font-bold text-emerald-600">{$stats.mealsPlanned}</div>
    <div class="text-sm text-gray-500">Meals planned</div>
  </div>

  <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-4 hover:shadow-md transition-shadow">
    <div class="text-3xl font-bold text-blue-600">{$stats.shoppingItems}</div>
    <div class="text-sm text-gray-500">Items to buy</div>
  </div>

  <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-4 hover:shadow-md transition-shadow">
    <div class="text-3xl font-bold text-gray-600">{$stats.itemsOnHand}</div>
    <div class="text-sm text-gray-500">Items on hand</div>
  </div>
</div>
