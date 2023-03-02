use crate::view::{View, Viewable};

pub struct Empty;
impl Viewable for Empty {
    fn view(&self) -> View {
        View::Empty
    }
}
