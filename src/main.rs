use std::marker::PhantomData;

use rui::property::{Property, Customizable};
use rui::view::{View, Text};
// use vbr::application::{Application, WebApplication};

macro_rules! view {
    ($e:expr) => {
        Box::new($e) 
    };
}


#[derive(Debug, Default)]
struct Undefined;

#[derive(Debug, Default)]
struct Vertical;

#[derive(Debug, Default)]
struct Horizontal;

#[derive(Debug, Default)]
struct Stack<T> {
    views: Vec<Box<dyn View>>,
    properties: Vec<Property>,
    direction: PhantomData<T>
}

impl Stack<Undefined> {

    pub fn vertical(self) -> Stack<Vertical> {
        Stack::<Vertical> {
            views: self.views,
            properties: self.properties,
            direction: PhantomData,
        }
    }
    
    pub fn horizontal(self) -> Stack<Horizontal> {
        Stack::<Horizontal> {
            views: self.views,
            properties: self.properties,
            direction: PhantomData,
        }
    }

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
