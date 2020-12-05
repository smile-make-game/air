use super::dep::*;

pub struct ErrorPage {
    _vm: Rc<ErrorPageViewMode>,

    message: MessageComponent,
}

impl From<Rc<ErrorPageViewMode>> for ErrorPage {
    fn from(view_model: Rc<ErrorPageViewMode>) -> Self {
        let message = MessageComponent::from(view_model.message.clone());
        Self {
            _vm: view_model,
            message,
        }
    }
}

impl Widget for &ErrorPage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.message.render(area, buf);
    }
}

#[derive(Default)]
pub struct ErrorPageViewMode {
    pub message: Rc<MessageComponentViewModel>,
}
