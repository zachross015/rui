use vbr::types::{StaticView, Result};
use vbr::builder::Builder;
use vbr::application::BasicApplication;


#[derive(Debug)]
struct Text {
    text: String 
}
impl Text {
    pub fn new(text: &str) -> Self {
        Self { text: text.to_string() }
    }
}
impl StaticView for Text {
    fn view<'a>(&'a self, v: &'a mut Builder) -> Result {
        v.text(&self.text)
    }
}

#[derive(Debug)]
struct Dummy { }
impl Dummy {
    pub fn new() -> Self {
        Self {  }
    }
}
impl StaticView for Dummy {
    fn view<'a>(&'a self, v: &'a mut Builder) -> Result {
        v.view(Text::new("Hello World")).bold()
    }
}

fn main() {
    println!("{}", BasicApplication::new(Dummy::new()));
}

