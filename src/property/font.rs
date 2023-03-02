#[derive(Debug)]
pub struct Font {
    size: u16,
}
impl Font {
    pub fn new(size: u16) -> Self {
        Self { size: size }
    }

    pub fn size(&self) -> u16 {
        self.size
    }
}


