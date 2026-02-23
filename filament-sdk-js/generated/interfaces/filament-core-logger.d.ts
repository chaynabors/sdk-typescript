declare module 'filament:core/logger' {
  export function log(level: Level, message: string): void;
  /**
   * # Variants
   * 
   * ## `"debug"`
   * 
   * ## `"info"`
   * 
   * ## `"warn"`
   * 
   * ## `"error"`
   */
  export type Level = 'debug' | 'info' | 'warn' | 'error';
}
