use std::cell::{Cell, RefCell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,                // Tracks the number of dropped threads
    pub states: RefCell<Vec<bool>>,        // Tracks the state of threads (false = not dropped, true = dropped)
}

impl Workers {
    // Creates a new Workers instance
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    // Creates a new thread and returns its pid and the thread instance
    pub fn new_worker(&self, cmd: String) -> (usize, Thread) {
        let pid = self.track_worker(); // Get the next available index
        self.states.borrow_mut().push(false); // Add a new thread state (not dropped)
        (pid, Thread::new_thread(pid, cmd, self))
    }

    // Returns the next available index for a new thread
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    // Checks if a thread with the given pid is dropped
    pub fn is_dropped(&self, pid: usize) -> bool {
        *self.states.borrow().get(pid).unwrap_or(&false)
    }

    // Marks a thread as dropped and increments the drop count
    pub fn add_drop(&self, pid: usize) {
        let mut states = self.states.borrow_mut();
        if let Some(state) = states.get_mut(pid) {
            if *state {
                panic!("{} is already dropped", pid);
            }
            *state = true;
            self.drops.set(self.drops.get() + 1);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,          // Thread ID
    pub cmd: String,         // Command associated with the thread
    pub parent: &'a Workers, // Reference to the parent Workers instance
}

impl<'a> Thread<'a> {
    // Creates a new thread
    pub fn new_thread(pid: usize, cmd: String, parent: &'a Workers) -> Thread<'a> {
        Thread { pid, cmd, parent }
    }

    // Drops the thread explicitly
    pub fn skill(self) {
        drop(self);
    }
}

// Implements the Drop trait for Thread
impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
}

/*
Instructions

    Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.

In this exercise, you will create a Drop Checker API.

Define the following structures:

    Workers: containing:
        drops: that will save the number of dropped threads.
        states: that will save the state of multiple threads. If the thread is not dropped, the state will be false, and will be true otherwise.
    Thread: containing:
        pid: the id of the thread.
        cmd: the name of the thread.
        parent: a link to the structure Workers. (Tip: this should be a reference).

You'll need to also add the following associated functions to the structures:

    Workers :
        new: that creates a default worker.
        new_worker: that returns a tuple with the pid and a new Thread. This function must receive a String representing the cmd.
        is_dropped: that receives a pid and returns a bool that indicates the state of the thread.
        track_worker: which returns a usize representing the length of the states vector. (The index of the next new thread).
        add_drop: which is called by the Drop trait. It will receive a pid that will be used to change the state of the thread. If the state of that thread is true then it will panic with the message "X is already dropped", where X represents the pid). Otherwise it should change the state to true and increment the drops field by 1.

    Thread:
        new_thread: that initializes a new thread.
        skill: that drops the thread.

    You must implement the Drop trait for the Thread structure. In this trait you must call the function add_drop so that the state of the thread changes.

Expected Functions

use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {}
    pub fn new_worker(&self, c: String) -> (usize, Thread) {}
    pub fn track_worker(&self) -> usize {}
    pub fn is_dropped(&self, id: usize) -> bool {}
    pub fn add_drop(&self, id: usize) {}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    // expected public fields
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {}
    pub fn skill(self) {}
}

Usage

Here is a program to test your function,

use std::rc::Rc;
use drop_the_thread::*;

fn main() {
    let worker = Workers::new();
    let (id, thread) = worker.new_worker(String::from("command"));
    let (id1, thread1) = worker.new_worker(String::from("command1"));

    thread.skill();

    println!("{:?}", (worker.is_dropped(id), id, &worker.drops));

    thread1.skill();
    println!("{:?}", (worker.is_dropped(id1), id1, &worker.drops));

    let (id2, thread2) = worker.new_worker(String::from("command2"));
    let thread2 = Rc::new(thread2);
    let thread2_clone = thread2.clone();

    drop(thread2_clone);

    println!("{:?}", (worker.is_dropped(id2), id2, &worker.drops, Rc::strong_count(&thread2)));
}

And its output:

$ cargo run
(true, 0, Cell { value: 1 })
(true, 1, Cell { value: 2 })
(false, 2, Cell { value: 2 }, 1)
$

Notions

    Trait std::ops::Drop
    Struct std::cell::RefCell
    Interior Mutability


*/