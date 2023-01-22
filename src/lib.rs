mod prototype;
pub mod types;
pub mod builder;

use types::{View, Result};
use builder::Builder;

struct Dummy {
    msg: String
}
impl View for Dummy  {
    fn vbr(&mut self, v: &mut Builder) -> Result {
        v.container(|v| {
            v.container(|v| {
                v.text(&self.msg)?;
                v.text("Goodbye World")
            })?;
            v.text("Another world")?;
            v.text("Testing")
        })
    } 
}


fn main() {
    let mut builder = Builder::new();
    let d = Dummy { msg: "Hello".to_string() };
    let a = builder.view(d);
    println!("{:#?}, {:#?}", builder, a);
}

