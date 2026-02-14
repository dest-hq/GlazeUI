use std::pin::Pin;

pub struct Task<M: Clone> {
    pub future: Option<Pin<Box<dyn Future<Output = M> + Send>>>,
}

impl<M: Clone> Task<M> {
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = M> + Send + 'static,
    {
        Self {
            future: Some(Box::pin(future)),
        }
    }

    pub fn none() -> Self {
        Self { future: None }
    }
}
