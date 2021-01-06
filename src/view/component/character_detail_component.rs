use super::dep::*;

pub struct CharacterDetailComponent {
    vm: Rc<CharacterDetailComponentViewModel>,
}

impl From<Rc<CharacterDetailComponentViewModel>> for CharacterDetailComponent {
    fn from(view_model: Rc<CharacterDetailComponentViewModel>) -> Self {
        Self { vm: view_model }
    }
}

impl Widget for &CharacterDetailComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut border = Block::default()
            .borders(Borders::ALL)
            .border_type(border_normal_type());
        let title = self.vm.title.borrow();
        border = border.title(format!(" {} ", title));
        border.render(area, buf);

        // let chunks = Layout::default()
        //     .direction(Direction::Vertical)
        //     .margin(1)
        //     .constraints(vec![Constraint::Length(1), Constraint::Min(1)])
        //     .split(area);
    }
}

pub struct CharacterDetailComponentViewModel {
    pub title: RefCell<String>,
}

impl Default for CharacterDetailComponentViewModel {
    fn default() -> Self {
        Self {
            title: RefCell::new("character detail".to_owned()),
        }
    }
}
