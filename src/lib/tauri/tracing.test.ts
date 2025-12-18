import { describe, it, expect, vi, beforeEach } from "vitest";
import { tracedInvoke } from "./tracing";
import { log } from "$lib/logging";

vi.mock("$lib/logging", () => ({
  log: {
    debug: vi.fn(),
    info: vi.fn(),
    error: vi.fn(),
  },
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("tracedInvoke logging", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("logs IPC start and success with timing", async () => {
    const { invoke } = await import("@tauri-apps/api/core");
    vi.mocked(invoke).mockResolvedValue({ data: "test" });

    await tracedInvoke("test_command", { arg: 1 });

    expect(log.debug).toHaveBeenCalledWith(
      expect.stringContaining("started"),
      "tracing",
      expect.objectContaining({ cmd: "test_command" }),
      expect.any(String)
    );
    expect(log.info).toHaveBeenCalledWith(
      expect.stringContaining("succeeded"),
      "tracing",
      expect.objectContaining({ cmd: "test_command", durationMs: expect.any(String) }),
      expect.any(String)
    );
  });

  it("logs IPC failure with error details", async () => {
    const { invoke } = await import("@tauri-apps/api/core");
    vi.mocked(invoke).mockRejectedValue(new Error("Connection failed"));

    await expect(tracedInvoke("test_command")).rejects.toThrow();

    expect(log.error).toHaveBeenCalledWith(
      expect.stringContaining("failed"),
      "tracing",
      expect.objectContaining({
        cmd: "test_command",
        error: "Connection failed",
      }),
      expect.any(String)
    );
  });

  it("generates unique correlation IDs for each call", async () => {
    const { invoke } = await import("@tauri-apps/api/core");
    vi.mocked(invoke).mockResolvedValue({});

    await tracedInvoke("cmd1");
    await tracedInvoke("cmd2");

    const debugCalls = vi.mocked(log.debug).mock.calls;
    expect(debugCalls.length).toBe(2);

    // Extract correlation IDs (4th argument)
    const correlationId1 = debugCalls[0][3];
    const correlationId2 = debugCalls[1][3];

    expect(correlationId1).toBeTruthy();
    expect(correlationId2).toBeTruthy();
    expect(correlationId1).not.toBe(correlationId2);
  });

  it("passes correlation ID to backend invoke call", async () => {
    const { invoke } = await import("@tauri-apps/api/core");
    vi.mocked(invoke).mockResolvedValue({});

    await tracedInvoke("test_command", { foo: "bar" });

    expect(invoke).toHaveBeenCalledWith("test_command", {
      foo: "bar",
      correlationId: expect.any(String),
    });
  });
});
