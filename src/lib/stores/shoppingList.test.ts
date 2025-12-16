import { describe, it, expect, vi, beforeEach } from "vitest";
import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import {
  manualItemsStore,
  weeklyShoppingListsStore,
  quickListsStore,
  manualItemsLoading,
  shoppingListsLoading,
  quickListsLoading,
} from "./shoppingList";

vi.mock("@tauri-apps/api/core");

// Mock toast store
vi.mock("./toast", () => ({
  toastStore: {
    success: vi.fn(),
    error: vi.fn(),
    info: vi.fn(),
  },
}));

// Mock dependent stores
vi.mock("./mealPlan", () => ({
  mealPlanStore: {
    subscribe: vi.fn((fn) => {
      fn([]);
      return () => {};
    }),
  },
}));

vi.mock("./recipes", () => ({
  recipeById: {
    subscribe: vi.fn((fn) => {
      fn(new Map());
      return () => {};
    }),
  },
}));

describe("manualItemsStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("loads manual items and maps isChecked to isOnHand", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "1",
        weekStart: "2025-01-01",
        name: "Milk",
        quantity: 1,
        unit: "gallon",
        category: "Dairy",
        isChecked: true,
        createdAt: "",
      },
    ]);

    await manualItemsStore.load("2025-01-01");

    const items = get(manualItemsStore);
    expect(items[0].isOnHand).toBe(true);
    expect(items[0].isManual).toBe(true);
    expect(invoke).toHaveBeenCalledWith("get_manual_items", { weekStart: "2025-01-01" });
  });

  it("adds manual item via backend", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([]); // Initial load
    await manualItemsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce({
      id: "2",
      weekStart: "2025-01-01",
      name: "Bread",
      quantity: 1,
      unit: "loaf",
      category: "Bakery",
      isChecked: false,
      createdAt: "",
    });

    await manualItemsStore.add("2025-01-01", {
      name: "Bread",
      quantity: 1,
      unit: "loaf",
      category: "Bakery",
      isOnHand: false,
    });

    expect(invoke).toHaveBeenCalledWith("create_manual_item", {
      input: {
        weekStart: "2025-01-01",
        name: "Bread",
        quantity: 1,
        unit: "loaf",
        category: "Bakery",
      },
    });

    const items = get(manualItemsStore);
    expect(items).toHaveLength(1);
    expect(items[0].name).toBe("Bread");
  });

  it("removes manual item via backend", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "1",
        weekStart: "2025-01-01",
        name: "Milk",
        quantity: 1,
        unit: "gallon",
        category: "Dairy",
        isChecked: false,
        createdAt: "",
      },
    ]);
    await manualItemsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce(undefined);
    await manualItemsStore.remove("1");

    expect(invoke).toHaveBeenCalledWith("delete_manual_item", { id: "1" });

    const items = get(manualItemsStore);
    expect(items).toHaveLength(0);
  });

  it("toggles isOnHand via backend isChecked", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "1",
        weekStart: "2025-01-01",
        name: "Milk",
        quantity: 1,
        unit: "gallon",
        category: "Dairy",
        isChecked: false,
        createdAt: "",
      },
    ]);
    await manualItemsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce({ id: "1", isChecked: true });
    await manualItemsStore.toggleOnHand("1");

    expect(invoke).toHaveBeenCalledWith("update_manual_item", {
      id: "1",
      quantity: undefined,
      isChecked: true,
    });

    const items = get(manualItemsStore);
    expect(items[0].isOnHand).toBe(true);
  });

  it("updates quantity via backend", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "1",
        weekStart: "2025-01-01",
        name: "Milk",
        quantity: 1,
        unit: "gallon",
        category: "Dairy",
        isChecked: false,
        createdAt: "",
      },
    ]);
    await manualItemsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce({ id: "1", quantity: 2 });
    await manualItemsStore.updateQuantity("1", 2);

    expect(invoke).toHaveBeenCalledWith("update_manual_item", {
      id: "1",
      quantity: 2,
      isChecked: undefined,
    });

    const items = get(manualItemsStore);
    expect(items[0].quantity).toBe(2);
  });

  it("sets loading state during load", async () => {
    let loadingDuringFetch = false;
    vi.mocked(invoke).mockImplementationOnce(async () => {
      loadingDuringFetch = get(manualItemsLoading);
      return [];
    });

    await manualItemsStore.load("2025-01-01");

    expect(loadingDuringFetch).toBe(true);
    expect(get(manualItemsLoading)).toBe(false);
  });
});

describe("weeklyShoppingListsStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("loads and transforms shopping lists", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "list1",
        weekStart: "2025-01-01",
        name: "Weekly",
        listType: "weekly",
        createdAt: "",
        items: [
          {
            id: "item1",
            listId: "list1",
            name: "Eggs",
            quantity: 12,
            unit: "count",
            category: "Dairy",
            isChecked: true,
            isDeleted: false,
            createdAt: "",
          },
        ],
      },
    ]);

    await weeklyShoppingListsStore.load("2025-01-01");

    expect(invoke).toHaveBeenCalledWith("get_shopping_lists", { weekStart: "2025-01-01" });
  });

  it("creates a new custom list", async () => {
    // First load existing lists
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "list1",
        weekStart: "2025-01-01",
        name: "Weekly",
        listType: "weekly",
        createdAt: "",
        items: [],
      },
    ]);
    await weeklyShoppingListsStore.load("2025-01-01");

    // Then create new list
    vi.mocked(invoke).mockResolvedValueOnce({
      id: "list2",
      weekStart: "2025-01-01",
      name: "Custom List",
      listType: "custom",
      createdAt: "",
    });
    await weeklyShoppingListsStore.addList("2025-01-01", "Custom List");

    expect(invoke).toHaveBeenCalledWith("create_shopping_list", {
      input: {
        weekStart: "2025-01-01",
        name: "Custom List",
        listType: "custom",
      },
    });
  });

  it("deletes a list", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "list1",
        weekStart: "2025-01-01",
        name: "Weekly",
        listType: "weekly",
        createdAt: "",
        items: [],
      },
      {
        id: "list2",
        weekStart: "2025-01-01",
        name: "Custom",
        listType: "custom",
        createdAt: "",
        items: [],
      },
    ]);
    await weeklyShoppingListsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce(undefined);
    await weeklyShoppingListsStore.removeList("2025-01-01", "list2");

    expect(invoke).toHaveBeenCalledWith("delete_shopping_list", { id: "list2" });
  });

  it("adds item to a list", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "list1",
        weekStart: "2025-01-01",
        name: "Weekly",
        listType: "weekly",
        createdAt: "",
        items: [],
      },
    ]);
    await weeklyShoppingListsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce({
      id: "item1",
      listId: "list1",
      name: "Butter",
      quantity: 1,
      unit: "stick",
      category: "Dairy",
      isChecked: false,
      isDeleted: false,
      createdAt: "",
    });
    await weeklyShoppingListsStore.addItem("2025-01-01", "list1", {
      name: "Butter",
      quantity: 1,
      unit: "stick",
      category: "Dairy",
      isOnHand: false,
    });

    expect(invoke).toHaveBeenCalledWith("add_shopping_item", {
      input: {
        listId: "list1",
        name: "Butter",
        quantity: 1,
        unit: "stick",
        category: "Dairy",
      },
    });
  });

  it("toggles item on-hand status", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "list1",
        weekStart: "2025-01-01",
        name: "Weekly",
        listType: "weekly",
        createdAt: "",
        items: [
          {
            id: "item1",
            listId: "list1",
            name: "Eggs",
            quantity: 12,
            unit: "count",
            category: "Dairy",
            isChecked: false,
            isDeleted: false,
            createdAt: "",
          },
        ],
      },
    ]);
    await weeklyShoppingListsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce({ id: "item1", isChecked: true });
    await weeklyShoppingListsStore.toggleItemOnHand("2025-01-01", "list1", "item1");

    expect(invoke).toHaveBeenCalledWith("update_shopping_item", {
      id: "item1",
      quantity: undefined,
      isChecked: true,
    });
  });

  it("moves item between lists", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "list1",
        weekStart: "2025-01-01",
        name: "Weekly",
        listType: "weekly",
        createdAt: "",
        items: [
          {
            id: "item1",
            listId: "list1",
            name: "Eggs",
            quantity: 12,
            unit: "count",
            category: "Dairy",
            isChecked: false,
            isDeleted: false,
            createdAt: "",
          },
        ],
      },
      {
        id: "list2",
        weekStart: "2025-01-01",
        name: "Midweek",
        listType: "midweek",
        createdAt: "",
        items: [],
      },
    ]);
    await weeklyShoppingListsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce({ id: "item1", listId: "list2" });
    await weeklyShoppingListsStore.moveItem("2025-01-01", "list1", "list2", "item1");

    expect(invoke).toHaveBeenCalledWith("move_shopping_item", {
      id: "item1",
      toListId: "list2",
    });
  });

  it("soft deletes an item", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "list1",
        weekStart: "2025-01-01",
        name: "Weekly",
        listType: "weekly",
        createdAt: "",
        items: [
          {
            id: "item1",
            listId: "list1",
            name: "Eggs",
            quantity: 12,
            unit: "count",
            category: "Dairy",
            isChecked: false,
            isDeleted: false,
            createdAt: "",
          },
        ],
      },
    ]);
    await weeklyShoppingListsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce(undefined);
    await weeklyShoppingListsStore.softDeleteItem("2025-01-01", "list1", "item1");

    expect(invoke).toHaveBeenCalledWith("soft_delete_shopping_item", { id: "item1" });
  });

  it("restores a soft-deleted item", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "list1",
        weekStart: "2025-01-01",
        name: "Weekly",
        listType: "weekly",
        createdAt: "",
        items: [
          {
            id: "item1",
            listId: "list1",
            name: "Eggs",
            quantity: 12,
            unit: "count",
            category: "Dairy",
            isChecked: false,
            isDeleted: true,
            deletedAt: "2025-01-01T00:00:00Z",
            createdAt: "",
          },
        ],
      },
    ]);
    await weeklyShoppingListsStore.load("2025-01-01");

    vi.mocked(invoke).mockResolvedValueOnce({ id: "item1", isDeleted: false });
    await weeklyShoppingListsStore.restoreItem("2025-01-01", "list1", "item1");

    expect(invoke).toHaveBeenCalledWith("restore_shopping_item", { id: "item1" });
  });

  it("sets loading state during load", async () => {
    let loadingDuringFetch = false;
    vi.mocked(invoke).mockImplementationOnce(async () => {
      loadingDuringFetch = get(shoppingListsLoading);
      return [];
    });

    await weeklyShoppingListsStore.load("2025-01-01");

    expect(loadingDuringFetch).toBe(true);
    expect(get(shoppingListsLoading)).toBe(false);
  });
});

describe("quickListsStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("loads quick lists from backend", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "ql1",
        name: "Pantry Staples",
        createdAt: "",
        updatedAt: "",
        items: [
          {
            id: "qli1",
            quickListId: "ql1",
            name: "Olive oil",
            quantity: 1,
            unit: "bottle",
            category: "Oils",
          },
        ],
      },
    ]);

    await quickListsStore.load();

    expect(invoke).toHaveBeenCalledWith("get_quick_lists");

    const lists = get(quickListsStore);
    expect(lists).toHaveLength(1);
    expect(lists[0].name).toBe("Pantry Staples");
    expect(lists[0].items).toHaveLength(1);
  });

  it("creates a new quick list", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([]); // Initial load
    await quickListsStore.load();

    vi.mocked(invoke).mockResolvedValueOnce({
      id: "ql1",
      name: "New List",
      createdAt: "",
      updatedAt: "",
    });
    await quickListsStore.addList("New List");

    expect(invoke).toHaveBeenCalledWith("create_quick_list", { name: "New List" });

    const lists = get(quickListsStore);
    expect(lists).toHaveLength(1);
    expect(lists[0].name).toBe("New List");
  });

  it("removes a quick list", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "ql1",
        name: "To Delete",
        createdAt: "",
        updatedAt: "",
        items: [],
      },
    ]);
    await quickListsStore.load();

    vi.mocked(invoke).mockResolvedValueOnce(undefined);
    await quickListsStore.removeList("ql1");

    expect(invoke).toHaveBeenCalledWith("delete_quick_list", { id: "ql1" });

    const lists = get(quickListsStore);
    expect(lists).toHaveLength(0);
  });

  it("renames a quick list", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "ql1",
        name: "Old Name",
        createdAt: "",
        updatedAt: "",
        items: [],
      },
    ]);
    await quickListsStore.load();

    vi.mocked(invoke).mockResolvedValueOnce({ id: "ql1", name: "New Name" });
    await quickListsStore.renameList("ql1", "New Name");

    expect(invoke).toHaveBeenCalledWith("update_quick_list", { id: "ql1", name: "New Name" });

    const lists = get(quickListsStore);
    expect(lists[0].name).toBe("New Name");
  });

  it("adds item to a quick list", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "ql1",
        name: "Pantry",
        createdAt: "",
        updatedAt: "",
        items: [],
      },
    ]);
    await quickListsStore.load();

    vi.mocked(invoke).mockResolvedValueOnce({
      id: "qli1",
      quickListId: "ql1",
      name: "Salt",
      quantity: 1,
      unit: "container",
      category: "Spices",
    });
    await quickListsStore.addItem("ql1", {
      name: "Salt",
      quantity: 1,
      unit: "container",
      category: "Spices",
    });

    expect(invoke).toHaveBeenCalledWith("add_quick_list_item", {
      quickListId: "ql1",
      input: {
        name: "Salt",
        quantity: 1,
        unit: "container",
        category: "Spices",
      },
    });

    const lists = get(quickListsStore);
    expect(lists[0].items).toHaveLength(1);
    expect(lists[0].items[0].name).toBe("Salt");
  });

  it("removes item from a quick list", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "ql1",
        name: "Pantry",
        createdAt: "",
        updatedAt: "",
        items: [
          {
            id: "qli1",
            quickListId: "ql1",
            name: "Salt",
            quantity: 1,
            unit: "container",
            category: "Spices",
          },
        ],
      },
    ]);
    await quickListsStore.load();

    vi.mocked(invoke).mockResolvedValueOnce(undefined);
    await quickListsStore.removeItem("ql1", "qli1");

    expect(invoke).toHaveBeenCalledWith("remove_quick_list_item", { id: "qli1" });

    const lists = get(quickListsStore);
    expect(lists[0].items).toHaveLength(0);
  });

  it("updates item in a quick list", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "ql1",
        name: "Pantry",
        createdAt: "",
        updatedAt: "",
        items: [
          {
            id: "qli1",
            quickListId: "ql1",
            name: "Salt",
            quantity: 1,
            unit: "container",
            category: "Spices",
          },
        ],
      },
    ]);
    await quickListsStore.load();

    vi.mocked(invoke).mockResolvedValueOnce({
      id: "qli1",
      quickListId: "ql1",
      name: "Salt",
      quantity: 2,
      unit: "container",
      category: "Spices",
    });
    await quickListsStore.updateItem("ql1", "qli1", { quantity: 2 });

    expect(invoke).toHaveBeenCalledWith("update_quick_list_item", {
      id: "qli1",
      input: {
        name: "Salt",
        quantity: 2,
        unit: "container",
        category: "Spices",
      },
    });

    const lists = get(quickListsStore);
    expect(lists[0].items[0].quantity).toBe(2);
  });

  it("sets loading state during load", async () => {
    let loadingDuringFetch = false;
    vi.mocked(invoke).mockImplementationOnce(async () => {
      loadingDuringFetch = get(quickListsLoading);
      return [];
    });

    await quickListsStore.load();

    expect(loadingDuringFetch).toBe(true);
    expect(get(quickListsLoading)).toBe(false);
  });

  it("adds quick list items to shopping list via backend", async () => {
    vi.mocked(invoke).mockResolvedValueOnce([
      {
        id: "item1",
        listId: "sl1",
        name: "Olive oil",
        quantity: 1,
        unit: "bottle",
        category: "Oils",
        isChecked: false,
        isDeleted: false,
        createdAt: "",
      },
    ]);

    await quickListsStore.addToShoppingList("ql1", "sl1");

    expect(invoke).toHaveBeenCalledWith("add_quick_list_to_shopping", {
      quickListId: "ql1",
      shoppingListId: "sl1",
    });
  });
});
