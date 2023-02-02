mod prototype;
pub mod style;
pub mod types;
pub mod builder;

use style::{Color, Alignment, Direction};
use types::{View, Result};
use builder::Builder;

#[derive(Debug)]
struct Dummy {
    msg: String
}
impl Dummy {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.to_string() }
    }
}
impl View for Dummy  {
    fn vbr<'a>(&'a mut self, v: &'a mut Builder) -> Result {
        v.vstack(|v| {
            v.hstack(|v| {
                v.text(&self.msg).padding(10, 10, 10, 10);
                v.text("Goodbye World")
            });
            v.text("Another world")
                .padding(100, 100, 100, 100);
            v.text("Red Text")
                .foreground_color(Color::rgba(255, 0, 0, 255))
                .background_color(Color::rgba(0, 0, 0, 255))
                .alignment(Alignment::Trailing)
        })
    } 
}

#[derive(Debug)]
struct External {
}
impl View for External {
    fn vbr<'a>(&'a mut self, v: &'a mut Builder) -> Result {
        v.view(Dummy::new("External"))
    }
}

fn main() {
    let mut builder = Builder::new();
    let a = builder.view(External {});
    println!("<body style=\"position: absolute; min-height: 100vh; min-width: 100vw; margin: 0; display: flex; flex-direction: row; justify-content: center;
    align-items: center;\">{}</body>", a);
}

