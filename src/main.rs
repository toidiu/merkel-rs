#![allow(dead_code)]
#![allow(unused)]

use boring::hash::hash;
use boring::hash::DigestBytes;
use boring::hash::Hasher;
use boring::hash::MessageDigest;
use std::cell::RefCell;
use std::fmt::Debug;
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

struct Tree {
    root: Node,
    elem_cnt: usize,
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
            elem_cnt: data_len,
            depth,
        }
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    // The position of the original data
    pub fn contains(&self, data: &str, pos: usize) -> Result<bool, ()> {
        if pos + 1 > self.elem_cnt {
            return Err(());
        }

        // data should be one of the leaf nodes. calculate the hash to compare
        // let data_hash = hash(MessageDigest::sha256(), data.as_bytes()).unwrap();
        let data_hash = Self::hash_data(data.as_bytes());
        let data_hash = data_hash.as_ref();

        let mut cmp_node = Box::new(self.root().clone());

        // make assertions about a single node tree
        if self.elem_cnt == 1 {
            let left_none = cmp_node.left.borrow().is_none();
            let right_none = cmp_node.right.borrow().is_none();
            assert!(left_none && right_none);

            if &cmp_node.hash == data_hash {
                return Ok(true);
            }
        }

        let mut left = 0;
        // since we build the merkel tree with even nodes (duplicating the last element), make
        // the search space even to make calculating easier
        let mut right = if self.elem_cnt % 2 == 0 {
            self.elem_cnt - 1
        } else {
            self.elem_cnt
        };

        // based on the position we need to traverse the tree down to the leaf node.
        // continue until we are at the leaf nodes
        while cmp_node.left.borrow().is_some() || cmp_node.right.borrow().is_some() {
            // the tree has even balanced nodes
            assert!(cmp_node.left.borrow().is_some() && cmp_node.right.borrow().is_some());

            let search_space = right - left;
            if search_space == 0 {
                unreachable!(
                    "leaf nodes have None branches and we should not have entered this loop"
                );
            }

            let mid_idx = left + (search_space / 2);
            println!("----------- pos:{pos}  {left} -{mid_idx}- {right}");

            if pos <= mid_idx {
                println!("<-- left");
                // search left
                cmp_node = cmp_node.left.clone().borrow().clone().unwrap();
                right = mid_idx;
            } else {
                println!("--> right");
                // search right
                cmp_node = cmp_node.right.clone().borrow().clone().unwrap();
                left = mid_idx + 1;
            }
        }

        println!("----------- pos:{pos}  {left}-{right}");
        if self.elem_cnt > 1 {
            assert_eq!(left, right);
        }

        if &cmp_node.hash == data_hash {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // Must be called with data of non-zero length
    fn build_tree(data: &[&str]) -> (Node, usize) {
        assert!(!data.is_empty());

        let mut depth = 1;

        // Construct the leaf nodes first
        let mut layer: Vec<Node> = data
            .iter()
            .map(|data| {
                let hash = Self::hash_data(data.as_bytes()).to_vec();

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

    fn hash_data(data: &[u8]) -> DigestBytes {
        hash(MessageDigest::sha256(), data).unwrap()
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

    #[test]
    fn contains_in_even_tree() {
        let data = ["1", "2", "3", "4"];
        let tree = Tree::new_with_data(&data);

        let check_data = "1";
        let hash = Tree::hash_data(check_data.as_bytes());
        assert!(tree.contains(check_data, 0).unwrap());

        let check_data = "2";
        let hash = Tree::hash_data(check_data.as_bytes());
        assert!(tree.contains(check_data, 1).unwrap());

        let check_data = "3";
        let hash = Tree::hash_data(check_data.as_bytes());
        assert!(tree.contains(check_data, 2).unwrap());

        let check_data = "4";
        let hash = Tree::hash_data(check_data.as_bytes());
        assert!(tree.contains(check_data, 3).unwrap());

        let check_data = "5";
        let hash = Tree::hash_data(check_data.as_bytes());
        assert!(!tree.contains(check_data, 0).unwrap());

        // pos is out of bounds
        let check_data = "5";
        assert!(tree.contains(check_data, 5).is_err());
    }

    #[test]
    fn contains_in_odd_tree() {
        let data = ["1", "2", "3"];
        let tree = Tree::new_with_data(&data);

        let check_data = "1";
        let hash = Tree::hash_data(check_data.as_bytes());
        assert!(tree.contains(check_data, 0).unwrap());

        let check_data = "2";
        let hash = Tree::hash_data(check_data.as_bytes());
        assert!(tree.contains(check_data, 1).unwrap());

        let check_data = "3";
        let hash = Tree::hash_data(check_data.as_bytes());
        assert!(tree.contains(check_data, 2).unwrap());

        let check_data = "4";
        let hash = Tree::hash_data(check_data.as_bytes());
        assert!(!tree.contains(check_data, 0).unwrap());
    }

    #[test]
    fn contains_in_single_node_tree() {
        let data = ["1"];
        let tree = Tree::new_with_data(&data);

        let check_data = "1";
        let hash = Tree::hash_data(check_data.as_bytes());
        assert!(tree.contains(check_data, 0).unwrap());

        let check_data = "2";
        let hash = Tree::hash_data(check_data.as_bytes());
        assert!(!tree.contains(check_data, 0).unwrap());
    }
}
