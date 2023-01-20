use std::collections::LinkedList;
use super::types::{Error, Result, View};
use super::prototype::{Type, Prototype};

#[derive(Debug)]
pub struct Builder {
    // TODO: Implement these
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
