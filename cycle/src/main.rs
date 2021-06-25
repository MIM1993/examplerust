use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    // println!("{}", Rc::strong_count(&a));
    // println!("{:?}", a.tail());
    //
    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    //
    // println!("{}", Rc::strong_count(&b));
    // println!("{:?}", b.tail());
    //
    // if let Some(item) = a.tail() {
    //     *item.borrow_mut() = Rc::clone(&b);
    // };
    //
    // println!("{}", Rc::strong_count(&a));
    // println!("{}", Rc::strong_count(&b));

    // let leaf =Rc::new(Node{
    //     value:10,
    //     children : RefCell::new(vec![]),
    //     parent : RefCell::new(Weak::new())
    // });
    //
    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    //
    // let banch = Rc::new(Node{
    //     value:100,
    //     children : RefCell::new(vec![Rc::clone(&leaf)]),
    //     parent : RefCell::new(Weak::new()),
    // });
    //
    // //修改leaf中的父节点信息
    // *leaf.parent.borrow_mut() = Rc::downgrade(&banch);
    //
    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), //1
        Rc::weak_count(&leaf),  //0
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch), //1
            Rc::weak_count(&branch), //1
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), //2
            Rc::weak_count(&leaf), //0
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),//1
        Rc::weak_count(&leaf),//0
    );
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}
