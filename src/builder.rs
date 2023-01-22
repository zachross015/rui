use std::collections::LinkedList;
use std::rc::Rc;
use std::cell::RefCell;

use super::types::{Error, Result, View};
use super::prototype::{Type, Prototype};

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
    tree: LinkedList<Prototype>
}

impl Builder {
    pub fn new() -> Self {
        Self { 
            tree: LinkedList::new()
        }
    }
}

impl Builder {

    pub fn container<F>(&mut self, vbr: F) -> Result where F: Fn(&mut Builder) -> Result {
        let mut b = Builder::new();
        vbr(&mut b)?;

        let mut p = Prototype::container(b);
        self.tree.push_back(p);
        Ok(())
    }

    pub fn text(&mut self, t: &str) -> Result {
        let mut p = Prototype::value(t);
        self.tree.push_back(p);
        Ok(())
    }

    pub fn view<T: View>(&mut self, mut t: T) -> Result {
        t.vbr(self)
    }
}
