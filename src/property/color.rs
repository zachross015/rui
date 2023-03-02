#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8   
}
impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r: r, g: g, b: b, a: a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r: r, g: g, b: b, a: 255 }
    }

    pub fn red(&self) -> u8 {
        self.r
    }

    pub fn blue(&self) -> u8 {
        self.b
    }

    pub fn green(&self) -> u8 {
        self.g
    }

    pub fn alpha(&self) -> u8 {
        self.a
    }
}
