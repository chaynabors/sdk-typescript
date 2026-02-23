declare module 'filament:core/types' {
  export type Error = ErrorNotFound | ErrorPermissionDenied;
  export interface ErrorNotFound {
    tag: 'not-found',
  }
  export interface ErrorPermissionDenied {
    tag: 'permission-denied',
  }
  export interface TraceContext {
    traceIdHi: bigint,
    traceIdLo: bigint,
    spanId: bigint,
    parentId?: bigint,
    traceFlags: number,
  }
  export type TraceState = Array<[string, string]>;
  export interface HostBoundEvent {
    topic: string,
    data?: Uint8Array,
    traceContext?: TraceContext,
    traceState?: TraceState,
  }
  export interface GuestBoundEvent {
    topic: string,
    id: bigint,
    timestamp: bigint,
    source: string,
    data?: Uint8Array,
    traceContext: TraceContext,
    traceState?: TraceState,
  }
  export type TimerId = bigint;
}
