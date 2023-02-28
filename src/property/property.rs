use crate::property::{
    Color,
    Font,
    Alignment
};

#[derive(Debug)]
pub enum Property {
    Padding(u16, u16, u16, u16),
    ForegroundColor(Color),
    BackgroundColor(Color),
    Alignment(Alignment),
    Bold,
    Font(Font),
}

