use std::cell::RefCell;

use crate::{model::*, view::*};
use anyhow::Result;
use crossterm::event;
use tokio::{
    select, spawn,
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
};

pub struct Core {
    view: ViewWrapper,
    repository: Option<JoinHandle<()>>,

    channel: Sender<RepositoryMessage>,
    receiver: RefCell<Receiver<RepositoryMessage>>,
    repository_channel: Option<Sender<RepositoryMessage>>,
}

impl Default for Core {
    fn default() -> Self {
        let (tx, rx) = channel::<RepositoryMessage>(1);

        Core {
            view: ViewWrapper::default(),
            repository: None,

            channel: tx,
            receiver: RefCell::new(rx),
            repository_channel: None,
        }
    }
}

impl Core {
    pub async fn init(&mut self) -> Result<()> {
        let mut repository = bootstrap_repository()?;
        let repo_addr = repository.get_channel();
        repository.register_view_channel(self.channel.clone());
        self.repository_channel = Some(repo_addr);

        let handle = tokio::spawn(async move {
            let task = repository.run().await;
            match task {
                Ok(_) => {}
                Err(err) => {
                    log::error!("error on repository: {:?}", err);
                }
            }
        });
        self.repository = Some(handle);

        Ok(())
    }

    pub async fn dead_loop(&self) -> Result<()> {
        let result = self.priv_dead_loop().await;
        if let Err(e) = result {
            log::error!("error in dead loop: {:?}", e);
            return Err(anyhow::Error::msg(
                "unexpected error occurred, please check log for detail information",
            ));
        }
        Ok(())
    }

    async fn priv_dead_loop(&self) -> Result<()> {
        loop {
            let input = self.select().await?;
            log::trace!("get input: {:?}", input);
            match input {
                Input::Idle => {
                    self.view.tick()?;
                }
                Input::Event(event) => {
                    if should_exit(&event) {
                        // TODO: save data

                        break;
                    }
                    self.view.handle_input(&event)?;
                }
                // Input::Tick => {
                //     self.view.tick()?;
                // }
                Input::Message(message) => {
                    if let Some(msg) = message {
                        self.view.handle_message(msg)?;
                    }
                }
            };
        }
        Ok(())
    }

    async fn select(&self) -> Result<Input> {
        let mut receiver = self.receiver.borrow_mut();
        select! {
            input = spawn(async { idle_or_event() }) => input?,
            // _ = sleep(Duration::from_millis(50)) => Ok(Input::Tick),
            message = receiver.recv() => Ok(Input::Message(message)),
        }
    }
}

#[derive(Debug)]
pub enum Input {
    Idle,
    // Tick,
    Event(event::Event),
    Message(Option<RepositoryMessage>),
}

pub fn idle_or_event() -> Result<Input> {
    let result: Input;
    if event::poll(std::time::Duration::from_millis(200))? {
        let event = event::read()?;
        log::debug!("get event from poll: {:?}", event);
        result = Input::Event(event)
    } else {
        result = Input::Idle;
    }

    Ok(result)
}

pub fn should_exit(event: &event::Event) -> bool {
    if let event::Event::Key(key) = event {
        if key.code == event::KeyCode::Char('c') && key.modifiers == event::KeyModifiers::CONTROL {
            log::info!("should exit");
            return true;
        }
    }
    return false;
}
