/// <reference types="@filament/sdk" />

// Import types from SDK
import type { LoadPluginArgs } from "filament:core/module";
import type { WeaveArgs, Signal } from "filament:core/plugin";
import type { GuestBoundEvent } from "filament:core/types";

// Import host functions - jco will wire these to the WIT imports
import { log } from "filament:core/logger";
import { openWriter } from "filament:core/channel";
import { arm } from "filament:core/timer";

const PING = "ping";
const PONG = "pong";

/**
 * Base plugin class required by componentize-js
 */
class PluginBase {
  weave(_args: WeaveArgs): Signal {
    return { tag: "park" };
  }
}

/**
 * The ping plugin will emit a "ping" event when its timer fires.
 * It starts when it receives a sys/lifecycle/init event.
 */
class PingPlugin extends PluginBase {
  weave(args: WeaveArgs): Signal {
    const isInit = args.triggers.some(
      (e: GuestBoundEvent) => e.topic === "sys/lifecycle/init",
    );
    const timerFired = args.timers.length > 0;

    if (isInit || timerFired) {
      log("debug", "Sending ping");

      // Send ping event
      const sender = openWriter({ topic: PING });
      sender.send({
        topic: PING,
        data: undefined,
        traceContext: undefined,
        traceState: undefined,
      });

      // Arm timer for next ping (1 second = 1,000,000,000 nanos)
      arm({ nanos: 1_000_000_000n });
    }

    // Park - runtime will wake us when timer fires or events arrive
    return { tag: "park" };
  }
}

/**
 * The pong plugin listens for "ping" events and emits "pong" events.
 */
class PongPlugin extends PluginBase {
  weave(args: WeaveArgs): Signal {
    for (const event of args.triggers) {
      if (event.topic === PING) {
        log("debug", "Sending pong");

        const sender = openWriter({ topic: PONG });
        sender.send({
          topic: PONG,
          data: undefined,
          traceContext: undefined,
          traceState: undefined,
        });
      }
    }

    return { tag: "park" }; // Wait for more work
  }
}

/**
 * Module entrypoint - loads the appropriate plugin based on the entrypoint name.
 */
function loadPluginImpl(args: LoadPluginArgs): PluginBase {
  log("info", `Loading plugin: ${args.entrypoint}`);

  // Return the specific plugin implementation
  switch (args.entrypoint) {
    case PING:
      return new PingPlugin();
    case PONG:
      return new PongPlugin();
    default:
      throw { tag: "not-found" };
  }
}

// Export the module interface as expected by componentize-js
export const plugin = {
  Plugin: PluginBase,
};
export { loadPluginImpl as loadPlugin };
