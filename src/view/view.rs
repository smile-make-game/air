use super::{component::*, composite::*, view_model::ViewModel};
use std::rc::Rc;
use tui::{layout::*, widgets::Widget};

pub struct View {
    _vm: Rc<ViewModel>,

    edge_border: EdgeComponent,

    tab: TabComponent,

    event_page: EventPage,
}

impl From<Rc<ViewModel>> for View {
    fn from(view_model: Rc<ViewModel>) -> Self {
        Self {
            _vm: view_model.clone(),
            edge_border: EdgeComponent::from(view_model.edge_border.clone()),
            tab: TabComponent::from(view_model.tab.clone()),

            event_page: EventPage::from(view_model.event_page.clone()),
        }
    }
}

impl Widget for &View {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        self.edge_border.render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(1),
            ])
            .split(area);

        self.tab.render(chunks[0], buf);
        self.event_page.render(chunks[1], buf);
    }
}
