use super::builder::Builder;
use super::prototype::Prototype;

#[derive(Debug)]
pub struct Error {
    msg: String
}
impl Error {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.to_string() }
    }
}

pub type Result<'a> = std::result::Result<(), Error>;

pub trait View {
    fn vbr(&mut self, v: &mut Builder) -> Result; 
}
