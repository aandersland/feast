import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import ConfirmDialog from "./ConfirmDialog.svelte";

describe("ConfirmDialog", () => {
  const defaultProps = {
    open: true,
    title: "Test Title",
    message: "Test message",
    onConfirm: vi.fn(),
    onCancel: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("does not render when open=false", () => {
    render(ConfirmDialog, {
      props: { ...defaultProps, open: false },
    });

    expect(screen.queryByRole("dialog")).toBeNull();
  });

  it("renders when open=true", () => {
    render(ConfirmDialog, {
      props: defaultProps,
    });

    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });

  it("displays custom title and message", () => {
    render(ConfirmDialog, {
      props: {
        ...defaultProps,
        title: "Delete Item",
        message: "Are you sure you want to delete this?",
      },
    });

    expect(screen.getByText("Delete Item")).toBeInTheDocument();
    expect(
      screen.getByText("Are you sure you want to delete this?")
    ).toBeInTheDocument();
  });

  it("uses default button labels when not provided", () => {
    render(ConfirmDialog, {
      props: defaultProps,
    });

    expect(
      screen.getByRole("button", { name: /Confirm/i })
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Cancel/i })).toBeInTheDocument();
  });

  it("uses custom button labels when provided", () => {
    render(ConfirmDialog, {
      props: {
        ...defaultProps,
        confirmLabel: "Delete",
        cancelLabel: "Keep",
      },
    });

    expect(screen.getByRole("button", { name: /Delete/i })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Keep/i })).toBeInTheDocument();
  });

  it("shows red confirm button when destructive=true", () => {
    render(ConfirmDialog, {
      props: {
        ...defaultProps,
        destructive: true,
      },
    });

    const confirmButton = screen.getByRole("button", { name: /Confirm/i });
    expect(confirmButton).toHaveClass("bg-red-600");
  });

  it("shows green confirm button when destructive=false", () => {
    render(ConfirmDialog, {
      props: {
        ...defaultProps,
        destructive: false,
      },
    });

    const confirmButton = screen.getByRole("button", { name: /Confirm/i });
    expect(confirmButton).toHaveClass("bg-emerald-600");
  });

  it("calls onConfirm callback when confirm button is clicked", async () => {
    const onConfirm = vi.fn();
    render(ConfirmDialog, {
      props: {
        ...defaultProps,
        onConfirm,
      },
    });

    const confirmButton = screen.getByRole("button", { name: /Confirm/i });
    await fireEvent.click(confirmButton);

    expect(onConfirm).toHaveBeenCalledTimes(1);
  });

  it("calls onCancel callback when cancel button is clicked", async () => {
    const onCancel = vi.fn();
    render(ConfirmDialog, {
      props: {
        ...defaultProps,
        onCancel,
      },
    });

    const cancelButton = screen.getByRole("button", { name: /Cancel/i });
    await fireEvent.click(cancelButton);

    expect(onCancel).toHaveBeenCalledTimes(1);
  });
});
