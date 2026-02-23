use crate::types::{FilamentError, GuestBoundEvent};

/// A query for events in the timeline
#[derive(Debug, Clone)]
pub struct TimelineQuery {
    pub topic: Option<String>,
    pub from: Option<u64>,
    pub to: Option<u64>,
}

/// A handle to a blob in the blob store
#[derive(Debug, Clone)]
pub struct BlobHandle(pub String);

/// The Transactional Event Log.
#[async_trait::async_trait]
pub trait Timeline: Send + Sync {
    async fn append(&self, event: GuestBoundEvent) -> Result<(), FilamentError>;
    async fn query(&self, q: TimelineQuery) -> Result<Vec<GuestBoundEvent>, FilamentError>;
}

/// The Binary Object Store.
#[async_trait::async_trait]
pub trait BlobStore: Send + Sync {
    async fn put(&self, data: Vec<u8>) -> Result<BlobHandle, FilamentError>;
    async fn get(&self, id: &str) -> Result<Option<Vec<u8>>, FilamentError>;
    async fn exists(&self, id: &str) -> Result<bool, FilamentError>;
}
