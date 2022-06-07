use std::sync::Arc;

use tokio::sync::RwLock;

#[derive(Default)]
pub struct EventHandler<T> {
    data: Arc<RwLock<T>>,
}

impl<T> EventHandler<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(RwLock::new(data)),
        }
    }
}
