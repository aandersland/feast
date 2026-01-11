import { writable, get } from "svelte/store";
import { log } from "$lib/logging";
import { getCurrentCorrelationId } from "$lib/tauri/tracing";

export type TabId = "dashboard" | "recipes" | "mealplan" | "quicklists";

// Well-known scroll target IDs (prevents magic string duplication)
export const SCROLL_TARGETS = {
  SHOPPING_SECTION: "shopping-section",
} as const;

// Store for scroll targets after navigation
export const scrollTarget = writable<string | null>(null);

function createActiveTabStore() {
  const { subscribe, set } = writable<TabId>("dashboard");

  return {
    subscribe,
    set: (tab: TabId, options?: { scrollTo?: string }) => {
      log.info("Tab changed", "store::navigation", { tab, scrollTo: options?.scrollTo }, getCurrentCorrelationId());
      // Always clear any existing scroll target to prevent orphaned targets
      // Then set the new one if provided
      scrollTarget.set(options?.scrollTo ?? null);
      set(tab);
    },
  };
}

export const activeTab = createActiveTabStore();

// Helper to consume and clear the scroll target
export function consumeScrollTarget(): string | null {
  const target = get(scrollTarget);
  scrollTarget.set(null);
  return target;
}
