use std::fmt::Display;

use crate::types::StaticView;
use crate::builder::Builder;

pub struct BasicApplication<T: StaticView> {
    root_view: T
}
impl<T: StaticView> BasicApplication<T> {
    pub fn new(view: T) -> Self {
        Self { root_view: view }
    }
}
impl<T: StaticView> Display for BasicApplication<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vbr = Builder::new();
        self.root_view.view(&mut vbr);
        write!(f, "<body style=\"position: absolute; min-height: 100vh; min-width: 100vw; margin: 0; display: flex; flex-direction: row; justify-content: center; align-items: center;\">{}</body>", vbr)
    }
}
