import { invoke } from "@tauri-apps/api/core";
import type { Item } from "$lib/types";

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
