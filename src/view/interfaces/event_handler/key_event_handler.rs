use crossterm::event::KeyEvent;

pub trait KeyEventHandler {
    fn handle_key(&self, key: &KeyEvent);
}
