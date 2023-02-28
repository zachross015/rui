use crate::view::{View, Empty};
use crate::property::{Customizable, Property};

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
