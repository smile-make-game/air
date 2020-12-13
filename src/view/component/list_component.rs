use super::dep::*;

pub struct ListComponent {
    vm: Rc<ListComponentViewModel>,
}

impl From<Rc<ListComponentViewModel>> for ListComponent {
    fn from(view_model: Rc<ListComponentViewModel>) -> Self {
        ListComponent { vm: view_model }
    }
}

impl Widget for &ListComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (outer_border_type, outer_border_style, selection_style) = if *self.vm.focused.borrow()
        {
            (
                border_highlight_type(),
                border_highlight_style(),
                selection_highlight_style(),
            )
        } else {
            (
                border_normal_type(),
                border_normal_style(),
                selection_normal_style(),
            )
        };
        let title = self.vm.title.borrow();
        let outer_border = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", title))
            .border_style(outer_border_style)
            .border_type(outer_border_type);

        self.vm.selected_index();
        let mut list_state = ListState::default();
        list_state.select(self.vm.selected_index());

        let mut index = 0;
        let list_items: Vec<ListItem> = self
            .vm
            .items
            .borrow()
            .iter()
            .map(|item| {
                index += 1;
                ListItem::new(format!("{}. {}", index, item.get_name()))
            })
            .collect();

        let list = List::new(list_items)
            .block(outer_border)
            .highlight_style(selection_style)
            .highlight_symbol("> ");
        StatefulWidget::render(list, area, buf, &mut list_state);
    }
}

pub struct ListComponentViewModel {
    pub focused: RefCell<bool>,
    pub items: RefCell<Vec<Rc<dyn ListComponentItem>>>,
    pub title: RefCell<String>,
}

impl Default for ListComponentViewModel {
    fn default() -> Self {
        Self {
            focused: RefCell::new(false),
            items: RefCell::new(vec![]),
            title: RefCell::new(String::new()),
        }
    }
}

impl ListComponentViewModel {
    fn select_next(&self, n: usize) {
        let selected = self.selected_index();
        if let Some(selected) = selected {
            self.items.borrow()[selected].unselect();

            let len = self.items.borrow().len();
            let selected = selected + n;
            let selected = if selected >= len { len - 1 } else { selected };
            self.items.borrow()[selected].select();
        } else {
            self.items.borrow()[0].select();
        }
    }

    fn select_prev(&self, n: usize) {
        let selected = self.selected_index();

        if let Some(selected) = selected {
            self.items.borrow()[selected].unselect();

            let selected = if selected < n { 0 } else { selected - n };
            self.items.borrow()[selected].select();
        } else {
            self.items.borrow()[0].select();
        }
    }

    fn selected_index(&self) -> Option<usize> {
        let items = self.items.borrow();
        items.iter().position(|item| item.is_selected())
    }

    fn selected_item(&self) -> Option<Rc<dyn ListComponentItem>> {
        let items = self.items.borrow();
        items
            .iter()
            .find(|item| item.is_selected())
            .and_then(|item| Some(item.clone()))
    }
}

impl KeyEventHandler for ListComponentViewModel {
    fn handle_key(&self, key: &KeyEvent) {
        if !*self.focused.borrow() {
            return;
        }

        match key.code {
            KeyCode::Backspace => {}
            KeyCode::Enter => {}
            KeyCode::Left => {}
            KeyCode::Right => {}
            KeyCode::Up => {
                self.select_prev(1);
            }
            KeyCode::Down => {
                self.select_next(1);
            }
            KeyCode::PageUp => {
                self.select_prev(10);
            }
            KeyCode::PageDown => {
                self.select_next(10);
            }
            KeyCode::Esc => {
                if let Some(item) = self.selected_item() {
                    item.unselect();
                }
            }

            _ => {}
        }
    }
}

pub trait ListComponentItem {
    fn get_name(&self) -> String;
    fn is_selected(&self) -> bool;
    fn select(&self);
    fn unselect(&self);
}
