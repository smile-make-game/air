use super::dep::*;

pub struct SystemPage {
    vm: Rc<SystemPageViewMode>,
}

impl From<Rc<SystemPageViewMode>> for SystemPage {
    fn from(view_model: Rc<SystemPageViewMode>) -> Self {
        Self { vm: view_model }
    }
}

impl Widget for &SystemPage {
    fn render(self, _area: Rect, _buf: &mut Buffer) {
        if !*self.vm.focused.borrow() {
            return;
        }

    }
}

pub struct SystemPageViewMode {
    pub focused: RefCell<bool>,

    pub page_title: Rc<RefCell<String>>,
}

impl Default for SystemPageViewMode {
    fn default() -> Self {
        let page_title = Rc::new(RefCell::new("System".to_owned()));

        Self {
            focused: RefCell::new(true),

            page_title,
        }
    }
}

impl DataProcessor for SystemPageViewMode {
    fn process_data(&self, msg: &FromRepositoryMessageItem) -> Result<()> {
        Ok(())
    }
}

impl KeyEventHandler for SystemPageViewMode {
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

impl Page for SystemPageViewMode {
    fn get_title(&self) -> String {
        self.page_title.borrow().clone()
    }

    fn set_focused(&self, focused: bool) {
        self.focused.replace(focused);
    }
}
