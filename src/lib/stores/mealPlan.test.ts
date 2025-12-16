import { describe, it, expect, vi, beforeEach } from "vitest";
import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core");

describe("mealPlanStore", () => {
  // Re-import fresh module for each test to reset store state
  let mealPlanStore: typeof import("./mealPlan").mealPlanStore;
  let mealPlansLoading: typeof import("./mealPlan").mealPlansLoading;
  let mealPlansError: typeof import("./mealPlan").mealPlansError;
  let mealPlanByDate: typeof import("./mealPlan").mealPlanByDate;

  beforeEach(async () => {
    vi.clearAllMocks();
    vi.resetModules();
    const module = await import("./mealPlan");
    mealPlanStore = module.mealPlanStore;
    mealPlansLoading = module.mealPlansLoading;
    mealPlansError = module.mealPlansError;
    mealPlanByDate = module.mealPlanByDate;
  });

  it("loads and groups meal plans by date", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      { id: "m1", date: "2025-01-01", mealType: "dinner", recipeId: "r1", servings: 4, createdAt: "2025-01-01" },
      { id: "m2", date: "2025-01-01", mealType: "lunch", recipeId: "r2", servings: 2, createdAt: "2025-01-01" },
      { id: "m3", date: "2025-01-02", mealType: "dinner", recipeId: "r1", servings: 4, createdAt: "2025-01-01" },
    ]);

    await mealPlanStore.load("2025-01-01", "2025-01-07");

    expect(invoke).toHaveBeenCalledWith("get_meal_plans", { startDate: "2025-01-01", endDate: "2025-01-07" });

    const plans = get(mealPlanStore);
    expect(plans).toHaveLength(2);
    expect(plans[0].date).toBe("2025-01-01");
    expect(plans[0].meals).toHaveLength(2);
    expect(plans[1].date).toBe("2025-01-02");
    expect(plans[1].meals).toHaveLength(1);
    expect(get(mealPlansLoading)).toBe(false);
  });

  it("sets error on load failure", async () => {
    vi.mocked(invoke).mockRejectedValueOnce(new Error("Network error"));

    await mealPlanStore.load("2025-01-01", "2025-01-07");

    expect(get(mealPlansError)).toBe("Network error");
    expect(get(mealPlanStore)).toEqual([]);
  });

  it("adds meal to existing date", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce([
        { id: "m1", date: "2025-01-01", mealType: "dinner", recipeId: "r1", servings: 4, createdAt: "2025-01-01" },
      ])
      .mockResolvedValueOnce({
        id: "m2",
        date: "2025-01-01",
        mealType: "lunch",
        recipeId: "r2",
        servings: 2,
        createdAt: "2025-01-01",
      });

    await mealPlanStore.load("2025-01-01", "2025-01-07");
    await mealPlanStore.addMeal("2025-01-01", "r2", "lunch", 2);

    expect(invoke).toHaveBeenCalledWith("create_meal_plan", {
      input: { date: "2025-01-01", mealType: "lunch", recipeId: "r2", servings: 2 },
    });

    const plans = get(mealPlanStore);
    expect(plans[0].meals).toHaveLength(2);
  });

  it("adds meal to new date", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce([]) // Empty initial load
      .mockResolvedValueOnce({
        id: "m1",
        date: "2025-01-01",
        mealType: "dinner",
        recipeId: "r1",
        servings: 4,
        createdAt: "2025-01-01",
      });

    await mealPlanStore.load("2025-01-01", "2025-01-07");
    await mealPlanStore.addMeal("2025-01-01", "r1", "dinner", 4);

    const plans = get(mealPlanStore);
    expect(plans).toHaveLength(1);
    expect(plans[0].date).toBe("2025-01-01");
    expect(plans[0].meals).toHaveLength(1);
  });

  it("removes meal and cleans up empty dates", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce([
        { id: "m1", date: "2025-01-01", mealType: "dinner", recipeId: "r1", servings: 4, createdAt: "2025-01-01" },
      ])
      .mockResolvedValueOnce(undefined);

    await mealPlanStore.load("2025-01-01", "2025-01-07");
    await mealPlanStore.removeMeal("2025-01-01", "m1");

    expect(invoke).toHaveBeenCalledWith("delete_meal_plan", { id: "m1" });
    expect(get(mealPlanStore)).toHaveLength(0);
  });

  it("updates meal servings", async () => {
    vi.mocked(invoke)
      .mockResolvedValueOnce([
        { id: "m1", date: "2025-01-01", mealType: "dinner", recipeId: "r1", servings: 4, createdAt: "2025-01-01" },
      ])
      .mockResolvedValueOnce({
        id: "m1",
        date: "2025-01-01",
        mealType: "dinner",
        recipeId: "r1",
        servings: 6,
        createdAt: "2025-01-01",
      });

    await mealPlanStore.load("2025-01-01", "2025-01-07");
    await mealPlanStore.updateServings("2025-01-01", "m1", 6);

    expect(invoke).toHaveBeenCalledWith("update_meal_plan", { id: "m1", servings: 6 });

    const plans = get(mealPlanStore);
    expect(plans[0].meals[0].servings).toBe(6);
  });

  it("mealPlanByDate creates lookup map", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      { id: "m1", date: "2025-01-01", mealType: "dinner", recipeId: "r1", servings: 4, createdAt: "2025-01-01" },
    ]);

    await mealPlanStore.load("2025-01-01", "2025-01-07");

    const map = get(mealPlanByDate);
    expect(map.has("2025-01-01")).toBe(true);
    expect(map.has("2025-01-02")).toBe(false);
  });
});
