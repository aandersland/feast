import { writable, derived } from "svelte/store";
import type { Recipe } from "$lib/types";
import { PROTEIN_KEYWORDS, STARCH_KEYWORDS } from "$lib/types";

const mockRecipes: Recipe[] = [
  {
    id: "1",
    name: "Spaghetti Carbonara",
    description: "Classic Italian pasta with eggs, cheese, and pancetta. A Roman tradition that creates a silky, creamy sauce without any cream.",
    ingredients: [
      { id: "1", name: "Spaghetti", quantity: 400, unit: "g" },
      { id: "2", name: "Pancetta or guanciale", quantity: 200, unit: "g" },
      { id: "3", name: "Eggs", quantity: 4, unit: "" },
      { id: "4", name: "Pecorino Romano", quantity: 75, unit: "g" },
      { id: "5", name: "Parmesan", quantity: 50, unit: "g" },
      { id: "6", name: "Black pepper", quantity: 2, unit: "tsp" },
      { id: "7", name: "Salt", quantity: 1, unit: "tbsp", notes: "for pasta water" },
    ],
    instructions: [
      "Bring a large pot of salted water to boil for the pasta",
      "Cut pancetta into small cubes and cook in a large pan over medium heat until crispy, about 8 minutes",
      "While pancetta cooks, beat eggs with both cheeses and plenty of black pepper in a bowl",
      "Cook spaghetti until al dente, reserving 1 cup pasta water before draining",
      "Remove pan from heat and add drained pasta to pancetta",
      "Quickly add egg mixture, tossing constantly to create a creamy sauce (add pasta water if needed)",
      "Serve immediately with extra cheese and pepper",
    ],
    prepTime: 10,
    cookTime: 20,
    servings: 4,
    nutrition: { calories: 650, protein: 28, carbs: 72, fat: 28 },
    tags: ["Italian", "Pasta", "Quick"],
    createdAt: "2024-01-15",
  },
  {
    id: "2",
    name: "Chicken Stir Fry",
    description: "Quick and healthy chicken with crisp vegetables in a savory sauce. Ready in under 30 minutes for a weeknight dinner.",
    ingredients: [
      { id: "8", name: "Chicken breast", quantity: 500, unit: "g" },
      { id: "9", name: "Bell peppers (mixed colors)", quantity: 2, unit: "" },
      { id: "10", name: "Broccoli florets", quantity: 200, unit: "g" },
      { id: "11", name: "Snap peas", quantity: 150, unit: "g" },
      { id: "12", name: "Soy sauce", quantity: 3, unit: "tbsp" },
      { id: "13", name: "Sesame oil", quantity: 1, unit: "tbsp" },
      { id: "14", name: "Garlic", quantity: 4, unit: "cloves" },
      { id: "15", name: "Fresh ginger", quantity: 1, unit: "inch" },
      { id: "16", name: "Cornstarch", quantity: 1, unit: "tbsp" },
      { id: "17", name: "Rice", quantity: 300, unit: "g", notes: "for serving" },
    ],
    instructions: [
      "Slice chicken breast into thin strips and season with salt and pepper",
      "Mince garlic and grate ginger; mix soy sauce with cornstarch and 2 tbsp water",
      "Chop bell peppers into strips, cut broccoli into small florets",
      "Heat oil in a wok over high heat, stir fry chicken until golden (about 5 minutes), set aside",
      "Add more oil, stir fry vegetables for 3-4 minutes until crisp-tender",
      "Return chicken to wok, add garlic and ginger, cook 30 seconds",
      "Pour in sauce mixture, toss until everything is coated and glossy",
      "Drizzle with sesame oil and serve over steamed rice",
    ],
    prepTime: 15,
    cookTime: 15,
    servings: 4,
    nutrition: { calories: 320, protein: 35, carbs: 15, fat: 12 },
    tags: ["Asian", "Healthy", "Quick"],
    createdAt: "2024-01-20",
  },
  {
    id: "3",
    name: "Caesar Salad",
    description: "Crisp romaine lettuce with homemade Caesar dressing, crunchy croutons, and shaved parmesan.",
    ingredients: [
      { id: "18", name: "Romaine lettuce", quantity: 2, unit: "heads" },
      { id: "19", name: "Parmesan", quantity: 100, unit: "g" },
      { id: "20", name: "Crusty bread", quantity: 200, unit: "g" },
      { id: "21", name: "Garlic", quantity: 3, unit: "cloves" },
      { id: "22", name: "Anchovy fillets", quantity: 4, unit: "" },
      { id: "23", name: "Egg yolks", quantity: 2, unit: "" },
      { id: "24", name: "Lemon juice", quantity: 2, unit: "tbsp" },
      { id: "25", name: "Dijon mustard", quantity: 1, unit: "tsp" },
      { id: "26", name: "Olive oil", quantity: 150, unit: "ml" },
    ],
    instructions: [
      "Cut bread into cubes, toss with olive oil and minced garlic, bake at 375F until golden (10 min)",
      "Mash anchovies with a fork to form a paste",
      "Whisk egg yolks with lemon juice, mustard, and anchovy paste",
      "Slowly drizzle in olive oil while whisking to create an emulsion",
      "Tear romaine into bite-sized pieces and place in a large bowl",
      "Toss lettuce with dressing until well coated",
      "Top with croutons and shaved parmesan, serve immediately",
    ],
    prepTime: 20,
    cookTime: 10,
    servings: 4,
    nutrition: { calories: 380, protein: 12, carbs: 18, fat: 30 },
    tags: ["Salad", "Healthy", "Classic"],
    createdAt: "2024-02-01",
  },
  {
    id: "4",
    name: "Beef Tacos",
    description: "Seasoned ground beef tacos with fresh toppings. A family favorite that is ready in 25 minutes.",
    ingredients: [
      { id: "27", name: "Ground beef", quantity: 500, unit: "g" },
      { id: "28", name: "Taco shells", quantity: 12, unit: "" },
      { id: "29", name: "Onion", quantity: 1, unit: "" },
      { id: "30", name: "Garlic", quantity: 2, unit: "cloves" },
      { id: "31", name: "Cumin", quantity: 2, unit: "tsp" },
      { id: "32", name: "Chili powder", quantity: 1, unit: "tbsp" },
      { id: "33", name: "Tomatoes", quantity: 2, unit: "" },
      { id: "34", name: "Lettuce", quantity: 1, unit: "cup" },
      { id: "35", name: "Cheddar cheese", quantity: 150, unit: "g" },
      { id: "36", name: "Sour cream", quantity: 100, unit: "g" },
    ],
    instructions: [
      "Dice onion and mince garlic",
      "Brown ground beef in a large skillet over medium-high heat",
      "Add onion and garlic, cook until softened (3 minutes)",
      "Stir in cumin, chili powder, and a splash of water; simmer 5 minutes",
      "Warm taco shells according to package directions",
      "Dice tomatoes, shred lettuce, and grate cheese",
      "Fill shells with seasoned beef and top with fresh toppings",
      "Serve with sour cream and your favorite salsa",
    ],
    prepTime: 10,
    cookTime: 15,
    servings: 4,
    nutrition: { calories: 450, protein: 28, carbs: 32, fat: 24 },
    tags: ["Mexican", "Family", "Quick"],
    createdAt: "2024-02-10",
  },
  {
    id: "5",
    name: "Overnight Oats",
    description: "Creamy no-cook oatmeal prepared the night before. Perfect for busy mornings with endless topping options.",
    ingredients: [
      { id: "37", name: "Rolled oats", quantity: 80, unit: "g" },
      { id: "38", name: "Greek yogurt", quantity: 120, unit: "g" },
      { id: "39", name: "Milk", quantity: 120, unit: "ml" },
      { id: "40", name: "Chia seeds", quantity: 1, unit: "tbsp" },
      { id: "41", name: "Maple syrup", quantity: 1, unit: "tbsp" },
      { id: "42", name: "Vanilla extract", quantity: 0.5, unit: "tsp" },
      { id: "43", name: "Mixed berries", quantity: 100, unit: "g" },
      { id: "44", name: "Almond butter", quantity: 1, unit: "tbsp" },
    ],
    instructions: [
      "Combine oats, yogurt, milk, and chia seeds in a jar or container",
      "Add maple syrup and vanilla, stir well to combine",
      "Cover and refrigerate overnight (or at least 4 hours)",
      "In the morning, stir and add more milk if desired for consistency",
      "Top with fresh berries and a drizzle of almond butter",
      "Enjoy cold or microwave for 2 minutes if you prefer it warm",
    ],
    prepTime: 5,
    cookTime: 0,
    servings: 1,
    nutrition: { calories: 420, protein: 18, carbs: 58, fat: 14 },
    tags: ["Breakfast", "Healthy", "Meal Prep"],
    createdAt: "2024-02-15",
  },
  {
    id: "6",
    name: "Grilled Salmon with Lemon Dill",
    description: "Perfectly grilled salmon fillets with a bright lemon and fresh dill finish. Heart-healthy and delicious.",
    ingredients: [
      { id: "45", name: "Salmon fillets", quantity: 4, unit: "", notes: "6 oz each" },
      { id: "46", name: "Olive oil", quantity: 2, unit: "tbsp" },
      { id: "47", name: "Lemons", quantity: 2, unit: "" },
      { id: "48", name: "Fresh dill", quantity: 3, unit: "tbsp" },
      { id: "49", name: "Garlic", quantity: 2, unit: "cloves" },
      { id: "50", name: "Salt", quantity: 1, unit: "tsp" },
      { id: "51", name: "Black pepper", quantity: 0.5, unit: "tsp" },
      { id: "52", name: "Asparagus", quantity: 400, unit: "g", notes: "for serving" },
    ],
    instructions: [
      "Remove salmon from fridge 20 minutes before cooking to bring to room temperature",
      "Preheat grill or grill pan to medium-high heat",
      "Pat salmon dry and brush with olive oil; season with salt and pepper",
      "Mince garlic and chop fresh dill; zest one lemon and juice both",
      "Grill salmon skin-side down for 4-5 minutes, flip and cook 3-4 more minutes",
      "Meanwhile, grill asparagus until tender-crisp (about 4 minutes)",
      "Mix lemon juice, zest, dill, and minced garlic; spoon over cooked salmon",
      "Serve immediately with grilled asparagus",
    ],
    prepTime: 10,
    cookTime: 15,
    servings: 4,
    nutrition: { calories: 380, protein: 42, carbs: 6, fat: 22 },
    tags: ["Seafood", "Healthy", "Grilling"],
    createdAt: "2024-02-20",
  },
];

function createRecipeStore() {
  const { subscribe, set, update } = writable<Recipe[]>(mockRecipes);

  return {
    subscribe,
    add: (recipe: Recipe) => update((recipes) => [...recipes, recipe]),
    remove: (id: string) => update((recipes) => recipes.filter((r) => r.id !== id)),
    update: (id: string, data: Partial<Recipe>) =>
      update((recipes) =>
        recipes.map((r) => (r.id === id ? { ...r, ...data } : r))
      ),
  };
}

export const recipeStore = createRecipeStore();

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
