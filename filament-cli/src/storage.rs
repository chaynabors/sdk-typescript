use filament::bindings::{BlobHandle, BlobStore, Timeline, TimelineQuery};
use filament::types::{FilamentError, GuestBoundEvent};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub struct VolatileStorage {
    events: Vec<GuestBoundEvent>,
}

impl VolatileStorage {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn query_internal(&self, query: TimelineQuery) -> Vec<GuestBoundEvent> {
        self.events
            .iter()
            .skip(query.from.unwrap_or(0).try_into().unwrap())
            .filter(|e| {
                if let Some(filter) = &query.topic {
                    if filter.ends_with("*") {
                        let prefix = &filter[..filter.len() - 1];
                        if !e.topic.starts_with(prefix) {
                            return false;
                        }
                    } else if &e.topic != filter {
                        return false;
                    }
                }

                true
            })
            .take_while(|e| {
                if let Some(until) = query.to {
                    e.id <= until
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }
}

pub struct VolatileBlobStore {
    blobs: RwLock<HashMap<String, Arc<Vec<u8>>>>,
}

impl VolatileBlobStore {
    pub fn new() -> Self {
        Self {
            blobs: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl BlobStore for VolatileBlobStore {
    async fn put(&self, data: Vec<u8>) -> Result<BlobHandle, FilamentError> {
        const CHUNK_SIZE: usize = 64 * 1024;
        let mut hasher = Sha256::new();
        let mut offset = 0;

        while offset < data.len() {
            let end = (offset + CHUNK_SIZE).min(data.len());
            hasher.update(&data[offset..end]);
            offset = end;
            if offset < data.len() {
                tokio::task::yield_now().await;
            }
        }

        let hash = hex::encode(hasher.finalize());
        {
            let mut map = self.blobs.write().await;
            if !map.contains_key(&hash) {
                map.insert(hash.clone(), Arc::new(data));
            }
        }

        Ok(BlobHandle(hash))
    }

    async fn get(&self, id: &str) -> Result<Option<Vec<u8>>, FilamentError> {
        let map = self.blobs.read().await;
        Ok(map.get(id).map(|data| data.as_ref().clone()))
    }

    async fn exists(&self, id: &str) -> Result<bool, FilamentError> {
        let map = self.blobs.read().await;
        Ok(map.contains_key(id))
    }
}

pub struct VolatileTimeline {
    store: Arc<Mutex<VolatileStorage>>,
}

impl VolatileTimeline {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(VolatileStorage::new())),
        }
    }
}

#[async_trait::async_trait]
impl Timeline for VolatileTimeline {
    async fn append(&self, event: GuestBoundEvent) -> Result<(), FilamentError> {
        let mut store = self.store.lock().await;
        store.events.push(event);
        Ok(())
    }

    async fn query(&self, q: TimelineQuery) -> Result<Vec<GuestBoundEvent>, FilamentError> {
        // Only return committed events
        Ok(self.store.lock().await.query_internal(q))
    }
}
