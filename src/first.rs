use std::mem;
use std::fmt;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
        }
    }

    pub fn pop_node(&mut self) -> Link {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => Link::Empty,
            Link::More(mut node) => {
                self.head = mem::replace(&mut node.next, Link::Empty);
                Link::More(node)
            },
        }
    }
}

#[derive(Debug)]
pub enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    elem: i32,
    next: Link,
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg_struct = f.debug_struct("List");
        let mut link = &self.head;
        loop { 
            match &link {
                Link::Empty => {
                    break;
                },
                Link::More(node) => {
                    dbg_struct.field("elem", &node.elem);
                    link = &node.next;
                },
            }
        }
        dbg_struct.finish()
    }
}
