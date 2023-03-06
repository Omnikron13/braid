#![feature(test)]
extern crate test;

use crate::strand::Strand;

mod strand;

fn main() {
    let rope = Strand::new_leaf("");
    println!("rope: {rope:?}");

    let rope = rope.insert("foo", 0);
    println!("rope: {rope:?}");
}

