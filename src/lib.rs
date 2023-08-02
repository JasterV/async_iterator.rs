//! This is a pet project that aims to provide  a set of Iterator implementations to iterate over a
//! sequence using async closures.
//!
//! It is built on top of Tokio and it only uses tokio primitives for the implementation.
//!
//! This is not meant to be a replace for libraries like [Rayon](https://github.com/rayon-rs/rayon), my knowledge on Rust can't compare to
//! the great minds behind libraries like that. This aims to be a very simple and small codebase to
//! suit my own needs.
//!
//! Feel free to contribute to improve it if you want to also satisfy your own use cases. I will gladly review
//! and discuss any issues/PRs to make this a proper crate :)
mod task_manager;
pub mod vec;

use async_trait::async_trait;
use std::future::Future;

/// `IntoAsyncIterator` implements the conversion to a [`AsyncIterator`].
///
/// By implementing `IntoAsyncIterator` for a type, you define how it will
/// transformed into an iterator.
pub trait IntoAsyncIterator {
    type Iter: AsyncIterator<Item = Self::Item>;

    type Item: Send;

    fn into_async_iter(self, max_concurrent_tasks: usize) -> Self::Iter;
}

/// Async version of the standard iterator trait.
/// At the moment it only provides the set of methods that suit my needs.
#[async_trait]
pub trait AsyncIterator: Sized + Send {
    type Item: Send + Sync;

    async fn for_each<O, P, F>(self, callback: F)
    where
        O: Send + 'static,
        P: Future<Output = O> + Send + 'static,
        F: Fn(Self::Item) -> P + 'static + Send + Sync;
}
