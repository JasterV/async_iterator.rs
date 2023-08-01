use async_trait::async_trait;
use std::{error::Error, future::Future, pin::Pin, sync::Arc};
use tokio::{sync::Semaphore, task::JoinSet};
use tracing::{error, Instrument};

#[async_trait]
pub trait ConcurrentIterator: Sized + Send {
    type Item: Send;

    async fn for_each<OP, E>(self, max_concurrent_tasks: u8, op: OP)
    where
        E: Error + Send + 'static,
        OP: 'static
            + Sync
            + Send
            + Copy
            + Fn(Self::Item) -> Pin<Box<dyn Future<Output = Result<(), E>> + Send>>;
}

#[async_trait]
impl<T> ConcurrentIterator for Vec<T>
where
    T: Send + 'static,
{
    type Item = T;

    async fn for_each<OP, E>(self, max_concurrent_tasks: u8, op: OP)
    where
        E: Error + Send + 'static,
        OP: 'static
            + Sync
            + Send
            + Copy
            + Fn(Self::Item) -> Pin<Box<dyn Future<Output = Result<(), E>> + Send>>,
    {
        let mut tasks_set = JoinSet::new();

        // We experienced some concurrent requests rate limit errors with some providers such as
        // Gmail. For this reason we limit the number of concurrent tasks.
        let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks as usize));

        for elem in self {
            let semaphore = semaphore.clone();
            tasks_set.spawn(
                async move {
                    let _permit = semaphore
                        .acquire_owned()
                        .await
                        .expect("Tried to acquire permit from an already closed semaphore");

                    op(elem).await
                }
                .instrument(tracing::debug_span!("Semaphore acquire").or_current()),
            );
        }

        while let Some(res) = tasks_set.join_next().await {
            match res {
                Ok(_) => (),
                Err(error) => {
                    error!(%error, "Unexpected task error");
                }
            }
        }
    }
}
