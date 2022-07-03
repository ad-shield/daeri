use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    BinCodeError(#[from] bincode::Error),

    #[error(transparent)]
    RedisError(#[from] bb8_redis::redis::RedisError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
