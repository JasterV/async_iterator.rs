//! Implementation of [`AsyncIterator`] on [`Vec`]

use crate::{task_manager::TaskManager, AsyncIterator, IntoAsyncIterator};
use async_trait::async_trait;
use std::future::Future;

pub struct VecIterator<T: Send> {
    vec: Vec<T>,
    max_concurrent_tasks: usize,
}

impl<T> IntoAsyncIterator for Vec<T>
where
    T: Send + 'static,
{
    type Item = T;
    type Iter = VecIterator<T>;

    fn into_async_iter(self, max_concurrent_tasks: usize) -> Self::Iter {
        VecIterator {
            vec: self,
            max_concurrent_tasks,
        }
    }
}

#[async_trait]
impl<T> AsyncIterator for VecIterator<T>
where
    T: Send + 'static,
{
    type Item = T;

    async fn for_each<O, P, F>(self, callback: F)
    where
        O: Send + 'static,
        P: Future<Output = O> + Send + 'static,
        F: Fn(Self::Item) -> P + Send + Sync + std::marker::Copy + 'static,
    {
        let mut task_manager = TaskManager::with_max_concurrent_tasks(self.max_concurrent_tasks);

        for elem in self.vec {
            task_manager.spawn(callback(elem));
        }

        task_manager.await_tasks().await;
    }
}
