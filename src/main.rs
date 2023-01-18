mod vbr {

    pub struct Builder {
        // TODO: Implement these
    }

    impl Builder {
        pub fn vstack(&self, v: &'static mut Builder) -> &'static mut Builder {
            panic!("vstack not yet implemented")
        }

        pub fn hstack(&self, v: &'static mut Builder) -> &'static mut Builder {
            panic!("hstack not yet implemented")
        }

        pub fn zstack(&self, v: &'static mut Builder) -> &'static mut Builder {
            panic!("zstack not yet implemented")
        }

        pub fn text(&self, text: &str) -> &'static mut Builder {
            panic!("text not yet implemented")
        }
    }

    pub struct Error;
    impl Error {
        pub fn new() -> Self {
            Self {  }
        }
    }

    pub type Result<'a> = std::result::Result<&'a mut Builder, Error>;

    pub trait View {
        fn vbr(&self, v: &mut Builder) -> Result; 
    }
    
} /* vbr */

struct HStack {}
impl vbr::View for HStack  {
    fn vbr(&self, v: &mut vbr::Builder) -> vbr::Result {
        Ok(v.vstack({
            v.hstack({
                v.text("Hello World");
                v.text("Goodbye World")
            })
        }))
    } 
}


fn main() {
}
