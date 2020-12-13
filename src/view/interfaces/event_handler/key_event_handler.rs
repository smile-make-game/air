pub use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub trait KeyEventHandler {
    fn handle_key(&self, key: &KeyEvent);
}
