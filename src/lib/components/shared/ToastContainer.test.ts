import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";

interface Toast {
  id: string;
  type: "success" | "error" | "info";
  message: string;
  duration?: number;
}

let currentToasts: Toast[] = [];
const subscribers = new Set<(value: Toast[]) => void>();

const mockRemove = vi.fn();

function setMockToasts(toasts: Toast[]) {
  currentToasts = toasts;
  subscribers.forEach((fn) => fn(currentToasts));
}

vi.mock("$lib/stores/toast", () => ({
  toastStore: {
    subscribe: (fn: (value: Toast[]) => void) => {
      subscribers.add(fn);
      fn(currentToasts);
      return () => subscribers.delete(fn);
    },
    remove: (id: string) => mockRemove(id),
  },
}));

import ToastContainer from "./ToastContainer.svelte";

describe("ToastContainer", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setMockToasts([]);
  });

  it("renders nothing when no toasts in store", () => {
    render(ToastContainer);

    const buttons = screen.queryAllByRole("button");
    expect(buttons).toHaveLength(0);
  });

  it("renders multiple toasts from store", () => {
    setMockToasts([
      { id: "1", type: "success", message: "First toast" },
      { id: "2", type: "error", message: "Second toast" },
      { id: "3", type: "info", message: "Third toast" },
    ]);

    render(ToastContainer);

    expect(screen.getByText("First toast")).toBeInTheDocument();
    expect(screen.getByText("Second toast")).toBeInTheDocument();
    expect(screen.getByText("Third toast")).toBeInTheDocument();
  });

  it("success toast has bg-emerald-600 class", () => {
    setMockToasts([{ id: "1", type: "success", message: "Success message" }]);

    render(ToastContainer);

    const toast = screen.getByText("Success message").closest("div");
    expect(toast).toHaveClass("bg-emerald-600");
  });

  it("error toast has bg-red-600 class", () => {
    setMockToasts([{ id: "1", type: "error", message: "Error message" }]);

    render(ToastContainer);

    const toast = screen.getByText("Error message").closest("div");
    expect(toast).toHaveClass("bg-red-600");
  });

  it("info toast has bg-blue-600 class", () => {
    setMockToasts([{ id: "1", type: "info", message: "Info message" }]);

    render(ToastContainer);

    const toast = screen.getByText("Info message").closest("div");
    expect(toast).toHaveClass("bg-blue-600");
  });

  it("close button calls toastStore.remove(id) when clicked", async () => {
    setMockToasts([{ id: "toast-123", type: "success", message: "Test toast" }]);

    render(ToastContainer);

    const closeButton = screen.getByRole("button");
    await fireEvent.click(closeButton);

    expect(mockRemove).toHaveBeenCalledTimes(1);
    expect(mockRemove).toHaveBeenCalledWith("toast-123");
  });
});
