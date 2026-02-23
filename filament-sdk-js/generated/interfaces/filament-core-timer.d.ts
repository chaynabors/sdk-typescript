/// <reference path="./filament-core-types.d.ts" />
declare module 'filament:core/timer' {
  export function arm(args: ArmArgs): TimerId;
  export function disarm(args: DisarmArgs): void;
  export type Error = import('filament:core/types').Error;
  export type TimerId = import('filament:core/types').TimerId;
  export interface ArmArgs {
    nanos: bigint,
  }
  export interface DisarmArgs {
    id: TimerId,
  }
}
