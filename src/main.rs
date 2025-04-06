#![allow(dead_code)]
#![allow(unused)]

use std::cell::RefCell;
use std::hash::Hash;

fn main() {
    println!("Hello, world!");
}

struct Node {
    data: String,
    left: RefCell<Option<Box<Node>>>,
    right: RefCell<Option<Box<Node>>>,
}

struct Tree {
    root: Node,
    data_len: usize,
}

impl Tree {
    fn new_with_data(data: &[String]) -> Self {
        todo!()
        // Tree {}
    }

    fn root(&self) -> Node {
        todo!()
    }

    fn contains(&self, data: String) -> bool {
        todo!()
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
