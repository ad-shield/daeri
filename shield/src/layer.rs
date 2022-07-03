use std::sync::Arc;

use tower::Layer;

use super::{CacheStorage, Shield, Storage};

pub struct ShieldLayer {
    cache_storage: Arc<dyn CacheStorage<Storage> + Send + Sync>,
}

impl<S> Layer<S> for ShieldLayer {
    type Service = Shield<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Shield::new(inner, Arc::clone(&self.cache_storage))
    }
}

impl ShieldLayer {
    pub fn new<S>(cache_storage: S) -> ShieldLayer
    where
        S: CacheStorage<Storage> + Send + Sync,
    {
        Self {
            cache_storage: Arc::new(cache_storage),
        }
    }
}
