use super::dep::*;

pub struct CharacterPage {
    vm: Rc<CharacterPageViewMode>,

    title_list: ListComponent,
    character_detail: CharacterDetailComponent,
}

impl From<Rc<CharacterPageViewMode>> for CharacterPage {
    fn from(view_model: Rc<CharacterPageViewMode>) -> Self {
        Self {
            vm: view_model.clone(),

            title_list: ListComponent::from(view_model.character_list.clone()),
            character_detail: CharacterDetailComponent::from(view_model.character_detail.clone()),
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

        let detail_area = chunks[1];
        self.character_detail.render(detail_area, buf);
    }
}

pub struct CharacterPageViewMode {
    pub character_list: Rc<ListComponentViewModel>,
    pub character_detail: Rc<CharacterDetailComponentViewModel>,

    focused: RefCell<bool>,

    pub page_title: Rc<RefCell<String>>,
    pub brief_list: RefCell<Vec<Rc<CharacterBrief>>>,
}

impl Default for CharacterPageViewMode {
    fn default() -> Self {
        let page_title = Rc::new(RefCell::new("Characters".to_owned()));

        let title_list = ListComponentViewModel::default();
        title_list.title.replace("activated characters".to_owned());
        title_list.focused.replace(true);

        let event_content = MessageComponentViewModel::default();
        event_content
            .title
            .replace(Some("event content".to_owned()));

        Self {
            character_list: Rc::new(title_list),
            character_detail: Rc::new(CharacterDetailComponentViewModel::default()),

            focused: RefCell::new(false),

            page_title,
            brief_list: RefCell::new(vec![]),
        }
    }
}

impl DataProcessor for CharacterPageViewMode {
    fn process_data(&self, msg: &FromRepositoryMessageItem) -> Result<()> {
        match msg {
            FromRepositoryMessageItem::Insert(payload) => {
                payload.character_list.iter().for_each(|item| {
                    self.brief_list.borrow_mut().push(Rc::new(CharacterBrief {
                        id: item.id.clone(),

                        name: RefCell::new(item.name.clone()),

                        is_selected: RefCell::new(false),
                    }));
                });
            }
            FromRepositoryMessageItem::Update(payload) => {
                payload.character_list.iter().for_each(|item| {
                    if let Some(old_item) =
                        self.brief_list.borrow().iter().find(|oi| item.id == oi.id)
                    {
                        old_item.name.replace(item.name.clone());
                    }
                });
            }
            FromRepositoryMessageItem::Remove(payload) => {
                payload.character_list.iter().for_each(|item| {
                    if let Some(index) = self
                        .brief_list
                        .borrow()
                        .iter()
                        .position(|oi| item.id == oi.id)
                    {
                        self.brief_list.borrow_mut().remove(index);
                    }
                });
            }
        }
        self.character_list.items.replace(
            self.brief_list
                .borrow()
                .iter()
                .map(|item| item.clone() as Rc<(dyn ListComponentItem)>)
                .collect::<Vec<Rc<dyn ListComponentItem>>>(),
        );

        Ok(())
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
        self.character_list.handle_key(key);
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

pub struct CharacterBrief {
    id: String,
    name: RefCell<String>,

    is_selected: RefCell<bool>,
}

impl ListComponentItem for CharacterBrief {
    fn get_name(&self) -> String {
        self.name.borrow().clone()
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
