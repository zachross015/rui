use crate::prototype::{Type, Prototype};
use crate::style::{Style, Direction, Alignment};
use crate::types::View;
use crate::builder::Builder;
use crate::application::Application;

pub struct WebApplication<T: View> {
    root_view: T
}

impl<T: View> WebApplication<T> {
    pub fn new(view: T) -> Self {
        Self { root_view: view }
    }

    fn match_style(&self, style: &Style) -> String {
        match style {
            Style::Padding(t, l, b, r) => format!("padding: {}px {}px {}px {}px;", t, l, b, r),
            Style::ForegroundColor(c) => format!("color: rgba({}, {}, {}, {});", c.red(), c.green(), c.blue(), c.alpha()),
            Style::BackgroundColor(c) => format!("background-color: rgba({}, {}, {}, {});", c.red(), c.green(), c.blue(), c.alpha()),
            Style::Direction(a) => match a {
                Direction::Horizontal => format!("flex-direction: row;"),
                Direction::Vertical => format!("flex-direction: column;"),
            }
            Style::Alignment(a) => match a {
                Alignment::Leading => format!("align-items: flex-start;"),
                Alignment::Center => format!("align-items: baseline;"),
                Alignment::Trailing => format!("align-items: flex-end;"),
            }
            Style::Bold => format!("font-weight: bold;"),
            Style::Font(f) => format!("font-size: {}px;", f.size()),
        }
    }

    fn match_prototype(&self, prototype: &Prototype) -> String {
        let style = prototype.styles().iter().map(|x| self.match_style(x)).collect::<String>();
        match &prototype.contents() {
            Type::Value(s) => format!("<div style=\"display:flex;justify-content:center;align-items:center;\"><p style=\"margin:0;display:inline-flex;{}\">{}</p></div>", style, s),
            Type::Container(b) => format!("<div style=\"display:flex;{}\">{}</div>", style, self.format_builder(b)),
        } 
        
    }

    fn format_builder(&self, vbr: &Builder) -> String {
        let s = vbr.tree().iter().map(|x| self.match_prototype(x)).collect::<String>();
        format!("{}", s)
    }
}

impl<T: View> Application for WebApplication<T> {
    fn run(&mut self) {
        let mut vbr = Builder::new();
        self.root_view.view(&mut vbr);
        self.format_builder(&vbr)
    }    
}
