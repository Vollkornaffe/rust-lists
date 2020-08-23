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
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl Drop for List {
    fn drop(&mut self) {
        self.head.drop();
    }
}

impl Drop for Link {
    fn drop(&mut self) {
        match *self {
            Link::Empty => {},
            Link::More(mut boxed_node) => {
                boxed_node.drop();
            },
        }
    }
}

impl Drop for Box<Node> {
    fn drop(&mut self) {
        self.ptr.drop(); // <--- not tail recursive
        deallocate(self.ptr); // <--- function doesn't exist? maybe just pseudo code
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.next.drop();
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
