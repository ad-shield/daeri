use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ProtectPolicy {
    Timed(SystemTime),
    ProtectOnce,
}
