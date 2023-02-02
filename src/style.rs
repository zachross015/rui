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
pub enum Style {
    Padding(i8, i8, i8, i8),
    ForegroundColor(Color),
    BackgroundColor(Color),
    Direction(Direction),
    Alignment(Alignment),
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
                Alignment::Leading => write!(f, "justify-content: flex-start;"),
                Alignment::Center => write!(f, "justify-content: center;"),
                Alignment::Trailing => write!(f, "justify-content: flex-end;"),
            }
        }
    }
}

