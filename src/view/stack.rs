use std::marker::PhantomData;

use crate::property::Property;
use crate::view::{View, Viewable};

#[derive(Debug, Default)]
pub struct Undefined;

#[derive(Default)]
pub struct Vertical;

#[derive(Default)]
pub struct Horizontal;

#[derive(Default)]
pub struct Stack<T> {
    views: Vec<View>,
    properties: Vec<Property>,
    direction: PhantomData<T>
}

impl Stack<Undefined> {
    pub fn new(views: Vec<View>) -> Self {
        Stack::default()
    }
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

impl Viewable for Stack<Horizontal> {
    fn view(&self) -> View {
        View::HStack
    }
}

impl Viewable for Stack<Vertical> {
    fn view(&self) -> View {
        View::HStack
    }
}
