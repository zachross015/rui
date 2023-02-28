use std::fmt::Debug;
use crate::view::View;

#[derive(Debug)]
pub struct Empty;
impl View for Empty {
    fn view(&self) -> Box<dyn View> {
        panic!("Algorithm should stop progressing here.")
    }
}
