use crate::List::{Cons, Nil};
use std::ops::Deref;


enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x: i32 = 10;
    // let y:&i32 = &x;

    let y = MyBox::new(x);

    assert_eq!(x, 10);
    assert_eq!(*y, 10);


    let x = MyBox::new(String::from("mim"));
    hello(&x)
}


struct MyBox<T>(T);

impl<T>  MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}


impl <T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T{
        &self.0
    }
}

fn hello(name:&str){
    println!("hello {}",name)
}