use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Stdout, Write};
use tui::{backend::CrosstermBackend, Terminal};

pub fn configure_terminal() -> Terminal<CrosstermBackend<Stdout>> {
    log::debug!("enable raw mode");
    enable_raw_mode().unwrap();

    let mut stdout = stdout();
    log::debug!("setup stdout mode");
    execute!(stdout, EnableMouseCapture, EnterAlternateScreen,).unwrap();

    log::debug!("take over terminal");
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();

    terminal
}

pub fn reset_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    log::debug!("disable raw mode");
    disable_raw_mode().unwrap();

    log::debug!("setup stdout mode");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();

    log::debug!("show cursor");
    terminal.show_cursor().unwrap();
}
