pub trait Page {
    fn get_title(&self) -> String;
    fn set_focused(&self, focused: bool);
}
