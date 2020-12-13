use super::dep::*;

pub struct MessageComponent {
    vm: Rc<MessageComponentViewModel>,
}

impl From<Rc<MessageComponentViewModel>> for MessageComponent {
    fn from(view_model: Rc<MessageComponentViewModel>) -> Self {
        MessageComponent { vm: view_model }
    }
}

impl Widget for &MessageComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut border = Block::default()
            .borders(Borders::ALL)
            .border_type(border_normal_type());
        let title = self.vm.title.borrow();
        if let Some(title) = title.as_deref() {
            border = border.title(format!(" {} ", title));
        }
        border.render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(vec![Constraint::Length(1), Constraint::Min(1)])
            .split(area);

        if let Some(text) = self.vm.subject.borrow().as_deref() {
            Paragraph::new(format!("Subject: {}", text)).style(subject_text_style()).render(chunks[0], buf);
        }

        if let Some(text) = self.vm.content.borrow().as_deref() {
            Paragraph::new(text).style(content_text_style()).render(chunks[1], buf);
        }
    }
}

pub struct MessageComponentViewModel {
    pub title: RefCell<Option<String>>,
    pub subject: RefCell<Option<String>>,
    pub content: RefCell<Option<String>>,
}

impl Default for MessageComponentViewModel {
    fn default() -> Self {
        MessageComponentViewModel {
            title: RefCell::new(None),
            subject: RefCell::new(None),
            content: RefCell::new(None),
        }
    }
}
