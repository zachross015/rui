mod prototype;
pub mod types;
pub mod builder;

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
        v.container(|v| {
            v.container(|v| {
                v.text(&self.msg);
                v.text("Goodbye World")
            });
            v.text("Another world")
                .padding(0, 0, 0, 0);
            v.text("Red Text")
                .foreground_color(255, 0, 0, 255)
                .background_color(0, 0, 0, 255)
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
    println!("{}", a);
}

