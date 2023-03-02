use rui::property::{Customizable};
use rui::view::{View, Viewable, Text, Stack};
// use vbr::application::{Application, WebApplication};

macro_rules! view {
    ($e:expr) => {
        View::new($e)
    };
}


#[derive(Debug)]
struct Dummy; 
impl Viewable for Dummy {
    fn view(&self) -> View {
        view! {
            Text::new("Hello World").padding(0, 0, 0, 0)
        }
    } 
}

#[derive(Debug)]
struct OtherDummy;
impl Viewable for OtherDummy {
    fn view(&self) -> View {
        view! {
            Dummy
        }
    }
}

#[derive(Debug)]
struct StackedDummy;
impl Viewable for StackedDummy {
    fn view(&self) -> View {
        Stack::new({
            vec![Text::new("Hello World").view()]
        }).horizontal().view()
    }
}

pub fn main() {

}
