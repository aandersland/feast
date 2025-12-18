import { writable } from "svelte/store";
import { log } from "$lib/logging";
import { getCurrentCorrelationId } from "$lib/tauri/tracing";

interface AppState {
  appName: string;
  version: string;
}

function createAppStore() {
  const { subscribe, set, update } = writable<AppState>({
    appName: "feast",
    version: "0.1.0",
  });

  return {
    subscribe,
    setVersion: (version: string) => {
      log.info("App version set", "store::app", { version }, getCurrentCorrelationId());
      update((state) => ({ ...state, version }));
    },
  };
}

export const appStore = createAppStore();
