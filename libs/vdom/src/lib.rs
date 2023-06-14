pub mod tree;

use std::{rc::Rc, fmt::Display, cell::RefCell};

use tree::*;

pub enum VElement {
    Text(String),
    View(Rc<View>),
}

impl Display for VElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VElement::Text(s) => write!(f, "Text(\"{s}\")"),
            VElement::View(v) => write!(f, "View({v})"),
        }
    }
}

type VNode = Tree<VElement>;

impl VNode {
    pub fn text(string: impl Into<String>) -> Self {
        let mut tree = VNode::new();
        tree.value = Some(VElement::Text(string.into()));
        tree
    }

    pub fn view(view: View) -> Self {
        let mut tree = VNode::new();
        tree.value = Some(VElement::View(Rc::new(view)));
        tree
    }


    /** Helper function for formatting nested VNodes when printing to a string.
     *
     */
    fn nested_fmt(&self, depth: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.as_ref().map(|x| write!(f, "{:indent$}{}", "", x, indent=depth*4));
        for child in self.children.iter() {
            let val = child.borrow();
            val.nested_fmt(depth + 1, f)?;
        }
        Ok(())
    }

}

impl Display for VNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.nested_fmt(0, f)
    }
}

pub struct View {
    render: Box<dyn Fn() -> VNode>,
}

impl View {
    pub fn new<T: Fn() -> VNode + 'static>(render: T) -> Self {
        Self { render: Box::new(render)  }
    }

    pub fn render(&self) -> VNode {
        (self.render)()
    }
}

impl Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render()) 
    }
}

macro_rules! view {
    ($func:expr) => {
        View::new(|| $func) 
    };
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{View, VNode, VElement};

    #[test]
    fn print_velement_text_test() {
        assert_eq!(format!("{}", VElement::Text(String::from("Hello World"))), "Text(\"Hello World\")")
    }

    #[test]
    fn print_vnode_text_test() {
        assert_eq!(format!("{}", VNode::text("Hello World")), "Text(\"Hello World\")")
    }

 
    #[test]
    fn print_text_test() {
        let view = View::new(|| VNode::text("Hello World"));
        assert_eq!(format!("{}", view), "Text(\"Hello World\")")
    }

    #[test]
    fn print_view_test() {
        let view = view! {
            VNode::view(view! {
                VNode::text("Hello World")
            })
        };
        assert_eq!(format!("{}", view), "View(Text(\"Hello World\"))")
    }

}

