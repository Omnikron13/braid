#![feature(test)]
extern crate test;

use crate::strand::Node;

mod strand;

fn main() {
    let rope = Node::new_leaf("");
    println!("rope: {rope:?}");

    let rope = rope.insert("foo", 0);
    println!("rope: {rope:?}");
}


/*
use std::rc::{Rc, Weak};
use std::{cmp, default};


// TODO: consider an enum!
#[derive(Clone, Debug)]
struct Node<'a> {
    length: usize,
    data: Option<&'a str>,
    left: Option<Rc<Node<'a>>>,
    right: Option<Rc<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new_leaf(data: &'a str) -> Option<Rc<Node<'a>>> {
        Some(Rc::new(Node {
            length: data.chars().count(),
            data: Some(data),
            left: None,
            right: None,
        }))
    }

    fn new_split(a: &'a str, b: &'a str) -> Node<'a> {
        Node {
            length: a.chars().count() + b.chars().count(),
            data: None,
            left: Node::new_leaf(a),
            right: Node::new_leaf(b),
        }
    }

    fn insert(&self, data: &'a str, i: usize) -> Node<'a> {
        match self.data {
            // Node is not a leaf node; recurse
            None => {
                let (left, right) = unsafe {(
                    self.left.as_ref().unwrap_unchecked(),
                    self.right.as_ref().unwrap_unchecked(),
                )};

                let (left, right) = match i < left.length {
                    true => (
                        Rc::new(left.insert(data, i)),
                        right.clone(),
                    ),
                    false => (
                        left.clone(),
                        Rc::new(right.insert(data, i - left.length)),
                    ),
                };

                Node {
                    length: self.length + data.chars().count(),
                    data: None,
                    left: Some(left),
                    right: Some(right),
                }
            },

            // Node is a leaf node; split and insert
            Some(s) => {
                // TODO: consider working with Rc<Node> so it can be cloned for better CoW
                if i == 0 {
                    Node::new_split(data, s)
                }
                else if i == self.length {
                    Node::new_split(s, data)
                }
                else {
                    Node {
                        length: self.length + data.chars().count(),
                        data: None,
                        left: Node::new_leaf(unsafe{s.get_unchecked(0..i)}),
                        right: Some(Rc::new(Node::new_split(data, unsafe{s.get_unchecked(i..s.len())}))),
                    }
                }
            },
        }
    }
}


fn main() {
    let n = Node {
        length: 6,
        data: None,
        left: Node::new_leaf("foo"),
        right: Node::new_leaf("bar"),
    };
    //println!("{n:#?}");

    //let q = n.insert("--", 0);
    //println!("{q:#?}");

    for i in [0,1,3,4,6,] {
        let q = n.insert("-", i);
        println!("{q:#?}");
    }
}
*/

