use crate::types::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct TimerFired {
    pub timer_id: TimerId,
    pub scheduled_for: u64,
    pub skew: i64,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct WeaveArgs {
    pub tick: u64,
    pub virtual_time: u64,
    pub physical_time: u64,
    pub delta_time: u64,
    pub trace: TraceContext,
    pub triggers: Vec<GuestBoundEvent>,
    pub timers: Vec<TimerFired>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub enum Signal {
    Park,
    Yield,
}

#[async_trait::async_trait]
pub trait Plugin: Send {
    async fn weave(
        &mut self,
        args: WeaveArgs,
        events: &mut Vec<GuestBoundEvent>,
    ) -> Result<Signal, FilamentError>;
}
