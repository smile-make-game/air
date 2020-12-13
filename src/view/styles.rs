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

pub fn subject_text_style() -> Style {
    Style::default()
        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        .bg(Color::White)
        .fg(Color::Black)
}

pub fn content_text_style() -> Style {
    Style::default()
        .bg(Color::Black)
        .fg(Color::Gray)
}
