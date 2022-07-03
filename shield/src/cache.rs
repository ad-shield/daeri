mod in_memory;
pub use in_memory::*;
mod redis;
pub use redis::*;

use crate::Error;

use serde::de::DeserializeOwned;
use serde::Serialize;

use std::time::Duration;

#[async_trait::async_trait]
pub trait CacheStorage<V>: Send + Sync + 'static
where
    V: Serialize + DeserializeOwned,
    V: Send + Sync + 'static,
{
    async fn insert(&self, k: &str, v: V, ttl: Duration) -> Result<(), Error>;
    async fn get(&self, k: &str) -> Result<Option<V>, Error>;
    async fn remove(&self, k: &str) -> Result<(), Error>;

    // health-check utility for Cache availability.
    async fn health_check(&self) -> bool;
}
