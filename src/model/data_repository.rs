use anyhow::Result;
use std::rc::Rc;
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[async_trait::async_trait]
pub trait DataRepository {
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn flush(&self) -> Result<()>;
}

pub fn init_data_repository() -> Rc<dyn DataRepository> {
    Rc::new(BackendProxy {})
}

struct BackendProxy {}

#[async_trait::async_trait]
impl DataRepository for BackendProxy {
    async fn start(&self) -> Result<()> {
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }

    async fn flush(&self) -> Result<()> {
        Ok(())
    }
}
