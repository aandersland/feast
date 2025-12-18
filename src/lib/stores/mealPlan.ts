import { writable, derived } from "svelte/store";
import type { MealPlan, PlannedMeal, MealType } from "$lib/types";
import {
  getMealPlans as getMealPlansCmd,
  createMealPlan as createMealPlanCmd,
  updateMealPlan as updateMealPlanCmd,
  deleteMealPlan as deleteMealPlanCmd,
  type MealPlanRow,
} from "$lib/tauri/commands";
import { toastStore } from "./toast";
import { log } from "$lib/logging";
import { getCurrentCorrelationId } from "$lib/tauri/tracing";

// Loading and error state
export const mealPlansLoading = writable(false);
export const mealPlansError = writable<string | null>(null);

const { subscribe, set, update } = writable<MealPlan[]>([]);

// Transform flat backend rows to nested frontend structure
function groupMealsByDate(flatMeals: MealPlanRow[]): MealPlan[] {
  const byDate = new Map<string, MealPlan>();

  for (const meal of flatMeals) {
    if (!byDate.has(meal.date)) {
      byDate.set(meal.date, {
        id: meal.date, // Use date as MealPlan ID
        date: meal.date,
        meals: [],
      });
    }
    byDate.get(meal.date)!.meals.push({
      id: meal.id,
      recipeId: meal.recipeId,
      mealType: meal.mealType as MealType,
      servings: meal.servings,
    });
  }

  return Array.from(byDate.values());
}

async function loadMealPlans(startDate: string, endDate: string) {
  mealPlansLoading.set(true);
  mealPlansError.set(null);
  log.debug("Loading meal plans", "store::mealPlan", { startDate, endDate }, getCurrentCorrelationId());
  try {
    const flatMeals = await getMealPlansCmd(startDate, endDate);
    const grouped = groupMealsByDate(flatMeals);
    set(grouped);
    log.info("Meal plans loaded", "store::mealPlan", { count: grouped.length }, getCurrentCorrelationId());
  } catch (e) {
    const message = e instanceof Error ? e.message : "Failed to load meal plans";
    mealPlansError.set(message);
    log.error("Failed to load meal plans", "store::mealPlan", { error: message }, getCurrentCorrelationId());
    toastStore.error(message);
  } finally {
    mealPlansLoading.set(false);
  }
}

export const mealPlanStore = {
  subscribe,
  load: loadMealPlans,

  addMeal: async (date: string, recipeId: string, mealType: MealType, servings: number) => {
    try {
      const created = await createMealPlanCmd({ date, mealType, recipeId, servings });

      update((plans) => {
        const existing = plans.find((p) => p.date === date);
        if (existing) {
          return plans.map((p) =>
            p.date === date
              ? { ...p, meals: [...p.meals, { id: created.id, recipeId, mealType, servings }] }
              : p
          );
        } else {
          return [...plans, {
            id: date,
            date,
            meals: [{ id: created.id, recipeId, mealType, servings }],
          }];
        }
      });
      log.info("Meal added", "store::mealPlan", { date, recipeId, mealType }, getCurrentCorrelationId());
      toastStore.success("Meal added to plan");
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to add meal";
      log.error("Failed to add meal", "store::mealPlan", { date, recipeId, error: message }, getCurrentCorrelationId());
      toastStore.error(message);
      throw e;
    }
  },

  removeMeal: async (date: string, mealId: string) => {
    try {
      await deleteMealPlanCmd(mealId);

      update((plans) =>
        plans
          .map((p) =>
            p.date === date ? { ...p, meals: p.meals.filter((m) => m.id !== mealId) } : p
          )
          .filter((p) => p.meals.length > 0)
      );
      log.info("Meal removed", "store::mealPlan", { date, mealId }, getCurrentCorrelationId());
      toastStore.success("Meal removed from plan");
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to remove meal";
      log.error("Failed to remove meal", "store::mealPlan", { date, mealId, error: message }, getCurrentCorrelationId());
      toastStore.error(message);
      throw e;
    }
  },

  updateServings: async (date: string, mealId: string, servings: number) => {
    try {
      await updateMealPlanCmd(mealId, servings);

      update((plans) =>
        plans.map((p) =>
          p.date === date
            ? { ...p, meals: p.meals.map((m) => (m.id === mealId ? { ...m, servings } : m)) }
            : p
        )
      );
      log.info("Servings updated", "store::mealPlan", { mealId, servings }, getCurrentCorrelationId());
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to update servings";
      log.error("Failed to update servings", "store::mealPlan", { mealId, error: message }, getCurrentCorrelationId());
      toastStore.error(message);
      throw e;
    }
  },
};

// Derived store for quick lookup by date
export const mealPlanByDate = derived(mealPlanStore, ($plans) => {
  const map = new Map<string, MealPlan>();
  $plans.forEach((p) => map.set(p.date, p));
  return map;
});
