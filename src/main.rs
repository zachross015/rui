use rui::property::{Customizable};


#[derive(Debug)]
enum View {
    Empty,
    Text(String),
    HStack(Vec<View>),
    VStack(Vec<View>),
}

fn text(text: impl Into<String>) -> View {
    View::Text(text.into())
}

fn hstack(stack: Vec<View>) -> View {
    View::HStack(stack)
}

fn hello_world() -> View {
   text("Hello World!")
}

fn stacked() -> View {
    hstack(vec!{
        hello_world(),
        text("Goodbye World")
    })
}

pub fn main() {
    println!("{:#?}", stacked())
}
