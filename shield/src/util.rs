use std::time::{Duration, SystemTime, UNIX_EPOCH};

use http::Uri;

pub fn crc8_hash<T>(v: T) -> u8
where
    T: AsRef<[u8]>,
{
    use crc::*;
    let crc: Crc<u8> = Crc::<u8>::new(&CRC_8_MAXIM_DOW);

    crc.checksum(v.as_ref())
}

pub(crate) fn guessed_as_protected<Q>(buf: Q) -> bool
where
    Q: AsRef<str>,
{
    let buf = buf.as_ref();

    // remove ext part.
    let buf = if let Some(idx) = buf.rfind('.') {
        &buf[..idx]
    } else {
        return false;
    };

    if buf.len() < 3 {
        return false;
    }

    // extract hash and data section from buffer.
    let hash = &buf[buf.len() - 2..];
    let data = &buf[..buf.len() - 2];

    // check that hash is same.
    if let Ok(hash) = u8::from_str_radix(hash, 16) {
        hash == crc8_hash(data.as_bytes())
    } else {
        false
    }
}

pub(crate) fn into_epoch<Q>(t: Q) -> u64
where
    Q: Into<SystemTime>,
{
    let time: SystemTime = t.into();
    time.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}

pub(crate) fn into_systime(from_epoch: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_millis(from_epoch)
}

pub(crate) fn generate_protected_path() -> String {
    use adshield_utils::RandExt;
    use rand::thread_rng as rng;

    // create random path.
    let path = format!("/{}", rng().gen_path());

    // make and append signature.
    return format!(
        "{}{:0>2x}.{}",
        path,
        crc8_hash(path.as_bytes()),
        rng().gen_ext()
    );
}

fn modify_path<Q>(_uri: Uri, path: Q) -> Uri
where
    Q: AsRef<str>,
{
    let builder = Uri::builder();
    builder.path_and_query(path.as_ref()).build().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signagture_check_test1() {
        let path = generate_protected_path();
        assert!(guessed_as_protected(&path));
    }

    #[test]
    fn signagture_check_test2() {
        for _ in 0..256 {
            let path = generate_protected_path();
            assert!(guessed_as_protected(&path));
        }
    }
}
