use std::fmt::Debug;
use crate::style::{Style, Color, Alignment, Font};

pub trait Customizable: Debug + Sized {
    fn properties(&mut self) -> &mut Vec<Style>;

    fn add_style(mut self, style: Style) -> Self {
        self.properties().push(style);
        self
    }

    fn padding(self, top: u16, left: u16, bottom: u16, right: u16) -> Self {
        self.add_style(Style::Padding(top, left, bottom, right))
    }   

    fn foreground_color(self, c: Color) -> Self {
        self.add_style(Style::ForegroundColor(c))
    }   

    fn background_color(self, c: Color) -> Self {
        self.add_style(Style::BackgroundColor(c))
    }   

    fn alignment(self, a: Alignment) -> Self {
        self.add_style(Style::Alignment(a))
    }

    fn bold(self) -> Self {
        self.add_style(Style::Bold)
    }

    fn font(self, f: Font) -> Self {
        self.add_style(Style::Font(f))
    }
}
