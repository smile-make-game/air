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
        Block::default()
            .borders(Borders::ALL)
            .border_type(border_normal_type())
            .render(area, buf);

        if let Some(text) = self.vm.text.borrow().as_deref() {
            let text_area = Rect::new(area.x + 4, area.y + 2, area.width - 8, area.height - 4);
            Paragraph::new(text).render(text_area, buf);
        }
    }
}

pub struct MessageComponentViewModel {
    pub text: RefCell<Option<String>>,
}

impl Default for MessageComponentViewModel {
    fn default() -> Self {
        MessageComponentViewModel {
            text: RefCell::new(None),
        }
    }
}
