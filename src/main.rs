use vbr::{View, Customizable};
use vbr::view::Text;
// use vbr::application::{Application, WebApplication};

macro_rules! view {
    ($e:expr) => {
        Box::new($e) 
    };
}

#[derive(Debug)]
struct Dummy; 
impl View for Dummy {
    fn view(&self) -> Box<dyn View> {
        view! {
            Text::new("Hello World").padding(0, 0, 0, 0)
        }
    } 
}

#[derive(Debug)]
struct OtherDummy;
impl View for OtherDummy {
    fn view(&self) -> Box<dyn View> {
        view! {
            Dummy
        }
    }
}

pub fn main() {
    println!("{:#?}", OtherDummy.view().view());
}
