use filament_sdk::{Module, Plugin};
use filament_sdk::exports::filament::core::module::LoadArgs;
use filament_sdk::filament::core::module::{Signal, WeaveArgs};
use filament_sdk::filament::core::types::{Error, HostBoundEvent};
use filament_sdk::filament::core::{channel, timer, logger};

const PING: &str = "ping";
const PONG: &str = "pong";

/// The ping plugin will emit a "ping" event when its timer fires.
/// It starts when it receives a sys/lifecycle/init event.
struct PingPlugin;

impl Plugin for PingPlugin {
    fn weave(&self, args: WeaveArgs) -> Result<Signal, Error> {
        // Check for lifecycle init
        let is_init = args.triggers.iter().any(|e| e.topic == "sys/lifecycle/init");

        // Check if timer fired (subsequent wakeups)
        let timer_fired = !args.timers.is_empty();

        if is_init || timer_fired {
            logger::log(logger::Level::Debug, "Sending ping");

            // Send ping event
            let sender = channel::open_writer(channel::OpenArgs {
                topic: PING.to_string(),
            })?;

            sender.send(HostBoundEvent {
                topic: PING.to_string(),
                data: None,
                trace_context: None,
                trace_state: None,
            })?;

            // Arm timer for next ping (1 second = 1,000,000,000 nanos)
            timer::arm(timer::ArmArgs {
                nanos: 1_000_000_000,
            })?;
        }

        // Park - runtime will wake us when timer fires or events arrive
        Ok(Signal::Park)
    }
}

/// The pong plugin listens for "ping" events and emits "pong" events.
struct PongPlugin;

impl Plugin for PongPlugin {
    fn weave(&self, args: WeaveArgs) -> Result<Signal, Error> {
        for event in &args.triggers {
            logger::log(logger::Level::Debug, "Sending pong");

            if event.topic == PING {
                let sender = channel::open_writer(channel::OpenArgs {
                    topic: PONG.to_string(),
                })?;

                sender.send(HostBoundEvent {
                    topic: PONG.to_string(),
                    data: None,
                    trace_context: None,
                    trace_state: None,
                })?;
            }
        }

        Ok(Signal::Park) // Wait for more work
    }
}

// Finally we define the module itself which exposes the plugins to the Filament runtime.
struct PingPongModule;

impl Module for PingPongModule {
    type Plugin = Box<dyn Plugin>;

    fn load(args: LoadArgs) -> Result<Self::Plugin, Error> {
        logger::log(logger::Level::Info, &format!("Loading plugin: {}", args.entrypoint));

        // Return the specific plugin based on entrypoint
        match args.entrypoint.as_str() {
            PING => Ok(Box::new(PingPlugin)),
            PONG => Ok(Box::new(PongPlugin)),
            _ => Err(Error::NotFound),
        }
    }
}

filament_sdk::export!(PingPongModule with_types_in filament_sdk);
