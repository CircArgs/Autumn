extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Node<'a, T: PartialOrd> {
    data: &'a T,
    left: Option<Box<Node<'a, T>>>,
    right: Option<Box<Node<'a, T>>>,
}

impl<'a, T: PartialOrd> Node<'a, T> {
    pub fn new(data: &'a T) -> Self {
        return Node {
            data,
            left: None,
            right: None,
        };
    }
    pub fn insert(&mut self, data: &'a T) {
        let target_node = if data < self.data {
            &mut self.left
        } else {
            &mut self.right
        };
        match target_node {
            &mut None => *target_node = Some(Box::new(Node::new(data))),
            &mut Some(ref mut boxed_node) => boxed_node.insert(data),
        }
    }
}
