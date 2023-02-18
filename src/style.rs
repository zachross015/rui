#[derive(Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub enum Alignment {
    Leading,
    Center,
    Trailing,
}

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

#[derive(Debug)]
pub enum Style {
    Padding(u16, u16, u16, u16),
    ForegroundColor(Color),
    BackgroundColor(Color),
    Direction(Direction),
    Alignment(Alignment),
    Bold,
    Font(Font),
}

