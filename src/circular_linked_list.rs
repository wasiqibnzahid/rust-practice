use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};
type NodeRef<T> = Rc<RefCell<Node<T>>>;
type WeakNodeRef<T> = Weak<RefCell<Node<T>>>;
struct LinkedList<T: Display + Copy> {
    head: Option<NodeRef<T>>,
    tail: Option<NodeRef<T>>,
}
pub struct Node<T: Display + Copy> {
    pub data: T,
    pub next: Option<NodeRef<T>>,
    pub prev: Option<WeakNodeRef<T>>,
}

impl<T: Display + Copy> LinkedList<T> {
    pub fn new(data: T) -> Self {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));
        node.borrow_mut().next = Some(Rc::clone(&node));
        node.borrow_mut().prev = Some(Rc::downgrade(&node));

        LinkedList {
            head: Some(Rc::clone(&node)),
            tail: Some(Rc::clone(&node)),
        }
    }
    pub fn push(&mut self, data: T) {
        let node = Node {
            data,
            prev: self.tail.take().map(|x| Rc::downgrade(&x)),
            next: self.head.take().map(|x| Rc::clone(&x)),
        };
        let node_val: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(node));
        if let None = node_val.borrow().next {
            node_val.borrow_mut().next = Some(Rc::clone(&node_val));
            node_val.borrow_mut().prev = Some(Rc::downgrade(&node_val));
        } else {
            if let Some(prev) = &node_val.borrow().prev {
                let prev_item = prev.upgrade();
                if let Some(prev) = prev_item {
                    prev.borrow_mut().next = Some(Rc::clone(&node_val));
                }
            }
            if let Some(next) = &node_val.borrow_mut().next {
                next.borrow_mut().prev = Some(Rc::downgrade(&node_val));
            }
        }

        self.tail = Some(Rc::clone(&node_val));
    }

    pub fn unshift(&mut self, data: T) {
        let node = Node {
            data,
            prev: self.tail.take().map(|x| Rc::downgrade(&x)),
            next: self.head.take().map(|x| Rc::clone(&x)),
        };

        let node_val = Rc::new(RefCell::new(node));
        if let None = node_val.borrow_mut().prev {
            node_val.borrow_mut().prev = Some(Rc::downgrade(&node_val));
            node_val.borrow_mut().next = Some(Rc::clone(&node_val));
        } else {
            if let Some(next_item) = &node_val.borrow().next {
                next_item.borrow_mut().prev = Some(Rc::downgrade(&node_val));
            }
            if let Some(prev_item) = &node_val.borrow().prev {
                if let Some(prev_item) = prev_item.upgrade() {
                    prev_item.borrow_mut().next = Some(Rc::clone(&node_val));
                }
            }
        }

        self.head = Some(Rc::clone(&node_val));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(last_item) = self.tail.take() {
            let second_last_item = last_item.borrow_mut().prev.take();
            if let Some(second_last_item) = second_last_item {
                if let Some(second_last_item) = second_last_item.upgrade() {
                    self.tail = Some(Rc::clone(&second_last_item));
                    let first_item = &self.head;
                    if let Some(first_item) = first_item {
                        second_last_item.borrow_mut().next = Some(Rc::clone(first_item));
                        first_item.borrow_mut().next = Some(Rc::clone(&second_last_item));
                    }
                }
            } else {
                self.head = None;
                self.tail = None;
            }

            let res = Rc::try_unwrap(last_item).ok();
            if let Some(res) = res {
                return Some(res.into_inner().data);
            }
        }

        None
    }
    pub fn shift(&mut self) -> Option<T> {
        if let Some(first_item) = self.head.take() {
            let second_item = first_item.borrow_mut().next.take();
            if let Some(second_item) = second_item {
                second_item.borrow_mut().prev = None;
                self.head = Some(Rc::clone(&second_item));
            }
            let res = Rc::try_unwrap(first_item).ok();
            if let Some(res) = res {
                return Some(res.into_inner().data);
            }
        }
        None
    }
}
impl<T: Display + Copy> Drop for Node<T> {
    fn drop(&mut self) {
        println!("DROPPING {}", self.data);
    }
}

impl<T: Display + Copy> Drop for LinkedList<T> {
    fn drop(&mut self) {
        println!("DROPPING THE LINKED_LIST");
    }
}
fn a() -> LinkedList<i32> {
    let mut list = LinkedList::new(43);
    list.unshift(510);
    list.push(50);
    list.push(5050);
    list
}

pub fn other_main() {
    let mut list = a();
    println!("First shift, {}", list.shift().unwrap());
    println!("Second shift, {}", list.shift().unwrap());
    println!("First pop, {}", list.pop().unwrap());
}
