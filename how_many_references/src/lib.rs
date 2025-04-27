pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);

    }
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        // Check if the element is in the list and if it points to the same allocation
        if let Some(pos) = self.ref_list.iter().position(|x| Rc::ptr_eq(x, &element)) {
            // Remove all occurrences of the element
            self.ref_list.retain(|x| !Rc::ptr_eq(x, &element));
        }

    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    // Use the strong_count method to get the number of references
    Rc::strong_count(ref_list)
}

/*
Instructions

Create the following functions:

    add_element: which adds an element to the list in the Node.
    how_many_references: which returns how many times the value is referenced in the code.
    rm_all_ref: which accepts an Rc<String> and removes all elements from the vector that are equal to that value. This should only happen if the two Rcs point to the same allocation.

Expected Functions and structures

pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }
    pub fn add_element(&mut self, element: Rc<String>) {}
    pub fn rm_all_ref(&mut self, element: Rc<String>) {}
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {}

Usage

Here is a program to test your functions,

use how_many_references::*;

fn main() {
    let a = Rc::new(String::from("a"));
    let b = Rc::new(String::from("b"));
    let c = Rc::new(String::from("c"));

    let a1 = Rc::new(String::from("a"));

    let mut new_node = Node::new(vec![a.clone()]);
    new_node.add_element(b.clone());
    new_node.add_element(a.clone());
    new_node.add_element(c.clone());
    new_node.add_element(a.clone());

    println!("a: {:?}", how_many_references(&a));
    println!("b: {:?}", how_many_references(&b));
    println!("c: {:?}", how_many_references(&c));
    new_node.rm_all_ref(a1.clone());
    new_node.rm_all_ref(a.clone());

    println!("a: {:?}", how_many_references(&a));
    println!("b: {:?}", how_many_references(&b));
    println!("c: {:?}", how_many_references(&c));
}

And its output:

$ cargo run
a: 4
b: 2
c: 2
a: 1
b: 2
c: 2
$

Notions

    Reference Counted Smart Pointer
    Struct std::rc::Rc

*/