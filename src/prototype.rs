use std::fmt::Display;

use super::builder::Builder;

#[derive(Debug)]
pub enum Style {
    Padding(i8, i8, i8, i8),
    ForegroundColor(u8, u8, u8, u8),
    BackgroundColor(u8, u8, u8, u8),
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Style::Padding(t, l, b, r) => write!(f, "padding: {}px {}px {}px {}px;", t, l, b, r),
            Style::ForegroundColor(r, g, b, a) => write!(f, "color: rgba({}, {}, {}, {});", r, g, b, a),
            Style::BackgroundColor(r, g, b, a) => write!(f, "background-color: rgba({}, {}, {}, {});", r, g, b, a),
        }
    }
}

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

impl Display for Prototype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let style_concat = self.styles.iter().map(|x| x.to_string()).collect::<String>();
        let style = if !style_concat.is_empty() {
            format!(" style=\"{}\"", style_concat)
        } else {
            "".to_string()
        };
        match &self.contents {
            Type::Value(s) => write!(f, "<span{}>{}</span>", style, s),
            Type::Container(b) => write!(f, "<div{}>{}</div>", style, b),
        } 
    }
}
