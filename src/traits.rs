use async_trait::async_trait;
use std::future::Future;

#[async_trait]
pub trait FutureOption<T> where T: Send + Sync + 'static {
    type Output<O: Send + Sync + 'static>: FutureOption<O>;

    async fn async_and_then<F, Fut, O>(self, op: F) -> Self::Output<O>
    where
        F: FnOnce(T) -> Fut + Send + Sync,
        Fut: Future<Output = Self::Output<O>> + Send + Sync + 'static,
        O: Send + Sync + 'static;

    async fn async_map<F, Fut, O>(self, op: F) -> Self::Output<O>
    where
        F: FnOnce(T) -> Fut + Send + Sync,
        Fut: Future<Output = O> + Send + Sync + 'static,
        O: Send + Sync + 'static;

    async fn async_map_or<F, Fut, O>(self, default: O, op: F) -> O
    where
        F: FnOnce(T) -> Fut + Send + Sync,
        Fut: Future<Output = O> + Send + Sync + 'static,
        O: Send + Sync + 'static;

    async fn async_map_or_else<F, D, Fut, Fut2, O>(self, default: D, op: F) -> O
    where
        F: FnOnce(T) -> Fut + Send + Sync,
        Fut: Future<Output = O> + Send + Sync + 'static,
        D: FnOnce() -> Fut2 + Send + Sync,
        Fut2: Future<Output = O> + Send + Sync + 'static,
        O: Send + Sync + 'static;

    async fn async_get_or_insert_with<F, Fut>(&mut self, op: F) -> &mut T
    where
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = T> + Send + Sync + 'static;

    async fn async_zip_with<F, Fut, U, R>(self, other: Self::Output<U>, op: F) -> Self::Output<R>
    where
        F: FnOnce(T, U) -> Fut + Send + Sync,
        Fut: Future<Output = R> + Send + Sync + 'static,
        U: Send + Sync + 'static,
        R: Send + Sync + 'static;

    async fn async_unwrap_or_else<F, Fut>(self, op: F) -> T
    where
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = T> + Send + Sync + 'static;

    async fn async_is_some_and<F, Fut>(&self, op: F) -> bool
    where
        F: FnOnce(&T) -> Fut + Send + Sync,
        Fut: Future<Output = bool> + Send + Sync + 'static;

    async fn async_inspect<F, Fut>(self, op: F) -> Self::Output<T>
    where
        F: FnOnce(&T) -> Fut + Send + Sync,
        Fut: Future<Output = ()> + Send + Sync + 'static;

    async fn async_ok_or_else<F, Fut, E>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = E> + Send + Sync + 'static,
        E: Send + Sync + 'static;

    async fn async_filter<F, Fut>(self, pred: F) -> Self::Output<T>
    where
        F: FnOnce(&T) -> Fut + Send + Sync,
        Fut: Future<Output = bool> + Send + Sync + 'static;

    async fn async_or_else<F, Fut>(self, op: F) -> Self::Output<T>
    where
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = Self::Output<T>> + Send + Sync + 'static;
}

#[async_trait]
pub trait FutureResult<T, E> where T: Send + Sync + 'static, E: Send + Sync + 'static {
    type Output<A, B>: FutureResult<A, B> where A: Send + Sync + 'static, B: Send + Sync + 'static;

    async fn is_ok_and<F, Fut>(&self, op: F) -> bool
    where
        F: FnOnce(&T) -> Fut + Send + Sync,
        Fut: Future<Output = bool> + Send + Sync + 'static;

    async fn is_err_and<F, Fut>(&self, op: F) -> bool
    where
        F: FnOnce(&E) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = bool> + Send + Sync + 'static;

    async fn async_map<F, Fut, O>(self, op: F) -> Self::Output<O, E>
    where
        F: FnOnce(T) -> Fut + Send + Sync,
        Fut: Future<Output = O> + Send + Sync + 'static,
        O: Send + Sync + 'static;

    async fn async_map_or<F, Fut, O>(self, default: O, op: F) -> O
    where
        F: FnOnce(T) -> Fut + Send + Sync,
        Fut: Future<Output = O> + Send + Sync + 'static,
        O: Send + Sync + 'static;

    async fn async_map_err<F, Fut, O>(self, op: F) -> Self::Output<T, O>
    where
        F: FnOnce(E) -> Fut + Send + Sync,
        Fut: Future<Output = O> + Send + Sync + 'static,
        O: Send + Sync + 'static;

    async fn async_inspect<F, Fut, O>(self, op: F) -> Self::Output<T, E>
    where
        F: FnOnce(&T) -> Fut + Send + Sync,
        Fut: Future<Output = ()> + Send + Sync + 'static,
        O: Send + Sync + 'static;

    async fn async_inspect_err<F, Fut, O>(self, op: F) -> Self::Output<T, E>
    where
        F: FnOnce(&E) -> Fut + Send + Sync,
        Fut: Future<Output = ()> + Send + Sync + 'static,
        O: Send + Sync + 'static;

    async fn async_and_then<F, Fut, O>(self, op: F) -> Self::Output<O, E>
    where
        F: FnOnce(T) -> Fut + Send + Sync,
        Fut: Future<Output = Self::Output<O, E>> + Send + Sync + 'static,
        O: Send + Sync + 'static;

    async fn async_or_else<F, Fut, O>(self, op: F) -> Self::Output<T, O>
    where
        F: FnOnce(E) -> Fut + Send + Sync,
        Fut: Future<Output = Self::Output<T, O>> + Send + Sync + 'static,
        O: Send + Sync + 'static;


    async fn async_unwrap_or_else<F, Fut>(self, op: F) -> T
    where
        F: FnOnce(E) -> Fut + Send + Sync,
        Fut: Future<Output = T> + Send + Sync + 'static;

    async fn async_map_or_else<F, D, Fut, Fut2, O>(self, default: D, op: F) -> O
    where
        F: FnOnce(T) -> Fut + Send + Sync,
        Fut: Future<Output = O> + Send + Sync + 'static,
        D: FnOnce(E) -> Fut2 + Send + Sync,
        Fut2: Future<Output = O> + Send + Sync + 'static,
        O: Send + Sync + 'static;
}
