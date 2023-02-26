use std::fmt::Debug;

pub mod empty;
pub mod text;

pub use empty::Empty;
pub use text::Text;

pub trait View: Debug {
    fn view(&self) -> Box<dyn View>;
}
