pub trait Application {
    fn run(&mut self);
}

pub mod web_application;
pub use crate::application::web_application::WebApplication;
