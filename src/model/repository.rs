use super::{models::*, repository_message::*};
use anyhow::Result;
use std::time::Duration;
use tokio::{
    select,
    sync::mpsc::{channel, Receiver, Sender},
};

#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    fn register_view_channel(&mut self, channel: Sender<RepositoryMessage>);
    fn get_channel(&self) -> Sender<RepositoryMessage>;
    async fn run(&mut self) -> Result<()>;
}

pub fn bootstrap_repository() -> Result<Box<dyn Repository>> {
    let (tx, rx) = channel::<RepositoryMessage>(1);

    let repository = DataRepository {
        view_channel: None,
        receiver: rx,
        channel: tx,

        data_local_cache: Model::default(),
    };

    Ok(Box::new(repository))
}

struct DataRepository {
    view_channel: Option<Sender<RepositoryMessage>>,
    receiver: Receiver<RepositoryMessage>,
    channel: Sender<RepositoryMessage>,

    data_local_cache: Model,
}

impl Drop for DataRepository {
    fn drop(&mut self) {
        log::debug!("DataRepository is dropped");
    }
}

#[async_trait::async_trait]
impl Repository for DataRepository {
    async fn run(&mut self) -> Result<()> {
        let mut change_set: Vec<FromRepositoryMessageItem>;
        self.load().await?;
        change_set = vec![FromRepositoryMessageItem::Insert(
            self.data_local_cache.clone(),
        )];
        loop {
            if change_set.len() > 0 {
                let channel = self.view_channel.clone().unwrap();
                channel
                    .send(RepositoryMessage::FromRepository(change_set.clone()))
                    .await?;
                change_set.clear();
            }
            let message = self.select().await;
            if let Some(_msg) = message {
                // TODO: collect change set
            }
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
    async fn select(&mut self) -> Option<RepositoryMessage> {
        select! {
            message = self.receiver.recv() => { message }
            _ = tokio::time::sleep(Duration::from_millis(100)) => { None }
        }
    }

    async fn load(&mut self) -> Result<()> {
        self.data_local_cache = Model {
            character_list: vec![CharacterModel {
                id: "cid1".to_owned(),
                name: "name1".to_owned(),
            }],
            quest_list: vec![QuestModel {
                id: "qid1".to_owned(),
                title: "quest title".to_owned(),
                max_characters: 1
            }],
            ..Default::default()
        };
        Ok(())
    }
}
