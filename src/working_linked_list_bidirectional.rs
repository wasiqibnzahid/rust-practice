use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

struct LinkedList<T: Display + Copy> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

struct Node<T: Display + Copy> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Display + Copy> LinkedList<T> {
    fn new(data: T) -> Self {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));
        LinkedList {
            head: Some(Rc::clone(&node)),
            tail: Some(Rc::clone(&node)),
        }
    }
    fn push(&mut self, data: T) {
        let node = Node {
            data,
            prev: self.tail.take().map(|x| Rc::downgrade(&x)),
            next: None,
        };
        let node_val: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(node));
        if let Some(prev) = &node_val.borrow().prev {
            let prev_item = prev.upgrade();
            if let Some(prev) = prev_item {
                prev.borrow_mut().next = Some(Rc::clone(&node_val));
            }
        }

        self.tail = Some(Rc::clone(&node_val));
    }

    fn unshift(&mut self, data: T) {
        let node = Node {
            data,
            prev: None,
            next: self.head.take().map(|x| Rc::clone(&x)),
        };
        let node_val = Rc::new(RefCell::new(node));
        if let Some(next_item) = &node_val.borrow().next {
            next_item.borrow_mut().prev = Some(Rc::downgrade(&node_val));
        }

        self.head = Some(Rc::clone(&node_val));
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(last_item) = self.tail.take() {
            let mut second_last_item = last_item.borrow_mut().prev.take();
            if let Some(second_last) = second_last_item.take() {
                if let Some(item) = second_last.upgrade() {
                    self.tail = Some(Rc::clone(&item));
                    item.borrow_mut().next = None;
                }
            } else {
                self.tail = None;
                self.head = None;
            }

            let f = Rc::try_unwrap(last_item).ok();
            if let Some(data) = f {
                return Some(data.into_inner().data);
            }
        };
        None
    }
    fn shift(&mut self) -> Option<T> {
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
