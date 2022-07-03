use serde::de::DeserializeOwned;
use serde::Serialize;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncryptionError {
    #[error(transparent)]
    Base64Error(#[from] base64::DecodeError),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error("bad encryption format!")]
    BadFormatError,
}

pub fn encode_xor<Q>(data: Q) -> Result<String, EncryptionError>
where
    Q: AsRef<[u8]>,
{
    let data = data.as_ref();

    let key = rand::random::<u8>() % 255;
    let mut result = Vec::with_capacity(data.len() + 2);
    result.push(key);
    data.iter().for_each(|v| result.push(*v ^ key));

    Ok(base64::encode(result))
}

pub fn encode_xor_json<Q>(data: Q) -> Result<String, EncryptionError>
where
    Q: Serialize,
{
    encode_xor(serde_json::to_string(&data)?)
}

pub fn decode_xor<Q>(data: Q) -> Result<Vec<u8>, EncryptionError>
where
    Q: AsRef<[u8]>,
{
    let data = data.as_ref();
    let data = base64::decode(data)?;

    if data.len() < 3 {
        return Err(EncryptionError::BadFormatError);
    }

    let key = data[0];
    Ok(data[1..].iter().map(|v| v ^ key).collect())
}

pub fn decode_xor_json<Q, R>(data: Q) -> Result<R, EncryptionError>
where
    Q: AsRef<[u8]>,
    R: DeserializeOwned,
{
    Ok(serde_json::from_slice(&decode_xor(data)?)?)
}
