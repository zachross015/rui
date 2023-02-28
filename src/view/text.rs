use crate::{View, Customizable};
use crate::view::Empty;
use crate::property::Property;

#[derive(Debug)]
pub struct Text {
    text: String,
    properties: Vec<Property>,
}
impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), properties: Vec::new(), }
    }
}
impl View for Text {
    fn view(&self) -> Box<dyn View> {
        Box::new(Empty)
    }
}

impl Customizable for Text {
    fn properties(&mut self) -> &mut Vec<Property> {
        &mut self.properties
    }
}
