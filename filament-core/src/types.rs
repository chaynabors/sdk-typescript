#[derive(thiserror::Error, Debug)]
pub enum FilamentError {
    #[error("Not found")]
    NotFound,
    #[error("Permission Denied")]
    PermissionDenied,
    #[error(transparent)]
    Custom(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct TraceContext {
    pub trace_id_hi: u64,
    pub trace_id_lo: u64,
    pub span_id: u64,
    pub parent_id: u64,
    pub trace_flags: u8,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct TraceState(pub Vec<(String, String)>);

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct HostBoundEvent {
    pub topic: String,
    pub data: Option<Vec<u8>>,
    pub trace_context: Option<TraceContext>,
    pub trace_state: Option<TraceState>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct GuestBoundEvent {
    pub topic: String,
    pub id: u64,
    pub timestamp: u64,
    pub source: String,
    pub data: Option<Vec<u8>>,
    pub trace_context: TraceContext,
    pub trace_state: Option<TraceState>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct TimerId(pub u64);
