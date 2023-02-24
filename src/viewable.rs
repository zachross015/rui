use std::fmt::Debug;
use crate::View;

pub trait Viewable: Debug {
    fn view(&self) -> View;
}

pub fn view<T: Viewable>(t: T) -> View {
    t.view()
}

impl Viewable for str {
    fn view(&self) -> View {
        View::text(self)
    }
}

impl<T: Viewable> Viewable for Vec<T> {
    fn view(&self) -> View {
        View::container(self.iter().map(|x| view(x)).collect())
    }
}
