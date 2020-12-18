use super::dep::*;

pub struct CharacterPage {
    vm: Rc<CharacterPageViewMode>,

    title_list: ListComponent,
}

impl From<Rc<CharacterPageViewMode>> for CharacterPage {
    fn from(view_model: Rc<CharacterPageViewMode>) -> Self {
        Self {
            vm: view_model.clone(),

            title_list: ListComponent::from(view_model.title_list.clone()),
        }
    }
}

impl Widget for &CharacterPage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !*self.vm.focused.borrow() {
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(area);

        let list_area = chunks[0];
        self.title_list.render(list_area, buf);
    }
}

pub struct CharacterPageViewMode {
    pub title_list: Rc<ListComponentViewModel>,

    focused: RefCell<bool>,

    pub page_title: Rc<RefCell<String>>,
}

impl Default for CharacterPageViewMode {
    fn default() -> Self {
        let page_title = Rc::new(RefCell::new("Characters".to_owned()));

        let title_list = ListComponentViewModel::default();
        title_list.title.replace("activated events".to_owned());
        title_list.focused.replace(true);

        let event_content = MessageComponentViewModel::default();
        event_content
            .title
            .replace(Some("event content".to_owned()));

        Self {
            title_list: Rc::new(title_list),

            focused: RefCell::new(true),

            page_title,
        }
    }
}

impl Evolute for CharacterPageViewMode {
    fn evolute(&self, evolution: &Evolution) {
    }
}

impl KeyEventHandler for CharacterPageViewMode {
    fn handle_key(&self, key: &crossterm::event::KeyEvent) {
        if !*self.focused.borrow() {
            return;
        }
        match key.code {
            crossterm::event::KeyCode::Enter => {}
            crossterm::event::KeyCode::Esc => {}
            _ => {}
        }
        self.title_list.handle_key(key);
    }
}

impl Page for CharacterPageViewMode {
    fn get_title(&self) -> String {
        self.page_title.borrow().clone()
    }

    fn set_focused(&self, focused: bool) {
        self.focused.replace(focused);
    }
}
