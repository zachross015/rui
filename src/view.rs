use std::fmt;

#[derive(Debug, Clone)]
pub enum ViewType {
    Empty,
    Spacer,
    Divider,
    Text(String),
    HStack(Vec<View>),
    VStack(Vec<View>),
    ZStack(Vec<View>)
}

impl fmt::Display for ViewType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ViewType::Empty => write!(f, ""),
            ViewType::Spacer => write!(f, "<div class='spacer'></div>"),
            ViewType::Divider => write!(f, "<div class='divider'></div>"),
            ViewType::Text(val) => write!(f, "<span class='text'>{}</span>", val),
            ViewType::HStack(val) => write!(f, "<div class='hstack'>{}</span>", val.into_iter().map(|x| format!("{}", x)).collect::<String>()),
            ViewType::VStack(val) => write!(f, "<div class='vstack'>{}</span>", val.into_iter().map(|x| format!("{}", x)).collect::<String>()),
            ViewType::ZStack(val) => write!(f, "<div class='zstack'>{}</span>", val.into_iter().map(|x| format!("{}", x)).collect::<String>()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct View {
    view_type: ViewType,
}

impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.view_type)
    }
}

impl View {
    pub fn new(view_type: ViewType) -> Self {
        Self { view_type: view_type }
    }
}

pub fn empty() -> View {
    View::new(ViewType::Empty)
}

pub fn spacer() -> View {
    View::new(ViewType::Spacer)
}

pub fn divider() -> View {
    View::new(ViewType::Divider)
}

pub fn text(text: &str) -> View {
    View::new(ViewType::Text(text.to_string()))
}

pub fn hstack(views: &[View]) -> View {
    View::new(ViewType::HStack(views.to_vec()))
}

pub fn vstack(views: &[View]) -> View {
    View::new(ViewType::VStack(views.to_vec()))
}

pub fn zstack(views: &[View]) -> View {
    View::new(ViewType::ZStack(views.to_vec()))
}
