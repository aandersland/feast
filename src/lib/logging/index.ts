/**
 * Frontend logging utility
 *
 * Provides structured logging that flows through the Rust backend,
 * enabling unified log filtering and analysis across the full stack.
 *
 * Features:
 * - Batching: Logs are batched and sent periodically to avoid UI blocking
 * - Queue fallback: Logs queue locally if backend isn't ready
 * - Console fallback: Logs also go to console in development
 * - Structured data: Attach arbitrary data to log entries
 *
 * Usage:
 * ```typescript
 * import { log } from '$lib/logging';
 *
 * log.info('User logged in', 'AuthStore', { userId: 123 });
 * log.error('Failed to fetch recipes', 'RecipeList', { error: e.message });
 * ```
 */

import { logFromFrontend, type FrontendLogEntry } from "$lib/tauri/commands";
import { getCurrentCorrelationId } from "$lib/tauri/tracing";

type LogLevel = "trace" | "debug" | "info" | "warn" | "error";

interface LoggerConfig {
  /** Batch flush interval in milliseconds */
  flushInterval: number;
  /** Maximum batch size before forced flush */
  maxBatchSize: number;
  /** Whether to also log to console */
  consoleEnabled: boolean;
  /** Minimum level to log (trace < debug < info < warn < error) */
  minLevel: LogLevel;
}

const LOG_LEVEL_PRIORITY: Record<LogLevel, number> = {
  trace: 0,
  debug: 1,
  info: 2,
  warn: 3,
  error: 4,
};

const DEFAULT_CONFIG: LoggerConfig = {
  flushInterval: 1000, // 1 second
  maxBatchSize: 50,
  consoleEnabled: import.meta.env.DEV,
  minLevel: import.meta.env.DEV ? "debug" : "info",
};

class Logger {
  private queue: FrontendLogEntry[] = [];
  private flushTimer: ReturnType<typeof setTimeout> | null = null;
  private config: LoggerConfig;
  private isReady = false;
  private pendingFlush: Promise<void> | null = null;

  constructor(config: Partial<LoggerConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    // Mark as ready after a short delay to allow Tauri to initialize
    setTimeout(() => {
      this.isReady = true;
      this.scheduleFlush();
    }, 100);
  }

  /**
   * Log at trace level (most verbose)
   */
  trace(
    message: string,
    target = "frontend",
    data?: Record<string, unknown>,
    correlationId?: string
  ): void {
    this.log("trace", message, target, data, correlationId);
  }

  /**
   * Log at debug level
   */
  debug(
    message: string,
    target = "frontend",
    data?: Record<string, unknown>,
    correlationId?: string
  ): void {
    this.log("debug", message, target, data, correlationId);
  }

  /**
   * Log at info level
   */
  info(
    message: string,
    target = "frontend",
    data?: Record<string, unknown>,
    correlationId?: string
  ): void {
    this.log("info", message, target, data, correlationId);
  }

  /**
   * Log at warn level
   */
  warn(
    message: string,
    target = "frontend",
    data?: Record<string, unknown>,
    correlationId?: string
  ): void {
    this.log("warn", message, target, data, correlationId);
  }

  /**
   * Log at error level
   */
  error(
    message: string,
    target = "frontend",
    data?: Record<string, unknown>,
    correlationId?: string
  ): void {
    this.log("error", message, target, data, correlationId);
  }

  /**
   * Flush pending logs immediately
   * Useful before navigation or app close
   */
  async flush(): Promise<void> {
    if (this.pendingFlush) {
      await this.pendingFlush;
    }

    if (this.queue.length === 0) {
      return;
    }

    const entries = [...this.queue];
    this.queue = [];

    if (this.flushTimer) {
      clearTimeout(this.flushTimer);
      this.flushTimer = null;
    }

    this.pendingFlush = this.sendToBackend(entries);
    await this.pendingFlush;
    this.pendingFlush = null;
  }

  /**
   * Update logger configuration
   */
  configure(config: Partial<LoggerConfig>): void {
    this.config = { ...this.config, ...config };
  }

  private log(
    level: LogLevel,
    message: string,
    target: string,
    data?: Record<string, unknown>,
    correlationId?: string
  ): void {
    // Check minimum level
    if (LOG_LEVEL_PRIORITY[level] < LOG_LEVEL_PRIORITY[this.config.minLevel]) {
      return;
    }

    // Auto-fetch correlation ID from tracing context if not provided
    const cid = correlationId ?? getCurrentCorrelationId();

    // Format target with frontend prefix
    const fullTarget = target.startsWith("frontend::") ? target : `frontend::${target}`;

    const entry: FrontendLogEntry = {
      level,
      message,
      target: fullTarget,
      ...(cid ? { correlationId: cid } : {}),
      ...(data && Object.keys(data).length > 0 ? { data } : {}),
    };

    // Console output in development
    if (this.config.consoleEnabled) {
      this.logToConsole(level, message, fullTarget, data, cid);
    }

    // Queue for backend
    this.queue.push(entry);

    // Force flush if batch is full
    if (this.queue.length >= this.config.maxBatchSize) {
      void this.flush();
    } else {
      this.scheduleFlush();
    }
  }

  private logToConsole(
    level: LogLevel,
    message: string,
    target: string,
    data?: Record<string, unknown>,
    correlationId?: string
  ): void {
    const timestamp = new Date().toISOString();
    const cidPart = correlationId ? ` [${correlationId}]` : "";
    const prefix = `[${timestamp}] [${level.toUpperCase()}] [${target}]${cidPart}`;

    const consoleMethod = level === "error" ? "error" : level === "warn" ? "warn" : "log";

    if (data && Object.keys(data).length > 0) {
      console[consoleMethod](prefix, message, data);
    } else {
      console[consoleMethod](prefix, message);
    }
  }

  private scheduleFlush(): void {
    if (this.flushTimer || !this.isReady) {
      return;
    }

    this.flushTimer = setTimeout(() => {
      this.flushTimer = null;
      void this.flush();
    }, this.config.flushInterval);
  }

  private async sendToBackend(entries: FrontendLogEntry[]): Promise<void> {
    if (entries.length === 0) {
      return;
    }

    try {
      await logFromFrontend(entries);
    } catch (error) {
      // Backend not ready or call failed - log to console as fallback
      if (this.config.consoleEnabled) {
        console.warn("[Logger] Failed to send logs to backend:", error);
        // Re-log entries to console so they're not lost
        for (const entry of entries) {
          this.logToConsole(
            entry.level as LogLevel,
            entry.message,
            entry.target,
            entry.data as Record<string, unknown>
          );
        }
      }
    }
  }
}

// Export singleton instance
export const log = new Logger();

// Export class for testing or custom instances
export { Logger };
export type { LoggerConfig, LogLevel };
