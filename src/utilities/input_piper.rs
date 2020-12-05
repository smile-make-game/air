use anyhow::Result;
use crossterm::event;
pub use crossterm::event::Event;
use event::KeyModifiers;

pub enum Input {
    Tick,
    Event(Event),
}

pub fn tick_or_event() -> Result<Input> {
    let result: Input;
    if event::poll(std::time::Duration::from_millis(200))? {
        let event = event::read()?;
        log::debug!("get event from poll: {:?}", event);
        result = Input::Event(event)
    } else {
        result = Input::Tick;
    }

    Ok(result)
}

pub fn should_exit(event: &Event) -> bool {
    if let Event::Key(key) = event {
        if key.code == event::KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
            log::info!("should exit");
            return true;
        }
    }
    return false;
}
