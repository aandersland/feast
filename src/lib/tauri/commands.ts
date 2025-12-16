import { invoke } from "@tauri-apps/api/core";
import type { Item, Recipe, Ingredient } from "$lib/types";

export async function greet(name: string): Promise<string> {
  return invoke<string>("greet", { name });
}

export async function getItems(): Promise<Item[]> {
  return invoke<Item[]>("get_items");
}

export async function createItem(name: string): Promise<Item> {
  return invoke<Item>("create_item", { name });
}

export async function deleteItem(id: number): Promise<void> {
  return invoke<void>("delete_item", { id });
}

// Recipe types for backend
export interface RecipeInput {
  name: string;
  description: string;
  prepTime: number;
  cookTime: number;
  servings: number;
  imagePath?: string;
  sourceUrl?: string;
  notes?: string;
  tags: string[];
  ingredients: IngredientInput[];
  instructions: string[];
}

export interface IngredientInput {
  name: string;
  quantity: number;
  unit: string;
  category?: string;
  notes?: string;
}

export interface RecipeRow {
  id: string;
  name: string;
  description: string;
  prepTime: number;
  cookTime: number;
  servings: number;
  imagePath?: string;
  sourceUrl?: string;
  notes?: string;
  createdAt: string;
  updatedAt: string;
}

// Recipe commands
export async function getRecipes(): Promise<RecipeRow[]> {
  return invoke<RecipeRow[]>("get_recipes");
}

export async function getRecipe(id: string): Promise<Recipe> {
  return invoke<Recipe>("get_recipe", { id });
}

export async function createRecipe(input: RecipeInput): Promise<Recipe> {
  return invoke<Recipe>("create_recipe", { input });
}

export async function updateRecipe(id: string, input: RecipeInput): Promise<Recipe> {
  return invoke<Recipe>("update_recipe", { id, input });
}

export async function deleteRecipe(id: string): Promise<void> {
  return invoke<void>("delete_recipe", { id });
}

// Ingredient commands
export async function getIngredients(): Promise<Ingredient[]> {
  return invoke<Ingredient[]>("get_ingredients");
}

export async function createIngredient(
  name: string,
  category: string,
  defaultUnit?: string
): Promise<Ingredient> {
  return invoke<Ingredient>("create_ingredient", { name, category, defaultUnit });
}

export async function getOrCreateIngredient(
  name: string,
  category: string,
  defaultUnit?: string
): Promise<Ingredient> {
  return invoke<Ingredient>("get_or_create_ingredient", { name, category, defaultUnit });
}

// Meal plan types and commands
// Backend returns flat rows (one row per meal), frontend groups by date
export interface MealPlanRow {
  id: string;
  date: string;
  mealType: string;
  recipeId: string;
  servings: number;
  createdAt: string;
}

export interface MealPlanInput {
  date: string;
  mealType: string;
  recipeId: string;
  servings: number;
}

export async function getMealPlans(startDate: string, endDate: string): Promise<MealPlanRow[]> {
  return invoke<MealPlanRow[]>("get_meal_plans", { startDate, endDate });
}

export async function createMealPlan(input: MealPlanInput): Promise<MealPlanRow> {
  return invoke<MealPlanRow>("create_meal_plan", { input });
}

export async function updateMealPlan(id: string, servings: number): Promise<MealPlanRow> {
  return invoke<MealPlanRow>("update_meal_plan", { id, servings });
}

export async function deleteMealPlan(id: string): Promise<void> {
  return invoke<void>("delete_meal_plan", { id });
}

// Shopping list types and commands
export interface ShoppingList {
  id: string;
  weekStart: string;
  name: string;
  listType: string;
  createdAt: string;
}

export interface ShoppingListItem {
  id: string;
  listId: string;
  ingredientId?: string;
  name: string;
  quantity: number;
  unit: string;
  category: string;
  isChecked: boolean;
  isDeleted: boolean;
  deletedAt?: string;
  movedToListId?: string;
  sourceRecipeIds?: string;
  createdAt: string;
}

export interface ShoppingListWithItems extends ShoppingList {
  items: ShoppingListItem[];
}

export interface ShoppingListInput {
  weekStart: string;
  name: string;
  listType?: string;
}

export interface ShoppingItemInput {
  listId: string;
  name: string;
  quantity: number;
  unit: string;
  category: string;
}

export interface AggregatedShoppingItem {
  name: string;
  quantity: number;
  unit: string;
  category: string;
  sourceRecipeIds: string[];
  isConverted: boolean;
}

export async function getShoppingLists(weekStart: string): Promise<ShoppingListWithItems[]> {
  return invoke<ShoppingListWithItems[]>("get_shopping_lists", { weekStart });
}

export async function createShoppingList(input: ShoppingListInput): Promise<ShoppingList> {
  return invoke<ShoppingList>("create_shopping_list", { input });
}

export async function deleteShoppingList(id: string): Promise<void> {
  return invoke<void>("delete_shopping_list", { id });
}

export async function addShoppingItem(input: ShoppingItemInput): Promise<ShoppingListItem> {
  return invoke<ShoppingListItem>("add_shopping_item", { input });
}

export async function updateShoppingItem(
  id: string,
  quantity?: number,
  isChecked?: boolean
): Promise<ShoppingListItem> {
  return invoke<ShoppingListItem>("update_shopping_item", { id, quantity, isChecked });
}

export async function softDeleteShoppingItem(id: string): Promise<void> {
  return invoke<void>("soft_delete_shopping_item", { id });
}

export async function restoreShoppingItem(id: string): Promise<ShoppingListItem> {
  return invoke<ShoppingListItem>("restore_shopping_item", { id });
}

export async function moveShoppingItem(id: string, toListId: string): Promise<ShoppingListItem> {
  return invoke<ShoppingListItem>("move_shopping_item", { id, toListId });
}

export async function getAggregatedShoppingList(
  startDate: string,
  endDate: string
): Promise<AggregatedShoppingItem[]> {
  return invoke<AggregatedShoppingItem[]>("get_aggregated_shopping_list", { startDate, endDate });
}

// Quick list types and commands
export interface QuickList {
  id: string;
  name: string;
  createdAt: string;
  updatedAt: string;
}

export interface QuickListItem {
  id: string;
  quickListId: string;
  name: string;
  quantity: number;
  unit: string;
  category: string;
}

export interface QuickListWithItems extends QuickList {
  items: QuickListItem[];
}

export interface QuickListItemInput {
  name: string;
  quantity: number;
  unit: string;
  category: string;
}

export async function getQuickLists(): Promise<QuickListWithItems[]> {
  return invoke<QuickListWithItems[]>("get_quick_lists");
}

export async function createQuickList(name: string): Promise<QuickList> {
  return invoke<QuickList>("create_quick_list", { name });
}

export async function updateQuickList(id: string, name: string): Promise<QuickList> {
  return invoke<QuickList>("update_quick_list", { id, name });
}

export async function deleteQuickList(id: string): Promise<void> {
  return invoke<void>("delete_quick_list", { id });
}

export async function addQuickListItem(
  quickListId: string,
  input: QuickListItemInput
): Promise<QuickListItem> {
  return invoke<QuickListItem>("add_quick_list_item", { quickListId, input });
}

export async function updateQuickListItem(
  id: string,
  input: QuickListItemInput
): Promise<QuickListItem> {
  return invoke<QuickListItem>("update_quick_list_item", { id, input });
}

export async function removeQuickListItem(id: string): Promise<void> {
  return invoke<void>("remove_quick_list_item", { id });
}

export async function addQuickListToShopping(
  quickListId: string,
  shoppingListId: string
): Promise<ShoppingListItem[]> {
  return invoke<ShoppingListItem[]>("add_quick_list_to_shopping", { quickListId, shoppingListId });
}

// Manual item types and commands
export interface ManualItem {
  id: string;
  weekStart: string;
  name: string;
  quantity: number;
  unit: string;
  category: string;
  isChecked: boolean;
  createdAt: string;
}

export interface ManualItemInput {
  weekStart: string;
  name: string;
  quantity: number;
  unit: string;
  category: string;
}

export async function getManualItems(weekStart: string): Promise<ManualItem[]> {
  return invoke<ManualItem[]>("get_manual_items", { weekStart });
}

export async function createManualItem(input: ManualItemInput): Promise<ManualItem> {
  return invoke<ManualItem>("create_manual_item", { input });
}

export async function updateManualItem(
  id: string,
  quantity?: number,
  isChecked?: boolean
): Promise<ManualItem> {
  return invoke<ManualItem>("update_manual_item", { id, quantity, isChecked });
}

export async function deleteManualItem(id: string): Promise<void> {
  return invoke<void>("delete_manual_item", { id });
}
