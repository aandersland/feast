import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { get } from "svelte/store";

describe("toastStore", () => {
  let toastStore: typeof import("./toast").toastStore;
  let uuidCounter = 0;

  beforeEach(async () => {
    vi.resetModules();
    vi.useFakeTimers();
    uuidCounter = 0;
    vi.stubGlobal("crypto", {
      randomUUID: vi.fn(() => `uuid-${++uuidCounter}`),
    });
    const module = await import("./toast");
    toastStore = module.toastStore;
  });

  afterEach(() => {
    vi.clearAllTimers();
    vi.useRealTimers();
    vi.unstubAllGlobals();
  });

  it("success() adds toast with type 'success'", () => {
    toastStore.success("Operation completed");

    const toasts = get(toastStore);
    expect(toasts).toHaveLength(1);
    expect(toasts[0].type).toBe("success");
    expect(toasts[0].message).toBe("Operation completed");
  });

  it("error() adds toast with type 'error' and 6000ms duration", () => {
    toastStore.error("Something went wrong");

    const toasts = get(toastStore);
    expect(toasts).toHaveLength(1);
    expect(toasts[0].type).toBe("error");
    expect(toasts[0].message).toBe("Something went wrong");
    expect(toasts[0].duration).toBe(6000);
  });

  it("info() adds toast with type 'info'", () => {
    toastStore.info("Information message");

    const toasts = get(toastStore);
    expect(toasts).toHaveLength(1);
    expect(toasts[0].type).toBe("info");
    expect(toasts[0].message).toBe("Information message");
  });

  it("remove(id) removes specific toast", () => {
    const id1 = toastStore.success("First");
    const id2 = toastStore.info("Second");

    expect(get(toastStore)).toHaveLength(2);

    toastStore.remove(id1);

    const toasts = get(toastStore);
    expect(toasts).toHaveLength(1);
    expect(toasts[0].id).toBe(id2);
  });

  it("auto-removes toast after default 4000ms", () => {
    toastStore.success("Will disappear");

    expect(get(toastStore)).toHaveLength(1);

    vi.advanceTimersByTime(3999);
    expect(get(toastStore)).toHaveLength(1);

    vi.advanceTimersByTime(1);
    expect(get(toastStore)).toHaveLength(0);
  });

  it("auto-removes error toast after 6000ms", () => {
    toastStore.error("Error will disappear");

    expect(get(toastStore)).toHaveLength(1);

    vi.advanceTimersByTime(4000);
    expect(get(toastStore)).toHaveLength(1);

    vi.advanceTimersByTime(1999);
    expect(get(toastStore)).toHaveLength(1);

    vi.advanceTimersByTime(1);
    expect(get(toastStore)).toHaveLength(0);
  });

  it("multiple toasts maintain insertion order", () => {
    toastStore.success("First");
    toastStore.info("Second");
    toastStore.error("Third");

    const toasts = get(toastStore);
    expect(toasts).toHaveLength(3);
    expect(toasts[0].message).toBe("First");
    expect(toasts[1].message).toBe("Second");
    expect(toasts[2].message).toBe("Third");
  });
});
