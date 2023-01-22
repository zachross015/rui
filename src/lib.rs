mod prototype;
pub mod types;
pub mod builder;

use types::{View, Result};
use builder::Builder;

struct Dummy {}
impl View for Dummy  {
    fn vbr(&mut self, v: &mut Builder) -> Result {
        v.container(|v| {
            v.container(|v| {
                v.text("Hello World")?;
                v.text("Goodbye World")
            })?;
            v.text("Another world")
        })
    } 
}

fn main() {
    let mut builder = Builder::new();
    let d = Dummy {};
    let a = builder.view(d);
    println!("{:#?}, {:#?}", builder, a);
}

