export interface MealPlan {
  id: string;
  date: string; // ISO date string YYYY-MM-DD
  meals: PlannedMeal[];
}

export interface PlannedMeal {
  id: string;
  recipeId: string;
  mealType: MealType;
  servings: number;
}

export type MealType = 'breakfast' | 'lunch' | 'dinner' | 'snack';
