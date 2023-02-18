use Type::{Value, Container};
use crate::builder::Builder;
use crate::style::Style;

#[derive(Debug)]
pub enum Type {
    Value(String),
    Container(Builder)
}

#[derive(Debug)]
pub struct Prototype {
    contents: Type,
    styles: Vec<Style>,
}

impl Prototype {

    pub fn container(b: Builder) -> Self {
        Self { 
            contents: Container(b),
            styles: vec![],

        }
    }

    pub fn value(value: &str) -> Self {
        Self { 
            contents: Value(value.to_string()),
            styles: vec![],
        }
    }

    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }

    pub fn styles(&self) -> &Vec<Style> {
        &self.styles
    }

    pub fn contents(&self) -> &Type {
        &self.contents
    }
}

