import { writable } from "svelte/store";
import { log } from "$lib/logging";
import { getCurrentCorrelationId } from "$lib/tauri/tracing";

export type TabId = "dashboard" | "recipes" | "mealplan" | "quicklists";

function createActiveTabStore() {
  const { subscribe, set } = writable<TabId>("dashboard");

  return {
    subscribe,
    set: (tab: TabId) => {
      log.info("Tab changed", "store::navigation", { tab }, getCurrentCorrelationId());
      set(tab);
    },
  };
}

export const activeTab = createActiveTabStore();
