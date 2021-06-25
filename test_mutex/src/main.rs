use std::sync::{Mutex,Arc};
use std::thread;
use std::rc::Rc;
use std::thread::current;


fn main() {

    // let mut m = Mutex::new(6);
    //
    // {
    //     let mut num = m.lock().unwrap();
    //
    //
    //     println!("{}",num);
    //     *num = 10;
    //
    // }
    //
    // println!("{:?}",m)


    let  counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10{
        let counter_tmp = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter_tmp.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }


    for h in handles{
        h.join().unwrap();
    }

    println!("{:?}",counter)

}

