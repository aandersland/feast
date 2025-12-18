/**
 * Tauri command tracing utilities
 *
 * Provides correlation ID generation and traced invoke wrapper
 * for request tracing through the full stack.
 */

import { invoke } from "@tauri-apps/api/core";
import { log } from "$lib/logging";

/**
 * Generate a correlation ID for request tracing
 *
 * Returns an 8-character alphanumeric string.
 * Uses crypto.randomUUID() and takes first 8 chars for brevity.
 */
export function generateCorrelationId(): string {
  // Use crypto API for randomness, take first 8 chars (no hyphens)
  return crypto.randomUUID().replace(/-/g, "").substring(0, 8);
}

/**
 * Current correlation ID context
 *
 * Stored in module scope to allow sharing across related operations.
 * Reset to undefined after each command completes.
 */
let currentCorrelationId: string | undefined;

/**
 * Get the current correlation ID, if any
 */
export function getCurrentCorrelationId(): string | undefined {
  return currentCorrelationId;
}

/**
 * Set the current correlation ID context
 *
 * Use this to establish a correlation context for a series of operations.
 */
export function setCurrentCorrelationId(id: string | undefined): void {
  currentCorrelationId = id;
}

/**
 * Invoke a Tauri command with automatic correlation ID
 *
 * Generates a correlation ID and passes it as `correlationId` parameter.
 * The ID is also set as the current context for related logging calls.
 *
 * @param cmd - The Tauri command name
 * @param args - Command arguments (correlationId will be added)
 * @returns Promise resolving to command result
 *
 * @example
 * ```typescript
 * const recipe = await tracedInvoke<Recipe>("get_recipe", { id: "123" });
 * // Backend receives: { id: "123", correlationId: "V1StGXR8" }
 * ```
 */
export async function tracedInvoke<T>(
  cmd: string,
  args?: Record<string, unknown>
): Promise<T> {
  const correlationId = generateCorrelationId();

  // Set context for any logging during this operation
  const previousId = currentCorrelationId;
  currentCorrelationId = correlationId;

  const start = performance.now();
  log.debug(`IPC ${cmd} started`, "tracing", { cmd }, correlationId);

  try {
    const result = await invoke<T>(cmd, {
      ...args,
      correlationId,
    });
    const duration = performance.now() - start;
    log.info(`IPC ${cmd} succeeded`, "tracing", { cmd, durationMs: duration.toFixed(2) }, correlationId);
    return result;
  } catch (error) {
    const duration = performance.now() - start;
    log.error(`IPC ${cmd} failed`, "tracing", {
      cmd,
      durationMs: duration.toFixed(2),
      error: error instanceof Error ? error.message : String(error)
    }, correlationId);
    throw error;
  } finally {
    // Restore previous context (supports nested calls)
    currentCorrelationId = previousId;
  }
}

/**
 * Type helper for traced command functions
 */
export type TracedCommand<TArgs, TResult> = (
  args: TArgs & { correlationId?: string }
) => Promise<TResult>;
