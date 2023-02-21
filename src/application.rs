use crate::builder::Builder;
pub trait Application {

    /// Use this method to actually start the application. This method starts the application by
    /// constructing a builder and injecting the builder into the `run` method, which is specified
    /// by the trait implementation.
    fn start(&mut self) {
        let mut vbr = Builder::new();
        self.run(&mut vbr);
    }

    /// Specifies how the application is to be run for the specific domain this trait is
    /// implemented for.
    ///
    /// # Arguments 
    /// * `vbr` - The ViewBuilder to build the application to.
    fn run(&mut self, vbr: &mut Builder);
}


pub mod web_application;
pub use crate::application::web_application::WebApplication;
