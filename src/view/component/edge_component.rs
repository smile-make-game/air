use super::dep::*;

pub struct EdgeComponent {
    _vm: Rc<EdgeComponentViewModel>,
}

impl From<Rc<EdgeComponentViewModel>> for EdgeComponent {
    fn from(view_model: Rc<EdgeComponentViewModel>) -> Self {
        EdgeComponent { _vm: view_model }
    }
}

impl Widget for &EdgeComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_type(border_edge_type())
            .render(area, buf);
    }
}

pub struct EdgeComponentViewModel {}

impl Default for EdgeComponentViewModel {
    fn default() -> Self {
        EdgeComponentViewModel {}
    }
}
