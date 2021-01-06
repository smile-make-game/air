use super::dep::*;
use crate::view::interfaces::data_processor::*;
use anyhow::Result;

pub struct QuestPage {
    vm: Rc<QuestPageViewMode>,

    title_list: ListComponent,
    event_detail: MessageComponent,
}

impl From<Rc<QuestPageViewMode>> for QuestPage {
    fn from(view_model: Rc<QuestPageViewMode>) -> Self {
        Self {
            vm: view_model.clone(),

            title_list: ListComponent::from(view_model.title_list.clone()),
            event_detail: MessageComponent::from(view_model.event_detail.clone()),
        }
    }
}

impl Widget for &QuestPage {
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

pub struct QuestPageViewMode {
    pub title_list: Rc<ListComponentViewModel>,
    pub event_detail: Rc<MessageComponentViewModel>,

    focused: RefCell<bool>,

    pub page_title: Rc<RefCell<String>>,
    pub event_items: RefCell<Vec<Rc<QuestItem>>>,
}

impl Default for QuestPageViewMode {
    fn default() -> Self {
        let page_title = Rc::new(RefCell::new("Quests".to_owned()));

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

impl DataProcessor for QuestPageViewMode {
    fn process_data(&self, msg: &FromRepositoryMessageItem) -> Result<()> {
        match msg {
            FromRepositoryMessageItem::Insert(payload) => {
                payload.quest_list.iter().for_each(|item| {
                    self.event_items.borrow_mut().push(Rc::new(QuestItem {
                        id: item.id.clone(),

                        subject: RefCell::new(item.title.clone()),
                        content: RefCell::new(format!("content of {}", item.title)),

                        is_selected: RefCell::new(false),
                    }));
                });
            }
            FromRepositoryMessageItem::Update(payload) => {
                payload.quest_list.iter().for_each(|item| {
                    if let Some(old_item) =
                        self.event_items.borrow().iter().find(|oi| item.id == oi.id)
                    {
                        old_item.subject.replace(item.title.clone());
                    }
                });
            }
            FromRepositoryMessageItem::Remove(payload) => {
                payload.quest_list.iter().for_each(|item| {
                    if let Some(index) = self
                        .event_items
                        .borrow()
                        .iter()
                        .position(|oi| item.id == oi.id)
                    {
                        self.event_items.borrow_mut().remove(index);
                    }
                });
            }
        }

        self.title_list.items.replace(
            self.event_items
                .borrow()
                .iter()
                .map(|item| item.clone() as Rc<(dyn ListComponentItem)>)
                .collect::<Vec<Rc<dyn ListComponentItem>>>(),
        );
        Ok(())
    }
}

impl KeyEventHandler for QuestPageViewMode {
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

impl Page for QuestPageViewMode {
    fn get_title(&self) -> String {
        self.page_title.borrow().clone()
    }

    fn set_focused(&self, focused: bool) {
        self.focused.replace(focused);
    }
}

pub struct QuestItem {
    id: String,
    subject: RefCell<String>,
    content: RefCell<String>,

    is_selected: RefCell<bool>,
}

impl ListComponentItem for QuestItem {
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
