#![allow(dead_code)]
#![allow(unused)]

use boring::hash::hash;
use boring::hash::MessageDigest;
use std::cell::RefCell;
use std::hash::Hash;

fn main() {
    println!("Hello, world!");
}

struct Node {
    hash: Vec<u8>,
    left: Option<RefCell<Box<Node>>>,
    right: Option<RefCell<Box<Node>>>,
}

impl Node {
    fn new(
        hash: Vec<u8>,
        left: Option<RefCell<Box<Node>>>,
        right: Option<RefCell<Box<Node>>>,
    ) -> Self {
        Node { hash, left, right }
    }
}

struct Tree {
    root: Node,
    data_len: usize,
}

impl Tree {
    /// Must be called with data of non-zero length
    pub fn new_with_data(data: &[String]) -> Self {
        assert!(!data.is_empty());

        let data_len = data.len();
        let root = Self::build_tree(data);

        Tree { root, data_len }
    }

    pub fn root(&self) -> Node {
        todo!()
    }

    pub fn contains(&self, data: String) -> bool {
        todo!()
    }

    // Must be called with data of non-zero length
    fn build_tree(data: &[String]) -> Node {
        assert!(!data.is_empty());

        let root = Node::new(vec![], None, None);

        // Construct the leaf nodes first
        let mut layer: Vec<Node> = data
            .iter()
            .map(|data| {
                let hash = hash(MessageDigest::sha256(), data.as_bytes())
                    .unwrap()
                    .to_vec();

                Node::new(hash, None, None)
            })
            .collect();

        // Iter over data and construct layers from the leaf up to the root

        root
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
