/// <reference path="./filament-core-types.d.ts" />
declare module 'filament:core/channel' {
  export function open(args: OpenArgs): [Sender, Receiver];
  export function openReader(args: OpenArgs): Receiver;
  export function openWriter(args: OpenArgs): Sender;
  export type HostBoundEvent = import('filament:core/types').HostBoundEvent;
  export type GuestBoundEvent = import('filament:core/types').GuestBoundEvent;
  export type Error = import('filament:core/types').Error;
  export interface OpenArgs {
    topic: string,
  }
  
  export class Receiver implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    recv(count: number): Array<GuestBoundEvent>;
    [Symbol.dispose](): void;
  }
  
  export class Sender implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    send(evt: HostBoundEvent): bigint;
    [Symbol.dispose](): void;
  }
}
