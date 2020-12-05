use tui::style::*;

pub fn border_highlight_style() -> Style {
    Style::default().fg(Color::LightCyan)
}

pub fn border_normal_style() -> Style {
    Style::default().fg(Color::Gray)
}

pub fn selection_highlight_style() -> Style {
    Style::default().fg(Color::Black).bg(Color::LightCyan)
}

pub fn selection_normal_style() -> Style {
    Style::default().fg(Color::Black).bg(Color::White)
}
