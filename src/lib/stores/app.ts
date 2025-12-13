import { writable } from "svelte/store";

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
    setVersion: (version: string) => update((state) => ({ ...state, version })),
  };
}

export const appStore = createAppStore();
