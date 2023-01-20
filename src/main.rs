mod vbr;

struct Dummy {
    msg: String
}
impl Dummy {
    pub fn new() -> Self {
        Self { msg: "Thing".to_string() }
    }
}
impl vbr::View for Dummy  {
    fn vbr(&mut self, v: &mut vbr::Builder) -> vbr::Result {
        v.hstack(|v| {
            v.vstack(|v| {
                match v.text(&self.msg) {
                    Ok(_u) => (),
                    Err(e) => return Err(e),
                };
                v.text("Goodbye World")
            })
        })
    } 
}

fn main() {
    let mut builder = vbr::Builder::new();
    builder.view(Dummy::new());
    print!("{:#?}", builder);
}
