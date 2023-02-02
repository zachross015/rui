use std::fmt::Display;
use super::builder::Builder;
use Type::{Value, Container};

use super::style::Style;

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
}

impl Display for Prototype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let style = self.styles.iter().map(|x| x.to_string()).collect::<String>();
        match &self.contents {
            Value(s) => write!(f, "<div style=\"display:flex;justify-content:center;align-items:center;\"><p style=\"margin:0;display:inline-flex;{}\">{}</p></div>", style, s),
            Container(b) => write!(f, "<div style=\"display:flex;{}\">{}</div>", style, b),
        } 
    }
}
