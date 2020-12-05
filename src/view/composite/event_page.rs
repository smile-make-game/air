use super::dep::*;

pub struct EventPage {
    vm: Rc<EventPageViewMode>,

    title_list: ListComponent,
}

impl From<Rc<EventPageViewMode>> for EventPage {
    fn from(view_model: Rc<EventPageViewMode>) -> Self {
        let title_list = ListComponent::from(view_model.title_list.clone());
        Self {
            vm: view_model,
            title_list,
        }
    }
}

impl Widget for &EventPage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !*self.vm.focused.borrow() {
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(area);
        let list_component_area = chunks[0];
        self.title_list.render(list_component_area, buf);
    }
}

pub struct EventPageViewMode {
    pub focused: RefCell<bool>,

    pub page_title: Rc<RefCell<String>>,
    pub title_list: Rc<ListComponentViewModel>,
}

impl Default for EventPageViewMode {
    fn default() -> Self {
        let page_title = Rc::new(RefCell::new("Event".to_owned()));
        let title_list_title = "activated events";
        let title_list = ListComponentViewModel::default();
        title_list.title.replace(title_list_title.to_owned());

        Self {
            focused: RefCell::new(true),

            page_title,
            title_list: Rc::new(title_list),
        }
    }
}

impl Evolute for EventPageViewMode {
    fn evolute(&self, evolution: &Evolution) {
        let data = evolution.new_data;
        let mut titles: Vec<String> = vec![];

        data.events
            .iter()
            .for_each(|event| titles.push(event.subject.clone()));

        self.title_list.items.replace(titles);
    }
}

impl KeyEventHandler for EventPageViewMode {
    fn handle_key(&self, key: &crossterm::event::KeyEvent) {
        match key.code {
            crossterm::event::KeyCode::Enter => {}
            crossterm::event::KeyCode::Left => {}
            crossterm::event::KeyCode::Right => {}
            crossterm::event::KeyCode::Up => {}
            crossterm::event::KeyCode::Down => {}
            crossterm::event::KeyCode::PageUp => {}
            crossterm::event::KeyCode::PageDown => {}
            crossterm::event::KeyCode::Esc => {}
            _ => {}
        }
    }
}

impl Page for EventPageViewMode {
    fn get_title(&self) -> String {
        self.page_title.borrow().clone()
    }

    fn set_focused(&self, focused: bool) {
        self.focused.replace(focused);
    }
}
