use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let val = Rc::new(RefCell::new(10));

    let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(20)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(40)),Rc::clone(&a));

    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",c);

    *val.borrow_mut() += 100;

    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",c);

}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
