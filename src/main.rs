mod vbr {

    pub struct Builder {
        // TODO: Implement these
    }

    impl Builder {
        pub fn vstack<'a>(&self, v: Result<'a>) -> Result<'a> {
            panic!("vstack not yet implemented")
        }

        pub fn hstack<'a>(&self, v: Result<'a>) -> Result<'a> {
            panic!("hstack not yet implemented")
        }

        pub fn zstack<'a>(&self, v: Result<'a>) -> Result<'a> {
            panic!("zstack not yet implemented")
        }

        pub fn text<'a>(&self, text: &str) -> Result<'a> {
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
        v.vstack({
            v.hstack({
                v.text("Hello World");
                v.text("Goodbye World")
            })
        })
    } 
}


fn main() {
}
