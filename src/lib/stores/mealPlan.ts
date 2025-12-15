import { writable, derived } from "svelte/store";
import type { MealPlan, PlannedMeal, MealType } from "$lib/types";

// Generate current week's dates
function getCurrentWeekDates(): string[] {
  const today = new Date();
  const monday = new Date(today);
  monday.setDate(today.getDate() - today.getDay() + 1);

  return Array.from({ length: 7 }, (_, i) => {
    const date = new Date(monday);
    date.setDate(monday.getDate() + i);
    return date.toISOString().split("T")[0];
  });
}

const weekDates = getCurrentWeekDates();

const mockMealPlans: MealPlan[] = [
  {
    id: "1",
    date: weekDates[0],
    meals: [
      { id: "m1", recipeId: "1", mealType: "dinner", servings: 4 },
    ],
  },
  {
    id: "2",
    date: weekDates[2],
    meals: [
      { id: "m2", recipeId: "2", mealType: "lunch", servings: 2 },
      { id: "m3", recipeId: "3", mealType: "dinner", servings: 4 },
    ],
  },
];

function createMealPlanStore() {
  const { subscribe, set, update } = writable<MealPlan[]>(mockMealPlans);

  return {
    subscribe,
    addMeal: (date: string, recipeId: string, mealType: MealType, servings: number) =>
      update((plans) => {
        const existing = plans.find((p) => p.date === date);
        const newMeal: PlannedMeal = {
          id: crypto.randomUUID(),
          recipeId,
          mealType,
          servings,
        };

        if (existing) {
          return plans.map((p) =>
            p.date === date ? { ...p, meals: [...p.meals, newMeal] } : p
          );
        }

        return [...plans, { id: crypto.randomUUID(), date, meals: [newMeal] }];
      }),
    removeMeal: (date: string, mealId: string) =>
      update((plans) =>
        plans.map((p) =>
          p.date === date
            ? { ...p, meals: p.meals.filter((m) => m.id !== mealId) }
            : p
        ).filter((p) => p.meals.length > 0)
      ),
    updateServings: (date: string, mealId: string, servings: number) =>
      update((plans) =>
        plans.map((p) =>
          p.date === date
            ? {
                ...p,
                meals: p.meals.map((m) =>
                  m.id === mealId ? { ...m, servings } : m
                ),
              }
            : p
        )
      ),
  };
}

export const mealPlanStore = createMealPlanStore();

export const mealPlanByDate = derived(mealPlanStore, ($plans) => {
  const map = new Map<string, MealPlan>();
  $plans.forEach((p) => map.set(p.date, p));
  return map;
});
