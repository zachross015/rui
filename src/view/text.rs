use crate::view::{View, Viewable};
use crate::property::{Customizable, Property};

pub struct Text {
    text: String,
    properties: Vec<Property>,
}
impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), properties: Vec::new(), }
    }
}
impl Viewable for Text {
    fn view(&self) -> View {
        View::Text
    }
}

impl Customizable for Text {
    fn properties(&mut self) -> &mut Vec<Property> {
        &mut self.properties
    }
}
