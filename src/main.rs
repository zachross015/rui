mod vbr;

struct Dummy {}
impl vbr::View for Dummy  {
    fn vbr(&mut self, v: &mut vbr::Builder) -> vbr::Result {
        v.hstack(|v| {
            v.vstack(|v| {
                v.text("Hello World");
                v.text("Goodbye World")
            })
        })
    } 
}

fn main() {
    let mut builder = vbr::Builder::new();
    builder.view(Dummy {});
    println!("{:#?}", builder);
}
