import { writable } from "svelte/store";

export interface Toast {
  id: string;
  type: "success" | "error" | "info";
  message: string;
  duration?: number;
}

const { subscribe, update } = writable<Toast[]>([]);

function addToast(toast: Omit<Toast, "id">) {
  const id = crypto.randomUUID();
  update((toasts) => [...toasts, { ...toast, id }]);

  const duration = toast.duration ?? 4000;
  if (duration > 0) {
    setTimeout(() => removeToast(id), duration);
  }
  return id;
}

function removeToast(id: string) {
  update((toasts) => toasts.filter((t) => t.id !== id));
}

export const toastStore = {
  subscribe,
  success: (message: string) => addToast({ type: "success", message }),
  error: (message: string) => addToast({ type: "error", message, duration: 6000 }),
  info: (message: string) => addToast({ type: "info", message }),
  remove: removeToast,
};
