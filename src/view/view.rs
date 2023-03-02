pub enum View {
    Empty,
    Text,
    VStack,
    HStack,
    Custom(Box<dyn Viewable>)
}

impl View {
    pub fn new<T: Viewable + 'static>(t: T) -> Self {
        Self::Custom(Box::new(t))
    }
}

pub trait Viewable {
    fn view(&self) -> View;
}

