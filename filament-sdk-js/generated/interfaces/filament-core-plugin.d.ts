/// <reference path="./filament-core-types.d.ts" />
declare module 'filament:core/plugin' {
  export type GuestBoundEvent = import('filament:core/types').GuestBoundEvent;
  export type TraceContext = import('filament:core/types').TraceContext;
  export type Error = import('filament:core/types').Error;
  export type TimerId = import('filament:core/types').TimerId;
  export interface TimerFired {
    timerId: TimerId,
    scheduledFor: bigint,
    skew: bigint,
  }
  export interface WeaveArgs {
    tick: bigint,
    virtualTime: bigint,
    physicalTime: bigint,
    deltaTime: bigint,
    trace: TraceContext,
    triggers: Array<GuestBoundEvent>,
    timers: Array<TimerFired>,
  }
  export type Signal = SignalPark | SignalYield;
  export interface SignalPark {
    tag: 'park',
  }
  export interface SignalYield {
    tag: 'yield',
  }
  export interface Version {
    major: number,
    minor: number,
    patch: number,
  }
  /**
   * # Variants
   * 
   * ## `"shared"`
   * 
   * ## `"dedicated"`
   */
  export type SchedulingPolicy = 'shared' | 'dedicated';
  export interface HostInfo {
    version: Version,
    memMax: bigint,
    timeLimit: bigint,
    busSize: bigint,
    cores: number,
    policy: SchedulingPolicy,
  }
  export interface LoadArgs {
    hostInfo: HostInfo,
    entrypoint: string,
    version: Version,
  }
  
  export class Plugin {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    static load(args: LoadArgs): Plugin;
    weave(args: WeaveArgs): Signal;
  }
}
