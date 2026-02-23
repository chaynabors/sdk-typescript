declare module 'filament:core/config' {
  export function getString(path: string): string;
  export function getInt(path: string): bigint;
  export function getFloat(path: string): number;
  export function getBool(path: string): boolean;
  export function getOffsetDateTime(path: string): OffsetDateTime;
  export function getLocalDateTime(path: string): LocalDateTime;
  export function getLocalDate(path: string): Date;
  export function getLocalTime(path: string): Time;
  export function getStringArray(path: string): Array<string>;
  export function getIntArray(path: string): BigInt64Array;
  export function getFloatArray(path: string): Float64Array;
  export function getBoolArray(path: string): Array<boolean>;
  export function getOffsetDateTimeArray(path: string): Array<OffsetDateTime>;
  export function getLocalDateTimeArray(path: string): Array<LocalDateTime>;
  export function getLocalDateArray(path: string): Array<Date>;
  export function getLocalTimeArray(path: string): Array<Time>;
  export function keys(path: string | undefined): Array<string>;
  export type Offset = OffsetZ | OffsetCustom;
  export interface OffsetZ {
    tag: 'z',
  }
  export interface OffsetCustom {
    tag: 'custom',
    val: number,
  }
  export interface Date {
    year: number,
    month: number,
    day: number,
  }
  export interface Time {
    hour: number,
    minute: number,
    second: number,
    nanosecond: number,
  }
  export interface OffsetDateTime {
    offset: Offset,
    date: Date,
    time: Time,
  }
  export interface LocalDateTime {
    date: Date,
    time: Time,
  }
  export type ConfigError = ConfigErrorNotFound | ConfigErrorTypeMismatch;
  export interface ConfigErrorNotFound {
    tag: 'not-found',
  }
  export interface ConfigErrorTypeMismatch {
    tag: 'type-mismatch',
    val: [string, string],
  }
}
