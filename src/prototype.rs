use super::builder::Builder;

#[derive(Debug)]
pub enum Style {
    Padding(i8, i8, i8, i8),
    ForegroundColor(u8, u8, u8, u8),
    BackgroundColor(u8, u8, u8, u8),
}

#[derive(Debug)]
pub enum Type {
    Atom,
    Value(String),
    Container(Builder)
}

#[derive(Debug)]
pub struct Prototype {
    contents: Type,
    styles: Vec<Style>,
}

impl Prototype {

    pub fn atom() -> Self {
        Self { 
            contents: Type::Atom,
            styles: vec![],
        }
    }

    pub fn container(b: Builder) -> Self {
        Self { 
            contents: Type::Container(b),
            styles: vec![],

        }
    }

    pub fn value(value: &str) -> Self {
        Self { 
            contents: Type::Value(value.to_string()),
            styles: vec![],
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }
}
