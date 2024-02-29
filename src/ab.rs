use std::{
    fmt::Display,
    rc::{Rc, Weak},
};

struct LinkedList<T: Display + Copy> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
}

struct Node<T: Display + Copy> {
    data: T,
    next: Option<Rc<Node<T>>>,
    prev: Option<Weak<Node<T>>>,
}

impl<T: Display + Copy> LinkedList<T> {
    fn new(data: T) -> Self {
        let node = Rc::new(Node {
            data,
            next: None,
            prev: None,
        });
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
        let node_val = Rc::new(node);
        if let Some(prev) = &node_val.prev {
            let prev_item = prev.upgrade();
            if let Some(mut prev) = prev_item {
                prev.next = Some(Rc::clone(&node_val));
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
        let node_val = Rc::new(node);
        if let Some(next_item) = &node_val.next {
            next_item.prev = Some(Rc::downgrade(&node_val));
        }

        self.head = Some(Rc::clone(&node_val));
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(last_item) = self.tail.take() {
            let mut second_last_item = last_item.prev.take();
            if let Some(second_last) = second_last_item.take() {
                if let Some(item) = second_last.upgrade() {
                    self.tail = Some(Rc::clone(&item));
                    item.next = None;
                }
            } else {
                self.tail = None;
                self.head = None;
            }

            let f = Rc::try_unwrap(last_item).ok();
            if let Some(data) = f {
                return Some(data.data);
            }
        };
        None
    }
    fn shift(&mut self) -> Option<T> {
        if let Some(mut first_item) = self.head.take() {
            let second_item = first_item.next.take();
            if let Some(mut second_item) = second_item {
                second_item.prev = None;
                self.head = Some(Rc::clone(&second_item));
            }
            let res = Rc::try_unwrap(first_item).ok();
            if let Some(res) = res {
                return Some(res.data);
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
