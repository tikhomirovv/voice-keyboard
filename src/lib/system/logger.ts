import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";

/**
 * Logger singleton object that provides extended logging functionality
 */
const Logger = {
  /**
   * Extracts caller file and line information from Error stack
   * @private
   */
  getCallerInfo(): string {
    const error = new Error();
    const stack = error.stack?.split("\n");
    if (!stack || stack.length < 4) return "";

    // Get the caller line (index 3 because: 0 - Error, 1 - getCallerInfo, 2 - formatMessage, 3 - actual caller)
    const callerLine = stack[4];

    // Extract file path and line number using regex
    const match = callerLine.match(/at\s+(?:.*\s+\()?(.+):(\d+):(\d+)/);
    if (!match) return "";

    const [, file, line] = match;
    // Clean up the file path by removing URL-encoded parameters and decode it
    const cleanFile = decodeURIComponent(file.split("?")[0]);
    // Get only filename from full path
    const fileName = cleanFile.split(/[\\/]/).pop() || cleanFile;
    return `[${fileName}:${line}]`;
  },

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
    const callerInfo = this.getCallerInfo();
    const baseMessage =
      args.length === 0
        ? message
        : `${message} | ${args
            .map((arg) => this.convertToString(arg))
            .join(" | ")}`;
    return `${callerInfo} ${baseMessage}`;
  },

  /**
   * Log warning message with additional parameters
   */
  warn(message: string, ...args: any[]): void {
    warn(this.formatMessage(message, ...args));
    console.warn(message, ...args);
  },

  /**
   * Log debug message with additional parameters
   */
  debug(message: string, ...args: any[]): void {
    debug(this.formatMessage(message, ...args));
    console.debug(message, ...args);
  },

  /**
   * Log trace message with additional parameters
   */
  trace(message: string, ...args: any[]): void {
    trace(this.formatMessage(message, ...args));
    console.trace(message, ...args);
  },

  /**
   * Log info message with additional parameters
   */
  info(message: string, ...args: any[]): void {
    info(this.formatMessage(message, ...args));
    console.info(message, ...args);
  },

  /**
   * Log error message with additional parameters
   */
  error(message: string, ...args: any[]): void {
    error(this.formatMessage(message, ...args));
    console.error(message, ...args);
  },
} as const;

export default Logger;
