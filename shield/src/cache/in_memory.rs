use crate::cache::CacheStorage;
use crate::Error;

use std::time::Duration;

use endorphin::policy::TTLPolicy;
use endorphin::HashMap as CachedHashMap;

use serde::de::DeserializeOwned;
use serde::Serialize;

use tokio::sync::RwLock;

pub struct InMemoryCacheStorage {
    cache: RwLock<CachedHashMap<String, Vec<u8>, TTLPolicy>>,
}

impl InMemoryCacheStorage {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(CachedHashMap::new(TTLPolicy::with_presision(
                Duration::from_secs(1),
            ))),
        }
    }
}

impl Default for InMemoryCacheStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl<V> CacheStorage<V> for InMemoryCacheStorage
where
    V: Serialize + DeserializeOwned + Send + 'static,
    V: Send + Sync + 'static,
{
    async fn insert(&self, k: &str, v: V, ttl: Duration) -> Result<(), Error> {
        let val = bincode::serialize(&v)?;
        self.cache.write().await.insert(k.to_string(), val, ttl);

        Ok(())
    }

    async fn get(&self, k: &str) -> Result<Option<V>, Error> {
        let val = if let Some(v) = self.cache.read().await.get(k) {
            v.clone()
        } else {
            return Ok(None);
        };

        Ok(Some(bincode::deserialize(&val)?))
    }

    async fn remove(&self, k: &str) -> Result<(), Error> {
        self.cache.write().await.remove(k);
        Ok(())
    }

    // health-check utility for Cache availability.
    async fn health_check(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    fn gen_storage() -> impl CacheStorage<String> {
        InMemoryCacheStorage::new()
    }

    #[test]
    async fn persist_test1() {
        let cache = gen_storage();
        cache
            .insert("key", "val".to_string(), Duration::from_secs(1))
            .await
            .unwrap();
    }

    #[test]
    async fn persist_test2() {
        let cache = gen_storage();
        cache
            .insert("key", "val".to_string(), Duration::from_secs(1))
            .await
            .unwrap();
        cache.get("key").await.unwrap().unwrap();
    }
}
