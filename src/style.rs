use std::fmt::Display;

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
}

#[derive(Debug)]
pub struct Font {
    size: u16,
}
impl Font {
    pub fn size(size: u16) -> Self {
        Self { size: size }
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

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Style::Padding(t, l, b, r) => write!(f, "padding: {}px {}px {}px {}px;", t, l, b, r),
            Style::ForegroundColor(c) => write!(f, "color: rgba({}, {}, {}, {});", c.r, c.g, c.b, c.a),
            Style::BackgroundColor(c) => write!(f, "background-color: rgba({}, {}, {}, {});", c.r, c.g, c.b, c.a),
            Style::Direction(a) => match a {
                Direction::Horizontal => write!(f, "flex-direction: row;"),
                Direction::Vertical => write!(f, "flex-direction: column;"),
            }
            Style::Alignment(a) => match a {
                Alignment::Leading => write!(f, "align-items: flex-start;"),
                Alignment::Center => write!(f, "align-items: baseline;"),
                Alignment::Trailing => write!(f, "align-items: flex-end;"),
            }
            Style::Bold => write!(f, "font-weight: bold;"),
            Style::Font(fs) => write!(f, "font-size: {}px;", fs.size),
        }
    }
}

