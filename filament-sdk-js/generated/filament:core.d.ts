/// <reference path="./interfaces/filament-core-blob-store.d.ts" />
/// <reference path="./interfaces/filament-core-channel.d.ts" />
/// <reference path="./interfaces/filament-core-config.d.ts" />
/// <reference path="./interfaces/filament-core-logger.d.ts" />
/// <reference path="./interfaces/filament-core-plugin.d.ts" />
/// <reference path="./interfaces/filament-core-timer.d.ts" />
/// <reference path="./interfaces/filament-core-types.d.ts" />
declare module 'filament:core/module' {
  export type * as FilamentCoreBlobStore from 'filament:core/blob-store'; // import filament:core/blob-store
  export type * as FilamentCoreChannel from 'filament:core/channel'; // import filament:core/channel
  export type * as FilamentCoreConfig from 'filament:core/config'; // import filament:core/config
  export type * as FilamentCoreLogger from 'filament:core/logger'; // import filament:core/logger
  export type * as FilamentCoreTimer from 'filament:core/timer'; // import filament:core/timer
  export type * as FilamentCoreTypes from 'filament:core/types'; // import filament:core/types
  export * as plugin from 'filament:core/plugin'; // export filament:core/plugin
}
