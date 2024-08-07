use std::{sync::Arc, time::Duration};

use rust_extensions::MyTimerTick;

use crate::app::AppContext;

pub struct PersistProcessGc {
    app: Arc<AppContext>,
    gc_timeout: Duration,
}

impl PersistProcessGc {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self {
            app,
            gc_timeout: Duration::from_secs(60),
        }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for PersistProcessGc {
    async fn tick(&self) {
        todo!("Implement");
        // self.app.grpc_persist_processes.gc(self.gc_timeout).await;
    }
}
