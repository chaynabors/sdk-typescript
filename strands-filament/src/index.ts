/// <reference types="@filament/sdk" />

// Import types from SDK
import type { WeaveArgs, Signal, LoadArgs } from "filament:core/plugin";
import type { GuestBoundEvent } from "filament:core/types";

// Import host functions
import { log } from "filament:core/logger";
import { openWriter } from "filament:core/channel";
import { arm } from "filament:core/timer";

const PING = "ping";
const PONG = "pong";

/**
 * Base Plugin resource class - required by WIT
 */
export class Plugin {
  _weaveHandler?: (args: WeaveArgs) => Signal;

  static load(args: LoadArgs): Plugin {
    log("info", `Loading plugin: ${args.entrypoint}`);

    switch (args.entrypoint) {
      case PING:
        return new PingPlugin();
      case PONG:
        return new PongPlugin();
      default:
        throw { tag: "not-found" };
    }
  }

  weave(args: WeaveArgs): Signal {
    if (this._weaveHandler) {
      return this._weaveHandler(args);
    }
    log("warn", "BASE Plugin.weave called with no handler!");
    return { tag: "park" };
  }
}

/**
 * Ping plugin implementation
 */
class PingPlugin extends Plugin {
  constructor() {
    super();
    this._weaveHandler = (args: WeaveArgs): Signal => {
      log("info", `PingPlugin.weave called with ${args.triggers.length} triggers, ${args.timers.length} timers`);

      const isInit = args.triggers.some(
        (e: GuestBoundEvent) => e.topic === "sys/lifecycle/init",
      );
      const timerFired = args.timers.length > 0;

      if (isInit || timerFired) {
        log("info", "Sending ping");

        const sender = openWriter({ topic: PING });
        sender.send({
          topic: PING,
          data: undefined,
          traceContext: undefined,
          traceState: undefined,
        });

        arm({ nanos: 1_000_000_000n });
      }

      return { tag: "park" };
    };
  }
}

/**
 * Pong plugin implementation
 */
class PongPlugin extends Plugin {
  constructor() {
    super();
    this._weaveHandler = (args: WeaveArgs): Signal => {
      log("info", `PongPlugin.weave called with ${args.triggers.length} triggers`);

      for (const event of args.triggers) {
        log("info", `PongPlugin received event: ${event.topic}`);
        if (event.topic === PING) {
          log("info", "Sending pong");

          const sender = openWriter({ topic: PONG });
          sender.send({
            topic: PONG,
            data: undefined,
            traceContext: undefined,
            traceState: undefined,
          });
        }
      }

      return { tag: "park" };
    };
  }
}

/**
 * Export the plugin interface - required for componentize-js resource registry
 */
export const plugin = {
  Plugin: Plugin
};
