pub trait Application {
    fn run(&self) -> String;
}

pub mod web_application;
pub use crate::application::web_application::WebApplication;
