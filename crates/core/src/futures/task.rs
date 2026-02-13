use std::pin::Pin;

pub struct Task<M: Clone> {
    pub future: Pin<Box<dyn Future<Output = M> + Send>>,
}

impl<M: Clone> Task<M> {
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = M> + Send + 'static,
    {
        Self {
            future: Box::pin(future),
        }
    }
}
