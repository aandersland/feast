import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";

import type { QuickList } from "$lib/types";

let currentLists: QuickList[] = [];
let currentLoading = false;
const listsSubscribers = new Set<(value: QuickList[]) => void>();
const loadingSubscribers = new Set<(value: boolean) => void>();

const mockLoad = vi.fn();
const mockAddList = vi.fn();
const mockRenameList = vi.fn();
const mockRemoveList = vi.fn();
const mockAddItem = vi.fn();
const mockRemoveItem = vi.fn();
const mockUpdateItem = vi.fn();

function setMockLists(lists: QuickList[]) {
  currentLists = lists;
  listsSubscribers.forEach((fn) => fn(currentLists));
}

function setMockLoading(loading: boolean) {
  currentLoading = loading;
  loadingSubscribers.forEach((fn) => fn(currentLoading));
}

vi.mock("$lib/stores", () => ({
  quickListsStore: {
    subscribe: (fn: (value: QuickList[]) => void) => {
      listsSubscribers.add(fn);
      fn(currentLists);
      return () => listsSubscribers.delete(fn);
    },
    load: () => mockLoad(),
    addList: (name: string) => mockAddList(name),
    renameList: (id: string, name: string) => mockRenameList(id, name),
    removeList: (id: string) => mockRemoveList(id),
    addItem: (listId: string, item: unknown) => mockAddItem(listId, item),
    removeItem: (listId: string, itemId: string) => mockRemoveItem(listId, itemId),
    updateItem: (listId: string, itemId: string, updates: unknown) =>
      mockUpdateItem(listId, itemId, updates),
  },
  quickListsLoading: {
    subscribe: (fn: (value: boolean) => void) => {
      loadingSubscribers.add(fn);
      fn(currentLoading);
      return () => loadingSubscribers.delete(fn);
    },
  },
}));

import QuickListsManager from "./QuickListsManager.svelte";

describe("QuickListsManager", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setMockLists([]);
    setMockLoading(false);
  });

  it("calls quickListsStore.load() on mount", () => {
    render(QuickListsManager);

    expect(mockLoad).toHaveBeenCalledTimes(1);
  });

  it("shows loading spinner when quickListsLoading is true", () => {
    setMockLoading(true);

    render(QuickListsManager);

    // Check for animate-spin class indicating the loading spinner
    const spinner = document.querySelector(".animate-spin");
    expect(spinner).toBeInTheDocument();
  });

  it("shows empty state when no lists and not loading", () => {
    setMockLoading(false);
    setMockLists([]);

    render(QuickListsManager);

    expect(screen.getByText("No Quick Lists Yet")).toBeInTheDocument();
    expect(
      screen.getByText("Create a quick list to easily add common items to your shopping list.")
    ).toBeInTheDocument();
    expect(screen.getByText("Create Your First List")).toBeInTheDocument();
  });

  it("renders list cards when lists exist", () => {
    const testLists: QuickList[] = [
      { id: "list-1", name: "Weekly Essentials", items: [] },
      { id: "list-2", name: "Pantry Staples", items: [] },
    ];
    setMockLists(testLists);

    render(QuickListsManager);

    // The QuickListCard components render the list names
    expect(screen.getByText("Weekly Essentials")).toBeInTheDocument();
    expect(screen.getByText("Pantry Staples")).toBeInTheDocument();
  });

  it("New List button opens modal", async () => {
    render(QuickListsManager);

    // Modal should not be open initially - check for the modal's title
    expect(screen.queryByText("Create New Quick List")).not.toBeInTheDocument();

    const newListButton = screen.getByRole("button", { name: /New List/i });
    await fireEvent.click(newListButton);

    // Modal should now be open
    expect(screen.getByText("Create New Quick List")).toBeInTheDocument();
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });

  it("modal submit calls quickListsStore.addList() with the name", async () => {
    render(QuickListsManager);

    // Open the modal
    const newListButton = screen.getByRole("button", { name: /New List/i });
    await fireEvent.click(newListButton);

    // Find the input and enter a name
    const input = screen.getByLabelText("List Name");
    await fireEvent.input(input, { target: { value: "Test List Name" } });

    // Submit the form via the Create List button
    const submitButton = screen.getByRole("button", { name: /Create List/i });
    await fireEvent.click(submitButton);

    expect(mockAddList).toHaveBeenCalledTimes(1);
    expect(mockAddList).toHaveBeenCalledWith("Test List Name");
  });

  it("Create Your First List button in empty state opens modal", async () => {
    setMockLists([]);

    render(QuickListsManager);

    // Modal should not be open initially
    expect(screen.queryByText("Create New Quick List")).not.toBeInTheDocument();

    const createButton = screen.getByText("Create Your First List");
    await fireEvent.click(createButton);

    // Modal should now be open
    expect(screen.getByText("Create New Quick List")).toBeInTheDocument();
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });
});
