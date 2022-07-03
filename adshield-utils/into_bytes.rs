use http_body::Body;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Bytes, BytesMut};

#[pin_project::pin_project]
pub struct IntoBytes<T> {
    #[pin]
    inner: T,
    buffer: Option<BytesMut>,
}

impl<T> Future for IntoBytes<T>
where
    T: Body + Unpin,
{
    type Output = Result<Bytes, T::Error>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        use bytes::Buf;

        let project = self.project();
        let buf = project.buffer.as_mut().unwrap();

        match project.inner.poll_data(ctx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Ok(mut data))) => {
                while data.has_remaining() {
                    let chunk = data.chunk();
                    let chunk_len = chunk.len();
                    buf.extend_from_slice(chunk);
                    data.advance(chunk_len);
                }

                ctx.waker().wake_by_ref();
                Poll::Pending
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Err(e)),
            Poll::Ready(None) => Poll::Ready(Ok(project.buffer.take().unwrap().freeze())),
        }
    }
}

impl<B> IntoBytesBodyExt for B where B: Body + Unpin {}
pub trait IntoBytesBodyExt: Body + Sized {
    fn into_bytes(self) -> IntoBytes<Self> {
        let buffer = if let Some(size) = self.size_hint().upper() {
            BytesMut::with_capacity(size as usize)
        } else {
            BytesMut::new()
        };

        IntoBytes {
            inner: self,
            buffer: Some(buffer),
        }
    }
}
