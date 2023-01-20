use std::collections::LinkedList;
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
    tree: LinkedList<LinkedList<Prototype>>
}

impl Builder {
    pub fn new() -> Self {
        Self { 
            tree: LinkedList::new()
        }
    }
}

impl Builder {

    pub fn container<F>(&mut self, build: F) -> Result where F: Fn(&mut Builder) -> Result {
        match build(self) {
            Ok(_u) => Err(Error::new("container not implemented")),
            Err(e) => Err(e),
        }
    }

    pub fn vstack<F>(&mut self, build: F) -> Result where F: Fn(&mut Builder) -> Result {
        self.container(build)
    }

    pub fn hstack<F>(&mut self, build: F) -> Result where F: Fn(&mut Builder) -> Result {
        self.container(build)
    }

    pub fn zstack<F>(&mut self, build: F) -> Result where F: Fn(&mut Builder) -> Result {
        self.container(build)
    }

    pub fn text(&mut self, t: &str) -> Result {
        Err(Error::new("text properly implemented"))
    }

    pub fn view<T: View>(&mut self, mut t: T) -> Result {
        match t.vbr(self) {
            Ok(_u) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
