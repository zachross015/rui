use crate::property::{ 
    Property,
    Color,
    Font,
    Alignment,
};

pub trait Customizable: Sized {
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
