use std::fmt::Debug;

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
pub enum Property {
    Padding(u16, u16, u16, u16),
    ForegroundColor(Color),
    BackgroundColor(Color),
    Alignment(Alignment),
    Bold,
    Font(Font),
}

pub trait Customizable: Debug + Sized {
    fn properties(&mut self) -> &mut Vec<Property>;

    fn add_style(mut self, style: Property) -> Self {
        self.properties().push(style);
        self
    }

    fn padding(self, top: u16, left: u16, bottom: u16, right: u16) -> Self {
        self.add_style(Property::Padding(top, left, bottom, right))
    }   

    fn foreground_color(self, c: Color) -> Self {
        self.add_style(Property::ForegroundColor(c))
    }   

    fn background_color(self, c: Color) -> Self {
        self.add_style(Property::BackgroundColor(c))
    }   

    fn alignment(self, a: Alignment) -> Self {
        self.add_style(Property::Alignment(a))
    }

    fn bold(self) -> Self {
        self.add_style(Property::Bold)
    }

    fn font(self, f: Font) -> Self {
        self.add_style(Property::Font(f))
    }
}
