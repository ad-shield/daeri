use sync_wrapper::SyncWrapper;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[pin_project::pin_project]
pub struct SyncFuture<F> {
    #[pin]
    inner: SyncWrapper<F>,
}

impl<F> SyncFuture<F> {
    pub fn new(inner: F) -> Self {
        Self {
            inner: SyncWrapper::new(inner),
        }
    }
}

impl<F: Future> Future for SyncFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().inner.get_pin_mut().poll(cx)
    }
}
