use rui::property::{Customizable, Color};
use rui::view::View;


fn hello_world() -> View {
   View::text("Hello World!")
       .foreground_color(Color::rgba(100, 100, 100, 100))
}

fn stacked() -> View {
    View::hstack(vec!{
        hello_world(),
        View::text("Goodbye World")
    })
}


pub trait Viewable {
    fn create() -> Self;  
    fn view(&self) -> View; 
}

pub fn main() {
    println!("{:#?}", stacked())
}
