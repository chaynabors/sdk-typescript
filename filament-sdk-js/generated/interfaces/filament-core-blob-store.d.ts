/// <reference path="./filament-core-types.d.ts" />
declare module 'filament:core/blob-store' {
  export function openWrite(): Writer;
  export function openRead(handle: BlobHandle): Reader;
  export function exists(handle: BlobHandle): boolean;
  export type Error = import('filament:core/types').Error;
  export type BlobHandle = bigint;
  
  export class Reader implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    read(len: bigint): Uint8Array;
    [Symbol.dispose](): void;
  }
  
  export class Writer implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    write(chunk: Uint8Array): void;
    commit(): BlobHandle;
    abort(): void;
    [Symbol.dispose](): void;
  }
}
