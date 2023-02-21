use vbr::style::Font;
use vbr::types::{View, ViewBuilder};
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
    fn view<'a>(&self, v: ViewBuilder<'a>) -> ViewBuilder<'a> {
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
    fn view<'a>(&self, v: ViewBuilder<'a>) -> ViewBuilder<'a> {
        v.view(Text::new("Hello World"))
            .bold()
            .font(Font::new(32))
    }
}

fn main() {
    println!("{:#?}", WebApplication::new(Dummy::new()).run());
}

