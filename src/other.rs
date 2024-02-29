use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct LinkedList<T> {
    item: T,
    child: Option<Rc<RefCell<LinkedList<T>>>>,
    parent: Weak<RefCell<LinkedList<T>>>,
}
impl LinkedList<i32> {
    fn new_node(
        item: i32,
        child: Option<&Rc<RefCell<LinkedList<i32>>>>,
        parent: Option<&Rc<RefCell<LinkedList<i32>>>>,
    ) -> Rc<RefCell<Self>> {
        let item = Rc::new(RefCell::new(LinkedList {
            item,
            parent: Weak::new(),
            child: None,
        }));

        if let Some(parent) = parent {
            item.borrow_mut().parent = Rc::downgrade(parent);
            parent.borrow_mut().child = Some(Rc::clone(&item));
        }

        if let Some(child) = child {
            item.borrow_mut().child = Some(Rc::clone(child));
            child.borrow_mut().parent = Rc::downgrade(&item);
        }
        item
    }
}
pub fn my_fn() {
    let parent = LinkedList::new_node(1, None, None);
    let child = LinkedList::new_node(2, None, Some(&parent));
    LinkedList::new_node(3, None, Some(&child));
    let mut item = Some(Rc::clone(&parent));
    while let Some(item_) = item {
        let val = item_.borrow();

        println!("THE ITEM IS {}", val.item);
        if let Some(child_ref) = &val.child {
            item = Some(Rc::clone(child_ref));
        } else {
            item = None;
        }
    }
}
