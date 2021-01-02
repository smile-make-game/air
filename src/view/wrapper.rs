use super::{helper::*, view::View, view_model::ViewModel};
use crate::model::*;
use anyhow::Result;
use crossterm::event::Event;
use std::{cell::RefCell, io::Stdout, rc::Rc};
use tui::{backend::CrosstermBackend, Terminal};

pub struct ViewWrapper {
    terminal: RefCell<Terminal<CrosstermBackend<Stdout>>>,

    view_model: Rc<ViewModel>,
    view: View,
}

impl Default for ViewWrapper {
    fn default() -> Self {
        let terminal = configure_terminal();
        let view_model = Rc::new(ViewModel::default());
        let view = View::from(view_model.clone());

        Self {
            terminal: RefCell::new(terminal),
            view_model,
            view,
        }
    }
}

impl Drop for ViewWrapper {
    fn drop(&mut self) {
        reset_terminal(self.terminal.get_mut())
    }
}

impl ViewWrapper {
    pub fn handle_input(&self, event: &Event) -> Result<()> {
        self.view_model.process_input(event);
        Ok(self.tick()?)
    }

    pub fn tick(&self) -> Result<()> {
        Ok(self.draw()?)
    }

    pub fn handle_message(&self, message: RepositoryMessage) -> Result<()> {
        self.view_model.process_data(message);

        Ok(self.tick()?)
    }

    fn draw(&self) -> Result<()> {
        self.terminal.borrow_mut().draw(|frame| {
            frame.render_widget(&self.view, frame.size());
        })?;
        Ok(())
    }
}
