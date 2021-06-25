use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx,rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move ||{
        let vals =vec![
            String::from("hi"),
            String::from("hello"),
            String::from("world"),
            String::from("mim"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    thread::spawn(move ||{
        let vals =vec![
            String::from("tx1 hi"),
            String::from("tx1 hello"),
            String::from("tx1 world"),
            String::from("tx1 mim"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });



    // let v =  rx.recv().unwrap();

    for received in rx{
        println!("{}",received)
    }


    // println!("{}",v)
}
