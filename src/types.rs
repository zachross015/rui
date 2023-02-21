use std::fmt::Debug;
use crate::builder::Builder;

/// Type for communicating Builder / Error signals that result from the output 
/// of the `Builder` class. Should be used to chain together builders via ? notation.
pub type ViewBuilder<'a> = &'a mut Builder;

/// Trait for defining how a struct can be interpreted as a view.
pub trait View: Debug {

    /// Defines how a *V*iew *B*uilde*r* is used with the fields of this struct 
    /// to define a view.
    ///
    /// # Arguments
    ///
    /// * `v` - Shorthand for `vbr`, the injected builder to *build* a view for.
    ///
    /// # Examples 
    ///
    /// A "Hello World" view can be constructed fairly simply using just a text 
    /// field.
    /// ```
    /// #[derive(Debug)]
    /// struct HelloWorld { }
    /// impl View for HelloWorld {
    ///     fn vbr<'a>(&'a mut self, v: &'a mut Builder) -> Result {
    ///         v.text("Hello World")
    ///     }
    /// }
    /// ```
    fn view<'a>(&'a self, v: ViewBuilder<'a>) -> ViewBuilder<'a>; 
}
