//! Implementation of [`TaskManager`].
//!
//! It helps managing the execution of concurrent tasks.
//! It can use a semaphore to control the amount of tasks being executed at the same time.
use std::{future::Future, sync::Arc};
use tokio::{sync::Semaphore, task::JoinSet};
use tracing::{error, Instrument};

pub struct TaskManager<T: Send + 'static> {
    join_set: JoinSet<T>,
    semaphore: Option<Arc<Semaphore>>,
}

impl<T: Send + 'static> TaskManager<T> {
    pub fn with_max_concurrent_tasks(value: usize) -> Self {
        TaskManager {
            join_set: JoinSet::new(),
            semaphore: Some(Arc::new(Semaphore::new(value))),
        }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = T> + Send + 'static,
    {
        let semaphore = self.semaphore.clone();
        let _handle = self
            .join_set
            .spawn(async move {
                let mut _permit = None;
                if let Some(semaphore) = semaphore {
                    _permit = Some(semaphore.acquire_owned().await.unwrap());
                }
                future.await
            })
            .instrument(tracing::debug_span!("Task spawn").or_current());
    }

    pub async fn await_tasks(&mut self) {
        while let Some(res) = self.join_set.join_next().await {
            if let Err(error) = res {
                error!(%error, "Unexpected task error");
            }
        }
    }
}
