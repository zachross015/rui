use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

use crate::view::View;
use crate::prototype::Prototype;
use crate::style::{
    Style,
    Color,
    Alignment,
    Direction,
    Font,
};

/// Structure for building user interfaces idiomatically.
///
/// Most user interfaces can be built using a combination of vstacks, hstacks, zstacks, text,
/// images, and inputs. This class takes advantage of this (in a manner similar to SwiftUI) by
/// maintaining an underlying structure for a UITree, which can be lazily loaded and dynamically
/// altered using state management.
///
/// This class should never be explicitly created, as the `Application` should handle the Builder's
/// lifecycle. Instead, the properties here should be used in the trait immplementation of `View`.
/// This is where the View should be contstructed on a per-struct basis.
#[derive(Debug)]
pub struct Builder {
    tree: LinkedList<Prototype>,
    linked_views: Vec<Rc<RefCell<dyn View>>>
}

impl Builder {

    /// Constructs a new builder by initializing the internal tree and linked_views to being empty.
    pub fn new() -> Self {
        Self { 
            tree: LinkedList::new(),
            linked_views: Vec::new(),
        }
    }

    pub fn tree(&self) -> &LinkedList<Prototype> {
        &self.tree
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
    /// #[derive(debug)]
    /// struct Dummy { }
    /// impl View for Dummy  {
    ///     fn vbr<'a>(&'a mut self, v: &'a mut Builder) -> &'a mut Builder {
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
    ///     fn vbr<'a>(&'a mut self, v: &'a mut Builder) -> &'a mut Builder {
    ///         v.container(|v| {
    ///             v.container(|v| {
    ///                 v.text("Hello")?;
    ///                 v.text("World")
    ///             })
    ///         })
    ///     } 
    /// }
    /// ```
    fn container<F>(&mut self, vbr: F) -> &mut Builder where F: Fn(&mut Builder) -> &mut Builder {
        let mut b = Builder::new();
        vbr(&mut b);

        let p = Prototype::container(b);
        self.tree.push_back(p);
        self
    }

    fn direction(&mut self, d: Direction) -> &mut Builder {
        self.add_style(Style::Direction(d))
    }


    pub fn vstack<F>(&mut self, vbr: F) -> &mut Builder where F: Fn(&mut Builder) -> &mut Builder {
        self.container(vbr).direction(Direction::Vertical).alignment(Alignment::Center)
    }

    pub fn hstack<F>(&mut self, vbr: F) -> &mut Builder where F: Fn(&mut Builder) -> &mut Builder {
        self.container(vbr).direction(Direction::Horizontal).alignment(Alignment::Center)
    }

    pub fn text(&mut self, t: &str) -> &mut Builder {
        let p = Prototype::value(t);
        self.tree.push_back(p);
        self.alignment(Alignment::Center)
    }

    /// Constructs a custom (i.e. user created) view in the current builder.
    ///
    /// # Arguments
    ///
    /// * `T` - A custom, pre-defined view.
    pub fn view<T: View + 'static>(&mut self, t: T) -> &mut Builder {
        t.view(self);
        self.linked_views.push(Rc::new(RefCell::new(t)));
        self
    }

    fn add_style(&mut self, style: Style) -> &mut Builder {
        self.tree.back_mut().unwrap().add_style(style);
        self
    }

    pub fn padding(&mut self, top: u16, left: u16, bottom: u16, right: u16) -> &mut Builder {
        self.add_style(Style::Padding(top, left, bottom, right))
    }   

    pub fn foreground_color(&mut self, c: Color) -> &mut Builder {
        self.add_style(Style::ForegroundColor(c))
    }   

    pub fn background_color(&mut self, c: Color) -> &mut Builder {
        self.add_style(Style::BackgroundColor(c))
    }   

    pub fn alignment(&mut self, a: Alignment) -> &mut Builder {
        self.add_style(Style::Alignment(a))
    }

    pub fn bold(&mut self) -> &mut Builder {
        self.add_style(Style::Bold)
    }

    pub fn font(&mut self, f: Font) -> &mut Builder {
        self.add_style(Style::Font(f))
    }
}

