use super::dep::*;

pub struct TabComponent {
    vm: Rc<TabComponentViewModel>,
}

impl From<Rc<TabComponentViewModel>> for TabComponent {
    fn from(view_model: Rc<TabComponentViewModel>) -> Self {
        Self { vm: view_model }
    }
}

impl Widget for &TabComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let pages = self.vm.pages.borrow();
        let tab_titles: Vec<Spans> = pages.iter().map(|p| {
            let title = format!(" {} ", p.get_title());
            Spans::from(title)
        }).collect();
        Tabs::new(tab_titles)
            .select(self.vm.selected.borrow().clone())
            .highlight_style(selection_normal_style())
            .render(area, buf);
    }
}

pub struct TabComponentViewModel {
    pub selected: RefCell<usize>,

    pub pages: RefCell<Vec<Rc<dyn Page>>>,
}

impl TabComponentViewModel {
    pub fn register_tab(&self, page: Rc<dyn Page>) {
        self.pages.borrow_mut().push(page);
    }

    pub fn select_next(&self) {
        let pages = self.pages.borrow();
        let len = pages.len();
        if len > 0 {
            let cur = *self.selected.borrow();
            let next = (cur + 1) % len;
            log::debug!("select next tab: index={}", next);
            pages[cur].set_focused(false);
            pages[next].set_focused(true);
            self.selected.replace(next);
        }
    }

    pub fn select_prev(&self) {
        let pages = self.pages.borrow();
        let len = pages.len();
        if len > 0 {
            let cur = *self.selected.borrow();
            let prev = (cur + 1) % len;
            log::debug!("select next tab: index={}", prev);
            pages[cur].set_focused(false);
            pages[prev].set_focused(true);
            self.selected.replace(prev);
        }
    }
}

impl Default for TabComponentViewModel {
    fn default() -> Self {
        Self {
            selected: RefCell::new(0),

            pages: RefCell::new(vec![]),
        }
    }
}
