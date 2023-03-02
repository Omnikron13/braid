use std::rc::Rc;

#[derive(Clone, Debug)]
enum Node<'a> {
    Branch(Rc<BranchNode<'a>>),
    Leaf(Rc<LeafNode<'a>>),
}

#[derive(Debug)]
struct BranchNode<'a> {
    length: usize,
    left: Node<'a>,
    right: Node<'a>,
}

#[derive(Debug)]
struct LeafNode<'a> {
    length: usize,
    value: &'a str,
}

impl<'a> Node<'a> {
    fn new(s: &'a str) -> Node<'a> {
        Node::Leaf(Rc::new(LeafNode{length: s.chars().count(), value: s}))
    }

    fn length(&self) -> usize {
        match self {
            Node::Branch(b) => b.length,
            Node::Leaf(l) => l.length,
        }
    }

    fn insert(&self, s: &'a str, i: usize) -> Node<'a> {
        match self {
            Node::Branch(b) => {
                if i < b.left.length() {
                    Node::Branch(Rc::new(BranchNode{
                        length: b.length + s.chars().count(),
                        left: b.left.insert(s, i),
                        right: b.right.clone(),
                    }))
                }
                else {
                    Node::Branch(Rc::new(BranchNode{
                        length: b.length + s.chars().count(),
                        left: b.left.clone(),
                        right: b.right.insert(s, i - b.left.length())
                    }))
                }
            },
            Node::Leaf(l) => {
                let length = l.length + s.chars().count();
                let (left, right) = if i == 0 {(
                    Node::new(s),
                    Node::Leaf(l.clone()),
                )} else if i == l.length {(
                    Node::Leaf(l.clone()),
                    Node::new(s),
                )} else {(
                        Node::new(unsafe {l.value.get_unchecked(0..i)}),
                        Node::Branch(Rc::new(BranchNode {
                            length: length - i,
                            left: Node::new(s),
                            right: Node::new(unsafe {l.value.get_unchecked(i..l.length)})
                        })),
                )};
                Node::Branch(Rc::new(BranchNode{length, left, right}))
            },
        }
    }
}


fn main() {
    let n = Node::new("foo");
    //println!("{n:?}");

    match n.clone() {
        Node::Leaf(l) => {
            println!("leaf: {l:?}");
            println!("leaf ptr: {l:p}");
        },
        Node::Branch(b) => {
            println!("branch: {b:?}");
        },
    }

    //let p = n.clone();
    let q = n.insert("bar", 0);
    println!("{:#?}", q);
    let p = q.insert("wop", 4);
    println!("{:#?}", p);
    match p.clone() {
        Node::Leaf(l) => {
            println!("p: {l:?}");
            println!("p ptr: {l:p}");
        },
        Node::Branch(b) => {
            println!("branch: {b:?}");
            println!("b ptr: {:p}", match b.right.clone() {
                Node::Leaf(l) => l,
                Node::Branch(b) => Rc::new(LeafNode { length: 0, value: "" }),
            });
        },
    }

    println!("n: {n:?}");
    println!("q: {q:?}");
    println!("p: {p:?}");
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

