use vbr::types::{View, Result};
use vbr::builder::Builder;
use vbr::application::{Application, WebApplication};


#[derive(Debug)]
struct Text {
    text: String 
}
impl Text {
    pub fn new(text: &str) -> Self {
        Self { text: text.to_string() }
    }
}
impl View for Text {
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
impl View for Dummy {
    fn view<'a>(&'a self, v: &'a mut Builder) -> Result {
        v.view(Text::new("Hello World")).bold()
    }
}

fn main() {
    WebApplication::new(Dummy::new()).run();
}

