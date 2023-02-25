use vbr::{View, Viewable, view, style::Alignment};
// use vbr::application::{Application, WebApplication};


#[derive(Debug)]
struct Dummy; 

impl Viewable for Dummy {
    fn view(&self) -> View {
        "I'm a dummy View".view()
            .padding(0, 0, 0, 0)
            .alignment(Alignment::Center)
    } 
}

pub fn main() {
    println!("{:#?}", view(Dummy));
}
