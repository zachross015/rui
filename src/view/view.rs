use std::fmt::Debug;

pub trait View: Debug {
    fn view(&self) -> Box<dyn View>;
}
