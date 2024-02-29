// use std::rc::Rc;
// mod ab;
mod circular_linked_list;
mod other;
mod working_linked_list_bidirectional;

fn main() {
    // other::my_fn();
    // working_linked_list_bidirectional::other_main();
    circular_linked_list::other_main();
}
// struct MyPointer {
//     data: String,
// }
// impl Drop for MyPointer {
//     fn drop(&mut self) {
//         println!("DROPPING STRING WITH DATA {}", self.data);
//     }
// }
// fn my_drop_fn<T>(_param: T) {}
// fn my_other_fn() {
//     // let f = MyPointer {
//     //     data: String::from("A"),
//     // };
//     //
//     // println!("DATA IS {} ", f.data);
//     // let other = MyPointer {
//     //     data: String::from("B"),
//     // };
//     //
//     // println!("DATA IS {} ", other.data);
//     // std::mem::drop(f); my_drop_fn(other);
//     let item = Rc::new(List::Item(32, Rc::new(List::None)));
//     let list_a = List::Item(25, Rc::clone(&item));
//     let list_b = List::Item(21, Rc::clone(&item));
//     println!("{}", Rc::strong_count(&item));
//     my_drop_fn(list_a);
//     println!("{}", Rc::strong_count(&item));
//
//     my_drop_fn(item);
//     let f = if let List::Item(_a, b) = &list_b {
//         Rc::strong_count(b)
//     } else {
//         0
//     };
//     println!("{}", f);
//     // println!(item.)
// }
//
// enum List {
//     Item(i32, Rc<List>),
//     None,
// }
//
// // enum List {
// //     Cons(i32, Box<List>),
// //     Nil,
// // }
// //
// // use crate::List::{Cons, Nil};
// //
// // fn main() {
// //     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// // }
