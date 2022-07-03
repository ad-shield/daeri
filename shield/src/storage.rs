use http::Uri;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub enum Storage {
    ProtectedUrl {
        #[serde(with = "http_serde::uri")]
        uri: Uri,
        expire_at: Option<u64>,
    },
    OptOut {
        expire_at: SystemTime,
    },
}
