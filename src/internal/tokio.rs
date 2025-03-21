use std::future::Future;

#[cfg(all(tokio_unstable, feature = "tokio_task_builder", not(feature = "tokio_compat")))]
pub fn spawn_named<F, T>(name: &str, future: F) -> tokio::task::JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::Builder::new().name(&*format!("serenity::{}", name)).spawn(future)
}

#[cfg(any(not(all(tokio_unstable, feature = "tokio_task_builder")), feature = "tokio_compat"))]
pub fn spawn_named<F, T>(_name: &str, future: F) -> tokio::task::JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(future)
}
