import { writable } from "svelte/store";
import { vi } from "vitest";

/**
 * Creates a mock Svelte store with test utilities
 */
export function createMockStore<T>(initialValue: T) {
  const store = writable(initialValue);
  return {
    subscribe: store.subscribe,
    _set: store.set,
    _update: store.update,
  };
}

/**
 * Creates a mock async store (like recipeStore) with common CRUD methods
 */
export function createMockAsyncStore<T>(initialValue: T) {
  const store = createMockStore(initialValue);
  return {
    ...store,
    load: vi.fn().mockResolvedValue(undefined),
    add: vi.fn(),
    remove: vi.fn(),
    update: vi.fn(),
  };
}

/**
 * Waits for all pending promises to resolve
 */
export async function flushPromises() {
  await new Promise(resolve => setTimeout(resolve, 0));
}
