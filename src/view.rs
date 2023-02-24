use std::fmt::Debug;

use crate::style::{
    Style,
    Color,
    Alignment,
    Font,
};

#[derive(Debug)]
enum Contents {
    Subviews(Vec<View>),
    Value(String),
    Empty,
}

impl Default for Contents {
    fn default() -> Self {
        Contents::Empty
    }
}

#[derive(Debug, Default)]
pub struct View {
    properties: Vec<Style>,
    contents: Contents,
}

impl View {

    pub fn empty() -> Self {
        View::default()
    }

    pub fn text(t: impl Into<String>) -> Self {
        View {
            properties: Vec::default(),
            contents: Contents::Value(t.into()),
        }
    }

    pub fn container(v: Vec<View>) -> Self {
        View {
            properties: Vec::default(),
            contents: Contents::Subviews(v),
        }
    }

    fn add_style(mut self, style: Style) -> View {
        self.properties.push(style);
        self
    }

    pub fn padding(self, top: u16, left: u16, bottom: u16, right: u16) -> View {
        self.add_style(Style::Padding(top, left, bottom, right))
    }   

    pub fn foreground_color(self, c: Color) -> View {
        self.add_style(Style::ForegroundColor(c))
    }   

    pub fn background_color(self, c: Color) -> View {
        self.add_style(Style::BackgroundColor(c))
    }   

    pub fn alignment(self, a: Alignment) -> View {
        self.add_style(Style::Alignment(a))
    }

    pub fn bold(self) -> View {
        self.add_style(Style::Bold)
    }

    pub fn font(self, f: Font) -> View {
        self.add_style(Style::Font(f))
    }
}
