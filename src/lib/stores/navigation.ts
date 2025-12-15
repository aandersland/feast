import { writable } from "svelte/store";

export type TabId = "dashboard" | "recipes" | "mealplan" | "quicklists";

export const activeTab = writable<TabId>("dashboard");
