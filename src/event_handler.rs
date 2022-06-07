use std::sync::Arc;

use tokio::sync::RwLock;

#[derive(Default)]
pub struct EventHandler<T> {
    lock_status: bool,
    data: Arc<RwLock<T>>,
}
