import { describe, it, expect, vi, beforeEach } from "vitest";
import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core");

const mockRecipe = {
  id: "1",
  name: "Test Recipe",
  description: "Test",
  prepTime: 10,
  cookTime: 20,
  servings: 4,
  ingredients: [{ id: "i1", name: "Salt", quantity: 1, unit: "tsp" }],
  instructions: ["Step 1"],
  tags: [],
  createdAt: "2025-01-01",
  updatedAt: "2025-01-01",
};

describe("recipeStore", () => {
  // Re-import fresh module for each test to reset store state
  let recipeStore: typeof import("./recipes").recipeStore;
  let recipesLoading: typeof import("./recipes").recipesLoading;
  let recipesError: typeof import("./recipes").recipesError;
  let recipeById: typeof import("./recipes").recipeById;

  beforeEach(async () => {
    vi.clearAllMocks();
    vi.resetModules();
    const module = await import("./recipes");
    recipeStore = module.recipeStore;
    recipesLoading = module.recipesLoading;
    recipesError = module.recipesError;
    recipeById = module.recipeById;
  });

  it("loads recipes from backend", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce([{ id: "1", name: "Test Recipe" }]) // getRecipes
      .mockResolvedValueOnce(mockRecipe); // getRecipe

    await recipeStore.load();

    expect(invoke).toHaveBeenCalledWith("get_recipes");
    expect(invoke).toHaveBeenCalledWith("get_recipe", { id: "1" });
    expect(get(recipeStore)).toEqual([mockRecipe]);
    expect(get(recipesLoading)).toBe(false);
  });

  it("sets error on load failure", async () => {
    vi.mocked(invoke).mockRejectedValueOnce(new Error("Network error"));

    await recipeStore.load();

    expect(get(recipesError)).toBe("Network error");
    expect(get(recipeStore)).toEqual([]);
  });

  it("adds recipe via backend", async () => {
    vi.mocked(invoke).mockResolvedValueOnce(mockRecipe);

    const result = await recipeStore.add({
      name: "Test Recipe",
      description: "Test",
      prepTime: 10,
      cookTime: 20,
      servings: 4,
      ingredients: [],
      instructions: [],
      tags: [],
    });

    expect(invoke).toHaveBeenCalledWith("create_recipe", expect.any(Object));
    expect(result).toEqual(mockRecipe);
  });

  it("removes recipe via backend", async () => {
    // Setup: add a recipe first
    vi.mocked(invoke)
      .mockResolvedValueOnce([{ id: "1" }])
      .mockResolvedValueOnce(mockRecipe);
    await recipeStore.load();

    vi.mocked(invoke).mockResolvedValueOnce(undefined);
    await recipeStore.remove("1");

    expect(invoke).toHaveBeenCalledWith("delete_recipe", { id: "1" });
    expect(get(recipeStore)).toEqual([]);
  });

  it("recipeById derived store creates map", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce([{ id: "1" }])
      .mockResolvedValueOnce(mockRecipe);
    await recipeStore.load();

    const map = get(recipeById);
    expect(map.get("1")).toEqual(mockRecipe);
  });
});
