#[derive(Debug)]
pub enum Type {
    Atom,
    Value(String),
    Container
}

#[derive(Debug)]
pub struct Prototype {
    contents: Type,
    background_color: [u8; 4],
    foreground_color: [u8; 4],
    padding: [f32; 4]
}

impl Prototype {

    pub fn atom() -> Self {
        Self { 
            contents: Type::Atom,
            background_color: [255, 255, 255, 255],
            foreground_color: [0, 0, 0, 255],
            padding: [0.0; 4]
        }
    }

    pub fn container() -> Self {
        Self { 
            contents: Type::Container,
            background_color: [255, 255, 255, 255],
            foreground_color: [0, 0, 0, 255],
            padding: [0.0; 4]
        }
    }

    pub fn value(value: &str) -> Self {
        Self { 
            contents: Type::Value(value.to_string()),
            background_color: [255, 255, 255, 255],
            foreground_color: [0, 0, 0, 255],
            padding: [0.0; 4]
        }
    }
}
