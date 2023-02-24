use vbr::{View, Viewable, view, style::Alignment};
// use vbr::application::{Application, WebApplication};


#[derive(Debug)]
struct Dummy; 

impl Viewable for Dummy {
    fn view(&self) -> View {
        view("I'm a dummy View")
            .padding(0, 0, 0, 0)
            .alignment(Alignment::Center);
        view(
            ("Hello", "My", "Name")
        )
    } 
}

pub fn main() {
    println!("{:#?}", view(Dummy));
}
