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
        if *self.vm.focused.borrow() {
        }
    }
}

pub struct CharacterDetailComponentViewModel {
    focused: RefCell<bool>,
}
