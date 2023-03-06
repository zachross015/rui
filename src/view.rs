use crate::property::{Customizable, Property};

#[derive(Debug)]
pub enum ViewType {
    Empty,
    Text(String),
    HStack(Vec<View>),
    VStack(Vec<View>),
}

#[derive(Debug)]
pub struct View {
    view_type: ViewType,
    properties: Vec<Property>
}

impl View {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            view_type: ViewType::Text(text.into()),
            properties: Vec::default(),
        }
    }

    pub fn hstack(stack: Vec<View>) -> Self {
        Self {
            view_type: ViewType::HStack(stack),
            properties: Vec::default(),
        }
    }
}

impl Customizable for View {
    fn properties(&mut self) -> &mut Vec<Property> {
        &mut self.properties
    }
}

