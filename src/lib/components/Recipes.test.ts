import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";

import type { Recipe } from "$lib/types";

let currentRecipes: Recipe[] = [];
let currentLoading = false;
const recipesSubscribers = new Set<(value: Recipe[]) => void>();
const loadingSubscribers = new Set<(value: boolean) => void>();
const recipeByIdSubscribers = new Set<(value: Map<string, Recipe>) => void>();

const mockLoad = vi.fn();
const mockAdd = vi.fn();
const mockUpdate = vi.fn();
const mockRemove = vi.fn();

function setMockRecipes(recipes: Recipe[]) {
  currentRecipes = recipes;
  recipesSubscribers.forEach((fn) => fn(currentRecipes));
  const map = new Map(currentRecipes.map((r) => [r.id, r]));
  recipeByIdSubscribers.forEach((fn) => fn(map));
}

function setMockLoading(loading: boolean) {
  currentLoading = loading;
  loadingSubscribers.forEach((fn) => fn(currentLoading));
}

vi.mock("$lib/stores", () => ({
  recipeStore: {
    subscribe: (fn: (value: Recipe[]) => void) => {
      recipesSubscribers.add(fn);
      fn(currentRecipes);
      return () => recipesSubscribers.delete(fn);
    },
    load: () => mockLoad(),
    add: (data: unknown) => mockAdd(data),
    update: (id: string, data: unknown) => mockUpdate(id, data),
    remove: (id: string) => mockRemove(id),
  },
  recipesLoading: {
    subscribe: (fn: (value: boolean) => void) => {
      loadingSubscribers.add(fn);
      fn(currentLoading);
      return () => loadingSubscribers.delete(fn);
    },
  },
  recipeById: {
    subscribe: (fn: (value: Map<string, Recipe>) => void) => {
      recipeByIdSubscribers.add(fn);
      const map = new Map(currentRecipes.map((r) => [r.id, r]));
      fn(map);
      return () => recipeByIdSubscribers.delete(fn);
    },
  },
  groupRecipes: vi.fn((recipes: Recipe[], _fn: unknown) => {
    // Simple mock that groups by first tag
    const groups = new Map<string, Recipe[]>();
    for (const recipe of recipes) {
      const key = recipe.tags[0] ?? "uncategorized";
      if (!groups.has(key)) groups.set(key, []);
      groups.get(key)!.push(recipe);
    }
    return groups;
  }),
  getRecipeProtein: vi.fn(() => "uncategorized"),
  getRecipeStarch: vi.fn(() => "uncategorized"),
}));

import Recipes from "./Recipes.svelte";

const createMockRecipe = (overrides: Partial<Recipe> = {}): Recipe => ({
  id: "r1",
  name: "Chicken Stir Fry",
  description: "Quick and easy dinner",
  prepTime: 15,
  cookTime: 20,
  servings: 4,
  ingredients: [{ id: "i1", name: "Chicken", quantity: 1, unit: "lb" }],
  instructions: ["Cook chicken", "Add vegetables"],
  tags: ["asian", "quick"],
  createdAt: "2025-01-01",
  ...overrides,
});

describe("Recipes", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setMockRecipes([]);
    setMockLoading(false);
  });

  describe("Loading/Initial State", () => {
    it("calls recipeStore.load() on mount", () => {
      render(Recipes);

      expect(mockLoad).toHaveBeenCalledTimes(1);
    });

    it("shows loading spinner when recipesLoading is true", () => {
      setMockLoading(true);

      render(Recipes);

      const spinner = document.querySelector(".animate-spin");
      expect(spinner).toBeInTheDocument();
      expect(screen.getByText("Loading recipes...")).toBeInTheDocument();
    });

    it("shows empty state when no recipes and not loading", () => {
      setMockLoading(false);
      setMockRecipes([]);

      render(Recipes);

      expect(
        screen.getByText("No recipes yet. Add your first recipe to get started!")
      ).toBeInTheDocument();
    });
  });

  describe("Recipe Display", () => {
    it("renders recipe cards when recipes exist", () => {
      const testRecipes: Recipe[] = [
        createMockRecipe({ id: "r1", name: "Chicken Stir Fry" }),
        createMockRecipe({ id: "r2", name: "Beef Tacos", tags: ["mexican"] }),
      ];
      setMockRecipes(testRecipes);

      render(Recipes);

      expect(screen.getByText("Chicken Stir Fry")).toBeInTheDocument();
      expect(screen.getByText("Beef Tacos")).toBeInTheDocument();
    });
  });

  describe("Filtering", () => {
    it("filters recipes by name search query", async () => {
      const testRecipes: Recipe[] = [
        createMockRecipe({ id: "r1", name: "Chicken Stir Fry" }),
        createMockRecipe({ id: "r2", name: "Beef Tacos", tags: ["mexican"] }),
        createMockRecipe({ id: "r3", name: "Chicken Curry", tags: ["indian"] }),
      ];
      setMockRecipes(testRecipes);

      render(Recipes);

      // All recipes should be visible initially
      expect(screen.getByText("Chicken Stir Fry")).toBeInTheDocument();
      expect(screen.getByText("Beef Tacos")).toBeInTheDocument();
      expect(screen.getByText("Chicken Curry")).toBeInTheDocument();

      // Enter search query
      const searchInput = screen.getByPlaceholderText(
        "Search recipes by name or tag..."
      );
      await fireEvent.input(searchInput, { target: { value: "chicken" } });

      // Only chicken recipes should be visible
      expect(screen.getByText("Chicken Stir Fry")).toBeInTheDocument();
      expect(screen.getByText("Chicken Curry")).toBeInTheDocument();
      expect(screen.queryByText("Beef Tacos")).not.toBeInTheDocument();
    });

    it("shows 'no recipes found' when search matches nothing", async () => {
      const testRecipes: Recipe[] = [
        createMockRecipe({ id: "r1", name: "Chicken Stir Fry" }),
      ];
      setMockRecipes(testRecipes);

      render(Recipes);

      const searchInput = screen.getByPlaceholderText(
        "Search recipes by name or tag..."
      );
      await fireEvent.input(searchInput, { target: { value: "nonexistent" } });

      expect(
        screen.getByText(
          "No recipes found. Try adjusting your filters or add a new recipe!"
        )
      ).toBeInTheDocument();
    });
  });

  describe("Delete Flow", () => {
    it("ConfirmDialog is hidden initially", () => {
      const testRecipes: Recipe[] = [
        createMockRecipe({ id: "r1", name: "Chicken Stir Fry" }),
      ];
      setMockRecipes(testRecipes);

      render(Recipes);

      // ConfirmDialog should not show delete confirmation initially
      expect(
        screen.queryByText('Are you sure you want to delete "Chicken Stir Fry"?')
      ).not.toBeInTheDocument();
    });

    // Note: The delete flow requires setting deleteTarget which is internal state.
    // The current UI doesn't expose a delete button on RecipeCard or RecipeDetailPanel.
    // These tests verify the ConfirmDialog component is wired correctly with the props.
  });

  describe("Modal Navigation", () => {
    it("Add Recipe button opens create modal", async () => {
      render(Recipes);

      // Modal dialog should not exist initially
      expect(screen.queryByRole("dialog")).not.toBeInTheDocument();

      const addButton = screen.getByRole("button", { name: /\+ Add Recipe/i });
      await fireEvent.click(addButton);

      // Modal should now be open - check for dialog role
      const dialog = screen.getByRole("dialog");
      expect(dialog).toBeInTheDocument();
      // Check for form text specific to RecipeForm (labels exist as text)
      expect(screen.getByText("Recipe Name")).toBeInTheDocument();
      expect(screen.getByText("Description")).toBeInTheDocument();
    });

    it("Import from URL button opens import modal", async () => {
      render(Recipes);

      // Modal should not exist initially
      expect(screen.queryByRole("dialog")).not.toBeInTheDocument();

      const importButton = screen.getByRole("button", { name: /Import from URL/i });
      await fireEvent.click(importButton);

      // Modal should now be open - dialog exists and we can find the recipe URL field
      expect(screen.getByRole("dialog")).toBeInTheDocument();
      expect(screen.getByPlaceholderText("https://www.example.com/recipe/...")).toBeInTheDocument();
    });
  });
});
