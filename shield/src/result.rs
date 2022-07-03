use std::time::Duration;

use http::Uri;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CacheResult {
    #[serde(with = "http_serde::uri")]
    uri: Uri,
    ttl: Option<Duration>,
}

impl CacheResult {
    pub fn new(uri: Uri, ttl: Option<Duration>) -> Self {
        Self { uri, ttl }
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn ttl(&self) -> Option<Duration> {
        self.ttl
    }
}
