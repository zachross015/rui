use std::collections::LinkedList;
use std::fmt::Display;
use super::types::{Result, View};
use super::prototype::{Prototype, Style};

/// Structure for building user interfaces idiomatically.
///
/// Most user interfaces can be built using a combination of vstacks, hstacks, zstacks, text,
/// images, and inputs. This class takes advantage of this (in a manner similar to SwiftUI) by
/// maintaining an underlying structure for a DOMTree, which can be lazily loaded and dynamically
/// altered using state management.
///
/// This class should never be explicitly created, as the `Application` should handle the Builder's
/// lifecycle. Instead, the properties here should be used in the trait immplementation of `View`.
/// This is where the View should be contstructed on a per-struct basis.
#[derive(Debug)]
pub struct Builder {
    tree: LinkedList<Prototype>,
    linked_views: Vec<Box<dyn View>>
}

impl Builder {

    /// Constructs a new builder by initializing the internal tree and linked_views to being empty.
    pub fn new() -> Self {
        Self { 
            tree: LinkedList::new(),
            linked_views: Vec::new(),
        }
    }

    /// Constructs a container (e.g. vstack, hstack, etc) in the current builder.
    ///
    /// # Arguments
    ///
    /// * `F` - A function which takes a Builder as an argument. Builds the internal view before
    /// appending it to the current builder.
    ///
    /// # Examples
    ///
    /// A dummy view which contains the strings "hello" and "world" in different text fields can be
    /// made using a container:
    /// ```
    /// #[derive(Debug)]
    /// struct Dummy { }
    /// impl View for Dummy  {
    ///     fn vbr<'a>(&'a mut self, v: &'a mut Builder) -> Result {
    ///         v.container(|v| {
    ///             v.text("Hello")?;
    ///             v.text("World")
    ///         })
    ///     } 
    /// }
    /// ```
    /// Note use of a closure as an anonymous function allows for simplifying the process of
    /// expanding out nested views. We can even make a nested container using similar notation
    /// ```
    /// #[derive(Debug)]
    /// struct Dummy { }
    /// impl View for Dummy  {
    ///     fn vbr<'a>(&'a mut self, v: &'a mut Builder) -> Result {
    ///         v.container(|v| {
    ///             v.container(|v| {
    ///                 v.text("Hello")?;
    ///                 v.text("World")
    ///             })
    ///         })
    ///     } 
    /// }
    /// ```
    pub fn container<F>(&mut self, vbr: F) -> Result where F: Fn(&mut Builder) -> Result {
        let mut b = Builder::new();
        vbr(&mut b);

        let p = Prototype::container(b);
        self.tree.push_back(p);
        self
    }

    pub fn text(&mut self, t: &str) -> Result {
        let p = Prototype::value(t);
        self.tree.push_back(p);
        self
    }

    /// Constructs a custom (i.e. user created) view in the current builder.
    ///
    /// # Arguments
    ///
    /// * `T` - A custom, pre-defined view.
    pub fn view<T: View + 'static>(&mut self, mut t: T) -> Result {
        t.vbr(self);
        self.linked_views.push(Box::new(t));
        self
    }

    fn add_style(&mut self, style: Style) -> Result {
        self.tree.back_mut().unwrap().add_style(style);
        self
    }

    pub fn padding(&mut self, top: i8, left: i8, bottom: i8, right: i8) -> Result {
        self.add_style(Style::Padding(top, left, bottom, right))
    }   

    pub fn foreground_color(&mut self, r: u8, g: u8, b: u8, a: u8) -> Result {
        self.add_style(Style::ForegroundColor(r, g, b, a))
    }   

    pub fn background_color(&mut self, r: u8, g: u8, b: u8, a: u8) -> Result {
        self.add_style(Style::BackgroundColor(r, g, b, a))
    }   
}

impl Display for Builder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.tree.iter().map(|x| x.to_string()).collect::<String>();
        write!(f, "{}", s)
    }
}
