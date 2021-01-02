use crate::model::{FromRepositoryMessageItem, RepositoryMessage};

use super::{
    component::view_model::*, composite::view_model::*, interfaces::event_handler::KeyEventHandler,
};
use crossterm::event::*;
use std::{cell::RefCell, rc::Rc};

pub struct ViewModel {
    // misc
    pub edge_border: Rc<EdgeComponentViewModel>,
    pub tab: Rc<TabComponentViewModel>,
    // pages
    pub event_page: Rc<EventPageViewMode>,
    pub character_page: Rc<CharacterPageViewMode>,
    pub system_page: Rc<SystemPageViewMode>,

    // internal fields
    _is_loading: RefCell<bool>,
}

impl Default for ViewModel {
    fn default() -> Self {
        let edge_border = Rc::new(EdgeComponentViewModel::default());

        let event_page = Rc::new(EventPageViewMode::default());
        let character_page = Rc::new(CharacterPageViewMode::default());
        let system_page = Rc::new(SystemPageViewMode::default());

        let tab = Rc::new(TabComponentViewModel::default());
        tab.register_tab(event_page.clone());
        tab.register_tab(character_page.clone());
        tab.register_tab(system_page.clone());

        Self {
            tab,
            edge_border,
            event_page,
            character_page,
            system_page,

            _is_loading: RefCell::new(false),
        }
    }
}

impl KeyEventHandler for ViewModel {
    fn handle_key(&self, key: &KeyEvent) {
        self.tab.handle_key(key);
        self.event_page.handle_key(key);
        self.character_page.handle_key(key);
    }
}

impl ViewModel {
    pub fn process_input(&self, event: &Event) {
        match event {
            Event::Key(key) => self.handle_key(key),
            Event::Mouse(_) => {}
            Event::Resize(_column_count, _row_count) => {}
        }
    }

    pub fn process_data(&self, data: RepositoryMessage) {
        if let RepositoryMessage::FromRepository(msg) = data {}
    }
}
