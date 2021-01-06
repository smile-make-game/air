use super::{component::*, composite::*, view_model::ViewModel};
use std::rc::Rc;
use tui::{layout::*, widgets::Widget};

pub struct View {
    _vm: Rc<ViewModel>,

    edge_border: EdgeComponent,

    tab: TabComponent,

    quest_page: QuestPage,
    character_page: CharacterPage,
    system_page: SystemPage,
}

impl From<Rc<ViewModel>> for View {
    fn from(view_model: Rc<ViewModel>) -> Self {
        Self {
            _vm: view_model.clone(),
            edge_border: EdgeComponent::from(view_model.edge_border.clone()),
            tab: TabComponent::from(view_model.tab.clone()),

            quest_page: QuestPage::from(view_model.quest_page.clone()),
            character_page: CharacterPage::from(view_model.character_page.clone()),
            system_page: SystemPage::from(view_model.system_page.clone()),
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

        // render tab
        self.tab.render(chunks[0], buf);

        // render pages
        self.quest_page.render(chunks[1], buf);
        self.character_page.render(chunks[1], buf);
        self.system_page.render(chunks[1], buf);
    }
}
