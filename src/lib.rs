mod prototype;
pub mod types;
pub mod builder;

use types::{View, Result};
use builder::Builder;

struct Dummy {}
impl View for Dummy  {
    fn vbr(&mut self, v: &mut Builder) -> Result {
        v.hstack(|v| {
            v.vstack(|v| {
                v.text("Hello World");
                v.text("Goodbye World")
            })
        })
    } 
}

fn main() {
    let mut builder = Builder::new();
    builder.view(Dummy {});
    println!("{:#?}", builder);
}
