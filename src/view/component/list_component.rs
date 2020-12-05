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
        let (outer_border_type, outer_border_style) = if *self.vm.focused.borrow() {
            (border_highlight_type(), border_highlight_style())
        } else {
            (border_normal_type(), border_normal_style())
        };
        let title = self.vm.title.borrow();
        let outer_border = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ",title))
            .border_style(outer_border_style)
            .border_type(outer_border_type);

        let mut list_state = ListState::default();
        list_state.select(*self.vm.selected.borrow());
        let mut index = 0;
        let list_items: Vec<ListItem> = self
            .vm
            .items
            .borrow()
            .iter()
            .map(|item| {
                index += 1;
                ListItem::new(format!("{}. {}", index, item))
            })
            .collect();

        let list = List::new(list_items)
            .block(outer_border)
            .highlight_style(outer_border_style)
            .highlight_symbol("> ");
        StatefulWidget::render(list, area, buf, &mut list_state);
    }
}

pub struct ListComponentViewModel {
    pub focused: RefCell<bool>,
    pub selected: RefCell<Option<usize>>,
    pub items: RefCell<Vec<String>>,
    pub title: RefCell<String>,
}

impl Default for ListComponentViewModel {
    fn default() -> Self {
        Self {
            focused: RefCell::new(false),
            selected: RefCell::new(None),
            items: RefCell::new(vec![]),
            title: RefCell::new(String::new()),
        }
    }
}
