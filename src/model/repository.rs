use super::RepositoryMessage;
use anyhow::Result;
use tokio::{
    select,
    sync::mpsc::{channel, Receiver, Sender},
};

#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    fn register_view_channel(&mut self, channel: Sender<RepositoryMessage>);
    fn get_channel(&self) -> Sender<RepositoryMessage>;
    async fn run(&mut self);
}

pub fn bootstrap_repository() -> Result<Box<dyn Repository>> {
    let (tx, rx) = channel::<RepositoryMessage>(1);

    let repository = DataRepository {
        view_channel: None,
        receiver: rx,
        channel: tx,
    };

    Ok(Box::new(repository))
}

struct DataRepository {
    view_channel: Option<Sender<RepositoryMessage>>,
    receiver: Receiver<RepositoryMessage>,
    channel: Sender<RepositoryMessage>,
}

impl Drop for DataRepository {
    fn drop(&mut self) {}
}

#[async_trait::async_trait]
impl Repository for DataRepository {
    async fn run(&mut self) {
        loop {
            self.select().await;
        }
    }

    fn register_view_channel(&mut self, channel: Sender<RepositoryMessage>) {
        self.view_channel = Some(channel);
    }

    fn get_channel(&self) -> Sender<RepositoryMessage> {
        self.channel.clone()
    }
}

impl DataRepository {
    async fn select(&mut self) {
        select! {
            message = self.receiver.recv() => { message }
        };
    }
}
