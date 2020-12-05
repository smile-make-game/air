use super::{
    component::view_model::*,
    composite::view_model::*,
    interfaces::{event_handler::KeyEventHandler, evolute::Evolute},
};
use crossterm::event::*;
use std::{cell::RefCell, rc::Rc};

pub struct ViewModel {
    // misc
    pub edge_border: Rc<EdgeComponentViewModel>,
    pub tab: Rc<TabComponentViewModel>,
    // pages
    pub event_page: Rc<EventPageViewMode>,
    pub system_page: Rc<SystemPageViewMode>,

    // internal fields
    _is_loading: RefCell<bool>,
}

impl ViewModel {
    pub fn process_input(&self, event: &Event) {
        match event {
            Event::Key(key) => self.handle_key(key),
            Event::Mouse(_) => {}
            Event::Resize(_column_count, _row_count) => {}
        }
    }
}

impl Default for ViewModel {
    fn default() -> Self {
        let edge_border = Rc::new(EdgeComponentViewModel::default());

        let event_page = Rc::new(EventPageViewMode::default());
        let system_page = Rc::new(SystemPageViewMode::default());

        let tab = Rc::new(TabComponentViewModel::default());
        tab.register_tab(event_page.clone());
        tab.register_tab(system_page.clone());

        Self {
            tab,
            edge_border,
            event_page,
            system_page,

            _is_loading: RefCell::new(false),
        }
    }
}
impl Evolute for ViewModel {
    fn evolute(&self, evolution: &crate::model::Evolution) {
        self.event_page.evolute(evolution);
    }
}

impl KeyEventHandler for ViewModel {
    fn handle_key(&self, key: &KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                if key.modifiers == KeyModifiers::NONE {
                    self.tab.select_next();
                }
            }
            KeyCode::BackTab => {
                if key.modifiers == KeyModifiers::SHIFT {
                    self.tab.select_prev();
                }
            }
            _ => {}
        }
    }
}
