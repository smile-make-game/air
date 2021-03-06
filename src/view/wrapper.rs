use super::{helper::*, interfaces::evolute::Evolute, view::View, view_model::ViewModel};
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
    pub fn quick_update(&self, event: &Event) -> Result<Option<Evolution>> {
        self.view_model.process_input(event);
        self.draw()?;
        Ok(None)
    }

    pub fn apply_evolution(&self, evolution: Evolution) -> Result<()> {
        self.view_model.evolute(&evolution);
        Ok(self.draw()?)
    }

    fn draw(&self) -> Result<()> {
        self.terminal.borrow_mut().draw(|frame| {
            frame.render_widget(&self.view, frame.size());
        })?;
        Ok(())
    }
}
