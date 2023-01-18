mod view;
use crate::view::*;

fn dummy() -> View {
    text("Dummy Text")
}

fn hello_world() -> View {
    hstack(&[
           text("Hello World"),
           divider(),
           text("Goodbye World"),
           vstack(&[
                  dummy(),
                  text("Inner hello"),
                  if 34 < 35 { text("True Statement") } 
                  else { empty() },
                  if 34 > 35 { empty() }
                  else { text("False Statement") }
           ])
    ])
}

fn main() {
    println!("{}", hello_world());
}
