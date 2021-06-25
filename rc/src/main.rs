use crate::List::{Cons, Nil};
use std::mem::drop;
use std::rc::Rc;
use std::fmt::Pointer;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&a));
    println!("{}", Rc::strong_count(&a));

    let c = Rc::new(Cons(3, Rc::clone(&a)));
    let b = Rc::new(Cons(4, Rc::clone(&a)));
    println!("{}", Rc::strong_count(&a));
    drop(a);



    println!("{:?}", b);
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
