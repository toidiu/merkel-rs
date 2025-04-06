#![allow(dead_code)]
#![allow(unused)]

use boring::hash::hash;
use boring::hash::Hasher;
use boring::hash::MessageDigest;
use std::cell::RefCell;
use std::hash::Hash;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct Node {
    hash: Vec<u8>,
    left: RefCell<Option<Box<Node>>>,
    right: RefCell<Option<Box<Node>>>,
}

impl Node {
    fn new(
        hash: Vec<u8>,
        left: RefCell<Option<Box<Node>>>,
        right: RefCell<Option<Box<Node>>>,
    ) -> Self {
        Node { hash, left, right }
    }

    fn hash(&self) -> &[u8] {
        &self.hash
    }
}

#[derive(Debug)]
struct Tree {
    root: Node,
    data_len: usize,
    depth: usize,
}

impl Tree {
    /// Must be called with data of non-zero length
    pub fn new_with_data(data: &[&str]) -> Self {
        assert!(!data.is_empty());

        let data_len = data.len();
        let (root, depth) = Self::build_tree(data);

        Tree {
            root,
            data_len,
            depth,
        }
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn contains(&self, data: String) -> bool {
        todo!()
    }

    // Must be called with data of non-zero length
    fn build_tree(data: &[&str]) -> (Node, usize) {
        assert!(!data.is_empty());

        let mut depth = 1;

        // Construct the leaf nodes first
        let mut layer: Vec<Node> = data
            .iter()
            .map(|data| {
                let hash = hash(MessageDigest::sha256(), data.as_bytes())
                    .unwrap()
                    .to_vec();

                Node::new(hash, RefCell::new(None), RefCell::new(None))
            })
            .collect();

        // Iter over data and construct layers from the leaf up to the root
        //
        // Continue to build layers until we have a single root node
        while layer.len() > 1 {
            depth += 1;

            // if we have odd nodes then duplicate the last node
            if layer.len() % 2 != 0 {
                let last = layer.last().unwrap().clone();
                layer.push(last);
            }
            assert_eq!(layer.len() % 2, 0);

            // construct next layer of the merkel tree
            let mut next_layer = vec![];
            for (i, _node) in layer.iter().enumerate().step_by(2) {
                let left = layer[i].clone();
                let right = layer[i + 1].clone();
                let hash = Self::hash_nodes(&left, &right);

                let node = Node::new(
                    hash,
                    RefCell::new(Some(Box::new(left))),
                    RefCell::new(Some(Box::new(right))),
                );

                next_layer.push(node);
            }

            layer = next_layer;
        }

        (layer[0].clone(), depth)
    }

    fn hash_nodes(left: &Node, right: &Node) -> Vec<u8> {
        let mut h = Hasher::new(MessageDigest::sha256()).unwrap();
        h.update(&left.hash).unwrap();
        h.update(&right.hash).unwrap();
        let res = h.finish().unwrap();
        res.to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_tree() {
        let data = ["1", "2", "3", "4"];
        let tree = Tree::new_with_data(&data);
        assert_eq!(tree.depth(), 3);

        let data = ["1", "2", "3"];
        let tree = Tree::new_with_data(&data);
        assert_eq!(tree.depth(), 3);

        let data = ["1", "2"];
        let tree = Tree::new_with_data(&data);
        assert_eq!(tree.depth(), 2);

        let data = ["1"];
        let tree = Tree::new_with_data(&data);
        assert_eq!(tree.depth(), 1);
    }

    #[test]
    fn eq_root() {
        let data = ["1", "2", "3", "4"];

        let tree1 = Tree::new_with_data(&data);
        let tree2 = Tree::new_with_data(&data);

        assert_eq!(tree1.root().hash(), tree2.root().hash());
    }
}
