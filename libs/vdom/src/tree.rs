use std::{rc::Rc, cell::RefCell};

#[derive(PartialEq)]
pub struct Tree<T> {
    pub value: Option<T>,
    pub children: Vec<Rc<RefCell<Tree<T>>>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, node: Tree<T>) {
        let child = Rc::new(RefCell::new(node));
        self.children.push(child);
    }

    pub fn nth_child(&self, n: usize) -> Rc<RefCell<Tree<T>>> {
        Rc::clone(&self.children[n])
    }

    pub fn width(&self) -> usize {
        self.children.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn add_child_test() {
        let mut tree = Tree::new();
        let mut child = Tree::new();
        child.value = Some(0);
        tree.add_child(child);
        
        let binding = tree.nth_child(0);
        let val: &RefCell<_> = binding.borrow();
        assert_eq!(val.borrow().value, Some(0));
    }

    #[test]
    fn add_children_test() {
        let mut tree = Tree::new();
        for i in 0..10 {
            let mut child = Tree::new();
            child.value = Some(i);
            tree.add_child(child);
        }

        // Make sure all the elements were added.
        assert_eq!(tree.width(), 10);
        
        // Make sure each element has the same number it was inserted with.
        for i in 0..10 {
            let binding = tree.nth_child(i);
            let val: &RefCell<_> = binding.borrow();
            assert_eq!(val.borrow().value, Some(i));
        }
    }
}
