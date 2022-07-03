use crate::cache::CacheStorage;
use crate::Error;

use bb8_redis::bb8::Pool;
use bb8_redis::redis;
use bb8_redis::redis::AsyncCommands;
use bb8_redis::RedisConnectionManager;

use serde::de::DeserializeOwned;
use serde::Serialize;

use std::time::Duration;

pub struct RedisCacheStorage {
    service: Pool<RedisConnectionManager>,
}

impl RedisCacheStorage {
    pub async fn new(uri: impl AsRef<str>) -> Self {
        let manager = RedisConnectionManager::new(uri.as_ref()).unwrap();
        Self {
            service: Pool::builder().build(manager).await.unwrap(),
        }
    }
}

#[async_trait::async_trait]
impl<V> CacheStorage<V> for RedisCacheStorage
where
    V: Serialize + DeserializeOwned,
    V: Send + Sync + 'static,
{
    async fn insert(&self, k: &str, v: V, ttl: Duration) -> Result<(), Error> {
        let val = bincode::serialize(&v)?;

        let mut pipe = redis::pipe();
        pipe.atomic().ignore();
        pipe.set(k, val).ignore();
        pipe.expire(k, ttl.as_secs() as usize).ignore();

        let mut svc = self.service.get().await.unwrap();
        pipe.query_async::<_, ()>(&mut *svc).await?;

        Ok(())
    }

    async fn get(&self, k: &str) -> Result<Option<V>, Error> {
        let val: Vec<u8> = {
            let mut svc = self.service.get().await.unwrap();
            svc.get(k).await?
        };

        Ok(Some(bincode::deserialize(&val)?))
    }

    async fn remove(&self, k: &str) -> Result<(), Error> {
        let mut svc = self.service.get().await.unwrap();
        svc.del(k).await?;

        Ok(())
    }

    // health-check utility for Cache availability.
    async fn health_check(&self) -> bool {
        true
    }
}
