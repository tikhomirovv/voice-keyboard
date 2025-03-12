import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";

/**
 * Logger singleton object that provides extended logging functionality
 */
const Logger = {
  /**
   * Converts any value to a string representation
   * @private
   */
  convertToString(value: any): string {
    if (typeof value === "object") {
      return JSON.stringify(value);
    }
    return String(value);
  },

  /**
   * Creates a formatted message from multiple parameters
   * @private
   */
  formatMessage(message: string, ...args: any[]): string {
    if (args.length === 0) return message;
    return `${message} | ${args
      .map((arg) => this.convertToString(arg))
      .join(" | ")}`;
  },

  /**
   * Log warning message with additional parameters
   */
  warn(message: string, ...args: any[]): void {
    warn(this.formatMessage(message, ...args));
  },

  /**
   * Log debug message with additional parameters
   */
  debug(message: string, ...args: any[]): void {
    debug(this.formatMessage(message, ...args));
  },

  /**
   * Log trace message with additional parameters
   */
  trace(message: string, ...args: any[]): void {
    trace(this.formatMessage(message, ...args));
  },

  /**
   * Log info message with additional parameters
   */
  info(message: string, ...args: any[]): void {
    info(this.formatMessage(message, ...args));
  },

  /**
   * Log error message with additional parameters
   */
  error(message: string, ...args: any[]): void {
    error(this.formatMessage(message, ...args));
  },
} as const;

export default Logger;
