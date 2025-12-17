import { writable, derived, get } from "svelte/store";
import type { Recipe } from "$lib/types";
import { PROTEIN_KEYWORDS, STARCH_KEYWORDS } from "$lib/types";
import {
  getRecipes,
  getRecipe,
  createRecipe as createRecipeCmd,
  updateRecipe as updateRecipeCmd,
  deleteRecipe as deleteRecipeCmd,
  type RecipeInput,
} from "$lib/tauri/commands";
import { toastStore } from "./toast";

// Loading and error state
export const recipesLoading = writable(false);
export const recipesError = writable<string | null>(null);

const { subscribe, set, update } = writable<Recipe[]>([]);

// Transform backend RecipeRow[] to frontend Recipe[]
// Note: getRecipe returns full Recipe with ingredients, getRecipes returns RecipeRow[]
async function loadRecipes() {
  recipesLoading.set(true);
  recipesError.set(null);
  try {
    const rows = await getRecipes();
    // Fetch full recipe details for each (includes ingredients)
    const recipes = await Promise.all(rows.map((row) => getRecipe(row.id)));
    set(recipes);
  } catch (e) {
    const message = e instanceof Error ? e.message : "Failed to load recipes";
    recipesError.set(message);
    toastStore.error(message);
  } finally {
    recipesLoading.set(false);
  }
}

// Convert frontend Recipe to backend RecipeInput
function toRecipeInput(recipe: Omit<Recipe, "id" | "createdAt" | "updatedAt">): RecipeInput {
  return {
    name: recipe.name,
    description: recipe.description ?? "",
    prepTime: recipe.prepTime ?? 0,
    cookTime: recipe.cookTime ?? 0,
    servings: recipe.servings ?? 1,
    imageUrl: recipe.imageUrl,
    sourceUrl: recipe.sourceUrl,
    notes: recipe.notes,
    tags: recipe.tags ?? [],
    ingredients: recipe.ingredients.map((i) => ({
      name: i.name,
      quantity: i.quantity,
      unit: i.unit,
      notes: i.notes,
    })),
    instructions: recipe.instructions ?? [],
  };
}

export const recipeStore = {
  subscribe,
  load: loadRecipes,

  add: async (recipe: Omit<Recipe, "id" | "createdAt" | "updatedAt">) => {
    try {
      const created = await createRecipeCmd(toRecipeInput(recipe));
      update((recipes) => [...recipes, created]);
      toastStore.success("Recipe created");
      return created;
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to create recipe";
      toastStore.error(message);
      throw e;
    }
  },

  remove: async (id: string) => {
    try {
      await deleteRecipeCmd(id);
      update((recipes) => recipes.filter((r) => r.id !== id));
      toastStore.success("Recipe deleted");
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to delete recipe";
      toastStore.error(message);
      throw e;
    }
  },

  update: async (id: string, data: Partial<Recipe>) => {
    try {
      const current = get({ subscribe }).find((r) => r.id === id);
      if (!current) throw new Error("Recipe not found");

      const input = toRecipeInput({ ...current, ...data });
      const updated = await updateRecipeCmd(id, input);
      update((recipes) => recipes.map((r) => (r.id === id ? updated : r)));
      toastStore.success("Recipe updated");
      return updated;
    } catch (e) {
      const message = e instanceof Error ? e.message : "Failed to update recipe";
      toastStore.error(message);
      throw e;
    }
  },
};

export const recipeById = derived(recipeStore, ($recipes) => {
  const map = new Map<string, Recipe>();
  $recipes.forEach((r) => map.set(r.id, r));
  return map;
});

// Extract all unique ingredient names across recipes
export const allIngredients = derived(recipeStore, ($recipes) => {
  const ingredients = new Set<string>();
  $recipes.forEach((r) => {
    r.ingredients.forEach((i) => {
      ingredients.add(i.name.toLowerCase());
    });
  });
  return Array.from(ingredients).sort();
});

// Helper to detect protein in recipe
export function getRecipeProtein(recipe: Recipe): string | null {
  const text = recipe.ingredients.map(i => i.name.toLowerCase()).join(" ");
  for (const protein of PROTEIN_KEYWORDS) {
    if (text.includes(protein)) return protein;
  }
  return null;
}

// Helper to detect starch in recipe
export function getRecipeStarch(recipe: Recipe): string | null {
  const text = recipe.ingredients.map(i => i.name.toLowerCase()).join(" ");
  for (const starch of STARCH_KEYWORDS) {
    if (text.includes(starch)) return starch;
  }
  return null;
}

// Group recipes by a key function
export function groupRecipes<K extends string>(
  recipes: Recipe[],
  keyFn: (r: Recipe) => K | null
): Map<K | "Other", Recipe[]> {
  const groups = new Map<K | "Other", Recipe[]>();
  recipes.forEach((r) => {
    const key = keyFn(r) ?? "Other";
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key)!.push(r);
  });
  return groups;
}
