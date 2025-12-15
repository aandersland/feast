export interface Recipe {
  id: string;
  name: string;
  description: string;
  ingredients: Ingredient[];
  instructions: string[];
  prepTime: number; // minutes
  cookTime: number; // minutes
  servings: number;
  nutrition?: NutritionInfo;
  sourceUrl?: string;
  notes?: string;
  tags: string[];
  imageUrl?: string;
  createdAt: string;
}

export interface Ingredient {
  id: string;
  name: string;
  quantity: number;
  unit: string;
  notes?: string;
}

export interface NutritionInfo {
  calories: number;
  protein: number;
  carbs: number;
  fat: number;
  fiber?: number;
  sodium?: number;
}

export type RecipeViewMode = "default" | "tag" | "protein" | "starch";

export const PROTEIN_KEYWORDS = [
  "chicken", "beef", "pork", "salmon", "fish", "shrimp", "tofu",
  "turkey", "lamb", "eggs", "seafood"
] as const;

export const STARCH_KEYWORDS = [
  "pasta", "spaghetti", "rice", "potato", "bread", "noodles",
  "quinoa", "oats", "tortilla"
] as const;
