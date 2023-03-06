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

