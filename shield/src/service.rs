use crate::{CacheResult, Error};

use super::{CacheStorage, Storage};
use crate::util::{generate_protected_path, guessed_as_protected, into_epoch, into_systime};
use axum::{body::Body, http::Request, response::Response};
use endorphin::policy::TTLPolicy;
use endorphin::HashMap as CachedHashMap;
use futures::future::BoxFuture;
use http::Uri;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::{
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;
use tower::Service;

#[derive(Clone)]
pub struct Shield<S> {
    inner: S,
    cache_storage: Arc<dyn CacheStorage<Storage> + Send + Sync>,
    reusable_records: Arc<RwLock<CachedHashMap<Uri, Uri, TTLPolicy>>>,
    local_opt_out_storage: Arc<RwLock<CachedHashMap<String, (), TTLPolicy>>>,
}

impl<S> Service<Request<Body>> for Shield<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let result = self.resolve(request.uri().clone());
        todo!()
        // let uri = format!("http://127.0.0.1:3000{}", path_query);

        // *request.uri_mut() = Uri::try_from(uri).unwrap();

        // let future = self.inner.call(request);
        // Box::pin(async move {
        //     let response: Response = future.await?;
        //     Ok(response)
        // })
    }
}

impl<S> Shield<S> {
    pub fn new(inner: S, cache_storage: Arc<dyn CacheStorage<Storage> + Send + Sync>) -> Self {
        Self {
            inner,
            cache_storage: cache_storage,
            reusable_records: Arc::new(RwLock::new(CachedHashMap::new(TTLPolicy::with_presision(
                Duration::from_secs(30),
            )))),
            local_opt_out_storage: Arc::new(RwLock::new(CachedHashMap::new(TTLPolicy::new()))),
        }
    }

    pub async fn opt_out_uuid(&self, _uuid: &str) -> Result<(), crate::Error> {
        let _7_days = Duration::from_secs(60 * 60 * 24 * 7);
        todo!()
        // SyncFuture::new(self.cache_storage.insert(
        //     uuid,
        //     Storage::OptOut {
        //         expire_at: SystemTime::now() + _7_days,
        //     },
        //     _7_days,
        // ))
        // .await?;

        // Ok(())
    }

    pub async fn is_opted_out(&self, uuid: &str) -> bool {
        if self.local_opt_out_storage.read().await.contains_key(uuid) {
            return true;
        }

        todo!()
        // let expire_at = match SyncFuture::new(self.cache_storage.get(uuid)).await {
        //     Ok(Some(Storage::OptOut { expire_at })) => expire_at,
        //     _ => return false,
        // };

        // let now = SystemTime::now();
        // if expire_at <= now {
        //     return false;
        // }

        // SyncFuture::new(self.local_opt_out_storage.write())
        //     .await
        //     .insert(uuid.to_string(), (), expire_at.duration_since(now).unwrap());
        // true
    }

    pub async fn resolve(&self, uri: Uri) -> Option<CacheResult> {
        let uri_path = uri.path();

        // check this uri is guessed as protected.
        // this function checks only possibilities.
        if !guessed_as_protected(uri_path) {
            return None;
        }

        let (uri, expire_at) = match self.cache_storage.get(uri_path).await {
            Ok(Some(Storage::ProtectedUrl { uri, expire_at })) => (uri, expire_at),
            _ => return None,
        };

        // extract ttl from epoch timestamp.
        // we'll transform it into Duration.
        let ttl = if let Some(expire_at) = expire_at {
            let current = SystemTime::now();
            let systime = into_systime(expire_at);

            if let Ok(ttl) = systime.duration_since(current) {
                Some(ttl)
            } else {
                // cache has already expired.
                return None;
            }
        } else {
            None
        };

        // try converting string into uri.
        Some(CacheResult::new(uri, ttl))
    }

    pub async fn protect(&self, uri: &Uri, ttl: Duration) -> Result<Uri, Error> {
        let expire_at = into_epoch(SystemTime::now() + ttl);

        let uri_postfix = generate_protected_path();
        let storage = Storage::ProtectedUrl {
            uri: uri.clone(),
            expire_at: Some(expire_at),
        };

        todo!()
        // SyncFuture::new(self.cache_storage.insert(&uri_postfix, storage, ttl * 2)).await?;
        // Ok(modify_path(uri.clone(), uri_postfix))
    }

    pub async fn protect_reusable(
        &self,
        uri: &Uri,
        ttl: Duration,
        reuse_for: Duration,
    ) -> Result<Uri, Error> {
        let reusable_record = self.reusable_records.read().await;
        if let Some(uri) = reusable_record.get(uri) {
            return Ok(uri.clone());
        }
        drop(reusable_record);

        let result = self.protect(uri, ttl).await?;
        let mut reusable_record = self.reusable_records.write().await;
        if !reusable_record.contains_key(uri) {
            reusable_record.insert(uri.clone(), result.clone(), reuse_for);
        }

        Ok(result)
    }

    pub async fn protect_once(&self, uri: &Uri) -> Result<Uri, Error> {
        let uri_postfix = generate_protected_path();

        let storage = Storage::ProtectedUrl {
            uri: uri.clone(),
            expire_at: None,
        };

        todo!()
        // SyncFuture::new(self.cache_storage.insert(
        //     &uri_postfix,
        //     storage,
        //     Duration::from_secs(60 * 15),
        // ))
        // .await?;

        // Ok(modify_path(uri.clone(), uri_postfix))
    }
}

// impl<S> Clone for Shield<S> {
//     fn clone(&self) -> Self {
//         Self {
//             inner,
//             cache_storage: self.cache_storage.clone(),
//             reusable_records: self.reusable_records.clone(),
//             local_opt_out_storage: self.local_opt_out_storage.clone(),
//         }
//     }
// }
