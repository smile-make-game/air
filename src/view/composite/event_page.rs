use super::dep::*;

pub struct EventPage {
    vm: Rc<EventPageViewMode>,

    title_list: ListComponent,
    event_detail: MessageComponent,
}

impl From<Rc<EventPageViewMode>> for EventPage {
    fn from(view_model: Rc<EventPageViewMode>) -> Self {
        Self {
            vm: view_model.clone(),

            title_list: ListComponent::from(view_model.title_list.clone()),
            event_detail: MessageComponent::from(view_model.event_detail.clone()),
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

        let list_area = chunks[0];
        self.title_list.render(list_area, buf);

        let content_area = chunks[1];
        self.event_detail.render(content_area, buf);
    }
}

pub struct EventPageViewMode {
    pub title_list: Rc<ListComponentViewModel>,
    pub event_detail: Rc<MessageComponentViewModel>,

    focused: RefCell<bool>,

    pub page_title: Rc<RefCell<String>>,
    pub event_items: RefCell<Vec<Rc<EventItem>>>,
}

impl Default for EventPageViewMode {
    fn default() -> Self {
        let page_title = Rc::new(RefCell::new("Events".to_owned()));

        let title_list = ListComponentViewModel::default();
        title_list.title.replace("activated events".to_owned());
        title_list.focused.replace(true);

        let event_content = MessageComponentViewModel::default();
        event_content
            .title
            .replace(Some("event content".to_owned()));

        Self {
            title_list: Rc::new(title_list),
            event_detail: Rc::new(event_content),

            focused: RefCell::new(true),

            page_title,
            event_items: RefCell::new(vec![]),
        }
    }
}

impl Evolute for EventPageViewMode {
    fn evolute(&self, evolution: &Evolution) {
        let data = evolution.new_data;

        data.events.iter().for_each(|event| {
            self.event_items.borrow_mut().push(Rc::new(EventItem {
                subject: RefCell::new(event.subject.clone()),
                content: RefCell::new(event.content.clone()),

                is_selected: RefCell::new(false),
            }))
        });

        self.title_list.items.replace(
            self.event_items
                .borrow()
                .iter()
                .map(|item| item.clone() as Rc<(dyn ListComponentItem)>)
                .collect::<Vec<Rc<dyn ListComponentItem>>>(),
        );
    }
}

impl KeyEventHandler for EventPageViewMode {
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

        let event_items = self.event_items.borrow();
        let selected_event = event_items.iter().find(|event| *event.is_selected.borrow());
        if let Some(event) = selected_event {
            self.event_detail
                .subject
                .replace(Some(event.subject.borrow().clone()));
            self.event_detail
                .content
                .replace(Some(event.content.borrow().clone()));
        } else {
            self.event_detail.subject.replace(None);
            self.event_detail.content.replace(None);
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

pub struct EventItem {
    subject: RefCell<String>,
    content: RefCell<String>,

    is_selected: RefCell<bool>,
}

impl ListComponentItem for EventItem {
    fn get_name(&self) -> String {
        self.subject.borrow().clone()
    }

    fn is_selected(&self) -> bool {
        *self.is_selected.borrow()
    }

    fn select(&self) {
        self.is_selected.replace(true);
    }

    fn unselect(&self) {
        self.is_selected.replace(false);
    }
}
